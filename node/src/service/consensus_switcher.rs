use babe::{BabeBlockImport, BabeConfiguration, BabeLink, BabeVerifier};
use creditcoin_node_runtime::{self, self as runtime, opaque::Block};
use jsonrpsee::core::async_trait;

use primitives::metrics::MiningMetrics;
use sc_consensus::{BlockCheckParams, BlockImportParams, LongestChain, Verifier};
use sc_consensus_babe as babe;
use sc_consensus_grandpa::{LinkHalf, SharedVoterState};
use sc_consensus_pow::PowVerifier;

use sc_keystore::LocalKeystore;
use sc_network::ProtocolName;
use sc_service::TaskManager;
use sc_telemetry::TelemetryHandle;

use sha3pow::Sha3Algorithm;

use sp_core::traits::SpawnNamed;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::SyncCryptoStorePtr;
use std::{sync::Arc, time::Duration};
use tokio::sync::{oneshot, Notify};

use super::{
	BlockHash, BlockTy, FullBackend, FullClient, FullNetworkService, FullPool, FullSelectChain,
};

pub(crate) struct LazyInit<T: Init> {
	inner: Option<T>,

	deps: Option<T::Deps>,
}

#[async_trait]
pub(crate) trait Init {
	type Deps;
	async fn init(deps: Self::Deps) -> Self;
}

impl<T: Init> LazyInit<T> {
	pub(crate) fn new(deps: T::Deps) -> Self {
		Self { inner: None, deps: Some(deps) }
	}

	pub(crate) async fn get_mut(&mut self) -> &mut T {
		if self.inner.is_none() {
			log::debug!("Initializing LazyInit <{}>", std::any::type_name::<T>());
			let deps = self.deps.take().expect("LazyInit is initialized only once; qed");
			self.inner = Some(T::init(deps).await);
		}

		self.inner.as_mut().expect("LazyInit is initialized only once; qed")
	}
}

pub(crate) type OneshotSender<T> = tokio::sync::oneshot::Sender<T>;

pub(crate) type MpscSender<T> = tokio::sync::mpsc::Sender<T>;

pub(crate) struct GrandpaAuthorityProvider {
	#[cfg(feature = "fast-runtime")]
	client: Arc<FullClient>,
	initial_authorities: Vec<sp_consensus_grandpa::AuthorityId>,
}
impl GrandpaAuthorityProvider {
	#[cfg(not(feature = "fast-runtime"))]
	pub(crate) fn new(initial_authorities: Vec<sp_consensus_grandpa::AuthorityId>) -> Self {
		Self { initial_authorities }
	}
	#[cfg(feature = "fast-runtime")]
	pub(crate) fn with_client(
		client: Arc<FullClient>,
		initial_authorities: Vec<sp_consensus_grandpa::AuthorityId>,
	) -> Self {
		Self { client, initial_authorities }
	}
}

impl sc_consensus_grandpa::GenesisAuthoritySetProvider<BlockTy> for GrandpaAuthorityProvider {
	fn get(&self) -> Result<runtime::GrandpaAuthorityList, sp_blockchain::Error> {
		#[cfg(feature = "fast-runtime")]
		{
			use sc_client_api::UsageProvider;
			use sp_api::ProvideRuntimeApi;
			use sp_consensus_grandpa::GrandpaApi;
			let at_hash = if self.client.usage_info().chain.finalized_state.is_some() {
				self.client.usage_info().chain.best_hash
			} else {
				log::debug!("No finalized state is available. Reading config from genesis");
				self.client.usage_info().chain.genesis_hash
			};
			if let Ok(authorities) = self.client.runtime_api().grandpa_authorities(at_hash) {
				log::debug!("successfully read grandpa authorities from runtime");
				return Ok(authorities);
			}
		}
		if self.initial_authorities.is_empty() {
			log::warn!("No initial grandpa authorities provided. Make sure this is configured correctly in the chain spec. Using a dummy authority for now.");
			return Ok(vec![(
				crate::chain_spec::get_from_seed::<sp_consensus_grandpa::AuthorityId>("Alice"),
				1,
			)]);
		}
		Ok(self.initial_authorities.iter().cloned().map(|auth| (auth, 1)).collect())
	}
}

pub(super) fn babe_import_initializer(
	client: Arc<FullClient>,
	grandpa_block_import: GrandpaImport,
	switch_notif: Arc<Notify>,
	spawner: impl SpawnNamed,
) -> BabeImportInitializer {
	let (sender, receiver) = tokio::sync::mpsc::channel::<BabeImportInitReq>(10);
	spawner.spawn_blocking(
		"babe-initializer",
		Some("pos-switcher"),
		Box::pin(async move {
			let mut receiver = receiver;
			let mut babe_import: Option<BabeImport> = None;
			let mut babe_link: Option<BabeLink<Block>> = None;
			while let Some(req) = receiver.recv().await {
				match &babe_import {
					Some(_) => {},
					None => {
						log::debug!("Initializing BabeImport");
						let babe_config = babe::configuration(&*client).unwrap();
						let babe_config = if babe_config.authorities.is_empty() {
							BabeConfiguration {
								authorities: vec![(
									crate::chain_spec::get_from_seed::<
										sp_consensus_babe::AuthorityId,
									>("Alice"),
									1,
								)],
								..babe_config
							}
						} else {
							babe_config
						};
						log::debug!("with babe config: {babe_config:?}");
						let (block_import, link) = babe::block_import(
							babe_config.clone(),
							grandpa_block_import.clone(),
							client.clone(),
						)
						.unwrap();
						babe_link = Some(link);
						babe_import = Some(block_import);
						switch_notif.notify_one();
					},
				}
				match req {
					BabeImportInitReq::Import(sender) => {
						let _ = sender.send(babe_import.clone().unwrap());
					},
					BabeImportInitReq::Link(sender) => {
						let _ = sender.send(babe_link.clone().unwrap());
					},
				}
			}
		}),
	);

	BabeImportInitializer::new(sender)
}

pub(crate) enum ImportInitReq<I, L> {
	Import(OneshotSender<I>),
	Link(OneshotSender<L>),
}

pub(crate) type GrandpaLink = LinkHalf<Block, FullClient, ChainSelection>;

pub(crate) type BabeImportInitReq = ImportInitReq<BabeImport, BabeLink<Block>>;

pub(crate) type BabeImportInitializer = ImportInitializerService<BabeImport, BabeLink<Block>>;

pub(crate) type GrandpaImportInitializer = ImportInitializerService<GrandpaImport, GrandpaLink>;

pub(crate) struct ImportInitializerService<I, L> {
	inner: MpscSender<ImportInitReq<I, L>>,
}

impl<I, L> Clone for ImportInitializerService<I, L> {
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<I, L> ImportInitializerService<I, L> {
	pub(crate) fn new(inner: MpscSender<ImportInitReq<I, L>>) -> Self {
		Self { inner }
	}

	pub(crate) async fn request_import(&mut self) -> I {
		let (tx, rx) = oneshot::channel();
		self.inner.send(ImportInitReq::Import(tx)).await.map_err(|_| ()).unwrap();
		rx.await.unwrap()
	}

	pub(crate) async fn request_link(&mut self) -> L {
		let (tx, rx) = oneshot::channel();
		self.inner.send(ImportInitReq::Link(tx)).await.map_err(|_| ()).unwrap();
		rx.await.unwrap()
	}
}

#[async_trait]
impl Init for BabeImport {
	type Deps = BabeImportInitializer;

	async fn init(mut babe_init: Self::Deps) -> Self {
		babe_init.request_import().await
	}
}

pub(crate) type GrandpaImport =
	sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

pub(crate) type BabeImport = BabeBlockImport<Block, FullClient, GrandpaImport>;

pub(crate) type PowImport<CIDP> = sc_consensus_pow::PowBlockImport<
	Block,
	GrandpaImport,
	FullClient,
	FullSelectChain,
	Sha3Algorithm<FullClient>,
	CIDP,
>;

pub(crate) type SyncService = Arc<sc_network_sync::SyncingService<Block>>;

pub(crate) struct AuthorshipSwitcher<'a, CIDP> {
	pub(crate) task_manager: &'a TaskManager,
	pub(crate) switch_notif: Arc<tokio::sync::Notify>,
	pub(crate) is_authority: bool,
	pub(crate) pow_params: PowAuthorshipParams<CIDP>,
	pub(crate) tokio_handle: tokio::runtime::Handle,
	pub(crate) babe_params: BabeAuthorshipParams,
}

impl<'a, CIDP> AuthorshipSwitcher<'a, CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
{
	pub(crate) fn run(self) {
		let Self {
			task_manager,
			switch_notif,
			pow_params,
			tokio_handle,
			is_authority,
			babe_params,
		} = self;

		task_manager.spawn_handle().spawn(
			"authorship-switcher",
			"pos-switcher",
			Box::pin(async move {
				if !is_authority {
					switch_notif.notified().await;
					let babe_task_manager = TaskManager::new(tokio_handle.clone(), None).unwrap();
					start_babe_authorship(&babe_task_manager, babe_params, is_authority).await;
					std::future::pending::<()>().await;
					return;
				}

				let client = pow_params.client.clone();
				if switched_to_pos(&client, client.chain_info().best_hash) {
					log::debug!("Already switched to PoS");
					let babe_task_manager = TaskManager::new(tokio_handle.clone(), None).unwrap();
					start_babe_authorship(&babe_task_manager, babe_params, is_authority).await;
					std::future::pending::<()>().await;
					return;
				}

				let pow_task_manager = TaskManager::new(tokio_handle.clone(), None).unwrap();
				let stopper = start_pow_authorship(&pow_task_manager, pow_params);

				log::debug!("Waiting to switch");
				loop {
					let sleep = tokio::time::sleep(std::time::Duration::from_secs(6));
					tokio::select! {
						_ = switch_notif.notified() => {
							break;
						}
						_ = sleep => {
							if switched_to_pos(&client, client.chain_info().best_hash) {
								break;
							}
						}
					}
				}

				log::debug!("Stopping PoW");

				stopper.stop();
				drop(pow_task_manager);
				let babe_task_manager = TaskManager::new(tokio_handle.clone(), None).unwrap();

				log::debug!("Starting Babe");
				start_babe_authorship(&babe_task_manager, babe_params, is_authority).await;
				std::future::pending::<()>().await;
			}),
		);
	}
}

pub(crate) struct BabeAuthorshipParams {
	pub(crate) client: Arc<FullClient>,
	pub(crate) backoff_authoring_blocks: Option<()>,
	pub(crate) force_authoring: bool,
	pub(crate) babe_link: BabeImportInitializer,
	pub(crate) transaction_pool: Arc<FullPool>,
	pub(crate) network: FullNetworkService,
	pub(crate) keystore: Option<SyncCryptoStorePtr>,
	pub(crate) grandpa_link: GrandpaLink,
	pub(crate) grandpa_protocol_name: ProtocolName,
	pub(crate) select_chain: ChainSelection,
	pub(crate) role: sc_network::config::Role,
	pub(crate) name: String,
	pub(crate) dev_key_seed: Option<String>,
	pub(crate) sync_service: SyncService,
	pub(crate) enable_grandpa: bool,
	pub(crate) prometheus_registry: Option<substrate_prometheus_endpoint::Registry>,
	pub(crate) shared_voter_state: SharedVoterState,
}

async fn start_babe_authorship(
	task_manager: &TaskManager,
	params: BabeAuthorshipParams,
	is_authority: bool,
) {
	let BabeAuthorshipParams {
		client,
		backoff_authoring_blocks,
		force_authoring,
		mut babe_link,
		transaction_pool,
		network,
		keystore,
		grandpa_link,
		grandpa_protocol_name,
		select_chain,
		role,
		name,
		dev_key_seed,
		sync_service,
		enable_grandpa,
		prometheus_registry,
		shared_voter_state,
	} = params;

	if is_authority {
		if let Some(seed) = dev_key_seed {
			sp_session::generate_initial_session_keys(
				client.clone(),
				client.chain_info().best_hash,
				vec![seed],
			)
			.unwrap();
		}

		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			None,
			None,
		);

		let block_import = babe_link.request_import().await;
		log::debug!("got babe block import");
		let babe_link = babe_link.request_link().await;
		log::debug!("got babe link");
		let slot_duration = babe_link.config().slot_duration();
		let babe_config = babe::BabeParams {
			keystore: keystore.clone().expect("Keystore must be present for authority node"),
			client: client.clone(),
			select_chain,
			block_import,
			env: proposer,
			sync_oracle: sync_service.clone(),
			justification_sync_link: sync_service.clone(),
			create_inherent_data_providers: move |_parent, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
				sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
					*timestamp,
					slot_duration,
				);

				Ok((slot, timestamp))
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			block_proposal_slot_portion: babe::SlotProportion::new(2f32 / 3f32),
			max_block_proposal_slot_portion: None,
			telemetry: None,
		};

		let babe = babe::start_babe(babe_config).unwrap();
		task_manager.spawn_essential_handle().spawn_blocking("babe", None, babe);
	}

	if enable_grandpa {
		let grandpa_config = sc_consensus_grandpa::Config {
			// FIXME #1578 make this available through chainspec
			gossip_duration: Duration::from_millis(833),
			justification_period: 512,
			name: Some(name),
			observer_enabled: false,
			keystore,
			local_role: role,
			telemetry: None,
			protocol_name: grandpa_protocol_name,
		};

		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_consensus_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network,
			voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
			telemetry: None,
			sync: sync_service,
		};
		log::debug!("starting grandpa voter");
		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_consensus_grandpa::run_grandpa_voter(grandpa_config).unwrap(),
		);
	}
}

pub(crate) struct PowAuthorshipParams<CIDP> {
	pub(crate) client: Arc<FullClient>,
	pub(crate) select_chain: FullSelectChain,
	pub(crate) block_import: PowImport<CIDP>,
	pub(crate) transaction_pool: Arc<FullPool>,
	pub(crate) mining_metrics: MiningMetrics,
	pub(crate) pre_runtime: Option<Vec<u8>>,
	pub(crate) keystore: Option<Arc<LocalKeystore>>,
	pub(crate) threads: Option<usize>,
	pub(crate) sync_service: SyncService,
}

struct PowStopper {
	stop: Arc<std::sync::atomic::AtomicBool>,
}

impl PowStopper {
	fn new(stop: Arc<std::sync::atomic::AtomicBool>) -> Self {
		Self { stop }
	}

	fn stop(&self) {
		self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
	}
}

fn start_pow_authorship<CIDP>(
	task_manager: &TaskManager,
	params: PowAuthorshipParams<CIDP>,
) -> PowStopper
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
{
	let PowAuthorshipParams {
		client,
		select_chain,
		block_import,
		transaction_pool,
		mining_metrics,
		pre_runtime,
		keystore,
		threads,
		sync_service,
	} = params;
	let proposer_factory = sc_basic_authorship::ProposerFactory::new(
		task_manager.spawn_handle(),
		client.clone(),
		transaction_pool,
		None,
		None,
	);

	let algorithm = Sha3Algorithm::new(client.clone());

	let (worker, worker_task) = sc_consensus_pow::start_mining_worker(
		Box::new(block_import),
		client.clone(),
		select_chain,
		algorithm,
		proposer_factory,
		sync_service.clone(),
		sync_service,
		pre_runtime,
		move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
			Ok(timestamp)
		},
		Duration::from_secs(10),
		Duration::from_secs(10),
	);

	task_manager
		.spawn_essential_handle()
		.spawn_blocking("pow", "pow_group", worker_task);

	let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

	let threads = threads.unwrap_or_else(num_cpus::get);
	for _ in 0..threads {
		if let Some(keystore) = keystore.clone() {
			let worker = worker.clone();
			let client = client.clone();
			let mining_metrics = mining_metrics.clone();
			let stop = stop.clone();
			std::thread::spawn(move || {
				let mut count = 0;
				loop {
					let metadata = worker.metadata();
					let version = worker.version();
					if stop.load(std::sync::atomic::Ordering::Relaxed) {
						log::debug!("Exiting mining thread");
						break;
					}
					if let Some(metadata) = metadata {
						loop {
							match sha3pow::mine(
								client.as_ref(),
								&keystore,
								&metadata.pre_hash,
								metadata.pre_runtime.as_ref().map(|v| &v[..]),
								metadata.difficulty,
							) {
								Ok(Some(seal)) => {
									if version == worker.version() {
										let _ = futures_lite::future::block_on(worker.submit(seal));
									}
								},
								Ok(None) => {
									count += 1;
								},
								Err(e) => eprintln!("Mining error: {e}"),
							}
							if count >= 1_000_000 {
								mining_metrics.add(count);
								count = 0;
								if stop.load(std::sync::atomic::Ordering::Relaxed) {
									break;
								}
							}
							if version != worker.version() {
								break;
							}
						}
					}
				}
			});
		}
	}

	PowStopper::new(stop)
}

pub(crate) struct BlockImportSwitcher<CIDP> {
	client: Arc<FullClient>,
	babe_import: LazyInit<BabeImport>,
	pow_import: PowImport<CIDP>,
}

pub(crate) fn switched_to_pos(client: &Arc<FullClient>, at_hash: BlockHash) -> bool {
	pos_switch_block(client, at_hash).is_some()
}

fn pos_switch_block(client: &Arc<FullClient>, at_hash: BlockHash) -> Option<runtime::BlockNumber> {
	use parity_scale_codec::Decode;
	use sc_client_api::backend::StateBackend;

	let key = runtime::pallet_pos_switch::SwitchBlockNumber::<runtime::Runtime>::hashed_key();

	let state_client = match client.state_at(at_hash) {
		Ok(s) => s,
		Err(e) => {
			log::warn!("Failed to get state client at {:?}: {e:?}", at_hash);
			return None;
		},
	};

	let state = state_client.storage(&key).unwrap();

	state.map(|v| Decode::decode(&mut v.as_slice()).unwrap())
}

impl<CIDP> BlockImportSwitcher<CIDP> {
	pub(crate) fn new(
		client: Arc<FullClient>,
		babe_import: LazyInit<BabeImport>,
		pow_import: PowImport<CIDP>,
	) -> Self {
		Self { client, babe_import, pow_import }
	}

	pub(crate) fn switched_to_pos(&self, at_hash: BlockHash) -> bool {
		switched_to_pos(&self.client, at_hash)
	}
}

#[async_trait]
impl<CIDP> sc_consensus::BlockImport<Block> for BlockImportSwitcher<CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()>,
{
	type Error = sp_consensus::Error;

	type Transaction = sp_api::TransactionFor<FullClient, Block>;

	/// Check block preconditions.
	async fn check_block(
		&mut self,
		block: BlockCheckParams<Block>,
	) -> Result<sc_consensus::ImportResult, Self::Error> {
		log::debug!("check_block: #{:?}", block.number);
		if self.switched_to_pos(block.parent_hash) {
			self.babe_import.get_mut().await.check_block(block).await
		} else {
			self.pow_import.check_block(block).await
		}
	}

	/// Import a block.
	async fn import_block(
		&mut self,
		block: BlockImportParams<Block, Self::Transaction>,
	) -> Result<sc_consensus::ImportResult, Self::Error> {
		log::debug!("import_block: #{:?}", block.header.number);
		if self.switched_to_pos(block.header.parent_hash) {
			self.babe_import.get_mut().await.import_block(block).await
		} else {
			log::debug!("import_block: pow");
			self.pow_import.import_block(block).await
		}
	}
}

#[async_trait]
impl Init for GrandpaImport {
	type Deps = GrandpaImportInitializer;

	async fn init(mut deps: Self::Deps) -> Self {
		deps.request_import().await
	}
}

pub(crate) type ChainSelection = LongestChain<FullBackend, Block>;

#[async_trait]
impl<CIDP> Init for BabeVerifier<Block, FullClient, ChainSelection, CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
	CIDP::InherentDataProviders: sc_consensus_slots::InherentDataProviderExt + Send + Sync,
{
	type Deps =
		(Arc<FullClient>, BabeImportInitializer, ChainSelection, CIDP, Option<TelemetryHandle>);

	async fn init((client, mut babe_init, select_chain, cidp, telemetry): Self::Deps) -> Self {
		BabeVerifier::new(babe_init.request_link().await, client, select_chain, cidp, telemetry)
	}
}

pub(crate) struct VerifierSwitcher<CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
	CIDP::InherentDataProviders: sc_consensus_slots::InherentDataProviderExt + Send + Sync,
{
	client: Arc<FullClient>,
	pow_verifier: PowVerifier<Block, Sha3Algorithm<FullClient>>,
	babe_verifier: LazyInit<BabeVerifier<Block, FullClient, ChainSelection, CIDP>>,
}

impl<CIDP> VerifierSwitcher<CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
	CIDP::InherentDataProviders: sc_consensus_slots::InherentDataProviderExt + Send + Sync,
{
	pub(crate) fn new(
		client: Arc<FullClient>,
		pow_verifier: PowVerifier<Block, Sha3Algorithm<FullClient>>,
		babe_verifier: LazyInit<BabeVerifier<Block, FullClient, ChainSelection, CIDP>>,
	) -> Self {
		Self { client, pow_verifier, babe_verifier }
	}

	pub(crate) fn switched_to_pos(&self, at_hash: BlockHash) -> bool {
		switched_to_pos(&self.client, at_hash)
	}
}

#[async_trait]
impl<CIDP> Verifier<Block> for VerifierSwitcher<CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
	CIDP::InherentDataProviders: sc_consensus_slots::InherentDataProviderExt + Send + Sync,
{
	async fn verify(
		&mut self,
		block: BlockImportParams<Block, ()>,
	) -> Result<BlockImportParams<Block, ()>, String> {
		log::debug!("verify: #{:?}", block.header.number);
		if self.switched_to_pos(block.header.parent_hash) {
			self.babe_verifier.get_mut().await.verify(block).await
		} else {
			self.pow_verifier.verify(block).await
		}
	}
}
