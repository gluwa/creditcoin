//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

mod nonce_monitor;

use crate::cli::Cli;
use babe::{BabeBlockImport, BabeConfiguration, BabeLink, BabeVerifier};
use creditcoin_node_runtime::{self, self as runtime, opaque::Block, RuntimeApi};
use jsonrpsee::core::async_trait;
use parity_scale_codec::Encode;
use primitives::metrics::MiningMetrics;
use sc_client_api::{Backend, BlockBackend};
use sc_consensus::{BlockCheckParams, BlockImportParams, LongestChain, Verifier};
use sc_consensus_babe as babe;
use sc_consensus_grandpa::{LinkHalf, SharedVoterState};
use sc_consensus_pow::PowVerifier;
pub use sc_executor::NativeElseWasmExecutor;
use sc_keystore::LocalKeystore;
use sc_network::ProtocolName;
use sc_service::{
	error::Error as ServiceError, Configuration, TaskManager, TransactionPoolOptions,
};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker};
use sc_transaction_pool::PoolLimit;
use sha3pow::Sha3Algorithm;
use sp_api::NumberFor;
use sp_consensus_babe::SlotDuration;
use sp_core::traits::SpawnNamed;
use sp_inherents::CreateInherentDataProviders;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::{
	app_crypto::Ss58Codec,
	traits::{BlakeTwo256, Block as BlockT},
	Justification, OpaqueExtrinsic,
	{offchain::DbExternalities, traits::IdentifyAccount},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::{oneshot, Notify};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		creditcoin_node_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		creditcoin_node_runtime::native_version()
	}
}

pub(crate) type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type PartialComponentsType<CIDP> = sc_service::PartialComponents<
	FullClient,
	FullBackend,
	FullSelectChain,
	sc_consensus::DefaultImportQueue<Block, FullClient>,
	FullPool,
	(
		BabeImportInitializer,
		GrandpaImportInitializer,
		PowImport<CIDP>,
		Arc<Notify>,
		Option<Telemetry>,
	),
>;

pub type FullPool = sc_transaction_pool::FullPool<Block, FullClient>;

pub type FullNetworkService = Arc<sc_network::NetworkService<Block, BlockHash>>;

pub type BlockTy =
	sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>;

/// Creates a transaction pool config where the limits are 5x the default, unless a limit has been set higher manually
fn create_transaction_pool_config(mut config: TransactionPoolOptions) -> TransactionPoolOptions {
	let set_limit = |limit: &mut PoolLimit, default: &PoolLimit| {
		// set the value to `max(5 * default_value, current_value)`
		let new_setting = |curr: usize, def: usize| curr.max(def.saturating_mul(5));

		limit.count = new_setting(limit.count, default.count);
		limit.total_bytes = new_setting(limit.total_bytes, default.total_bytes);
	};
	let default = TransactionPoolOptions::default();
	set_limit(&mut config.future, &default.future);
	set_limit(&mut config.ready, &default.ready);
	config
}

pub fn new_partial(
	config: &Configuration,
) -> Result<
	PartialComponentsType<impl CreateInherentDataProviders<Block, ()> + Send + Sync + 'static>,
	ServiceError,
> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other("Remote Keystores are not supported.".to_string()));
	}

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", "telemetry_tasks", worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let tx_pool_config = create_transaction_pool_config(config.transaction_pool.clone());
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		tx_pool_config,
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let algorithm = Sha3Algorithm::new(client.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		algorithm.clone(),
		0,
		select_chain.clone(),
		Box::new(move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
			Ok(timestamp)
		}),
	);

	let switch_notif = Arc::new(Notify::new());
	let grandpa_init = grandpa_initializer(
		client.clone(),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
		switch_notif.clone(),
		task_manager.spawn_handle(),
	);
	let babe_init = babe_import_initializer(
		client.clone(),
		grandpa_init.clone(),
		switch_notif.clone(),
		task_manager.spawn_handle(),
	);
	let bi = LazyInit::<BabeImport>::new(babe_init.clone());

	let switcher_block_import =
		BlockImportSwitcher::new(client.clone(), bi, pow_block_import.clone());

	let pow_verifier: PowVerifier<Block, _> = PowVerifier::new(algorithm);
	let slot_duration = SlotDuration::from_millis(runtime::SLOT_DURATION);
	let babe_verifier = LazyInit::<BabeVerifier<_, _, _, _>>::new((
		client.clone(),
		babe_init.clone(),
		select_chain.clone(),
		move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot =
			sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);

			Ok((slot, timestamp))
		},
		telemetry.as_ref().map(|x| x.handle()),
	));

	let switcher_justification_import =
		JustificationImportSwitcher::new(client.clone(), LazyInit::new(grandpa_init.clone()));

	let switcher_verifier = VerifierSwitcher::new(client.clone(), pow_verifier, babe_verifier);

	let import_queue = sc_consensus::BasicQueue::new(
		switcher_verifier,
		Box::new(switcher_block_import),
		Some(Box::new(switcher_justification_import)),
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
	);

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (babe_init, grandpa_init, pow_block_import, switch_notif, telemetry),
	})
}

fn remote_keystore(_url: &str) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

pub fn decode_mining_key(
	mining_key: Option<&str>,
) -> Result<creditcoin_node_runtime::AccountId, String> {
	if let Some(key) = mining_key {
		// raw public key
		if let Some(key_without_prefix) = key.strip_prefix("0x") {
			let key_bytes = hex::decode(key_without_prefix)
				.map_err(|e| format!("Invalid mining key, expected hex: {e}"))?;
			Ok(creditcoin_node_runtime::Signer::from(
				sp_core::ecdsa::Public::from_full(&key_bytes)
					.map_err(|_| String::from("Invalid mining key, expected 33 bytes"))?,
			)
			.into_account())
		} else {
			// ss58 encoded key
			match sp_core::ecdsa::Public::from_ss58check(key) {
				Ok(key) => Ok(creditcoin_node_runtime::Signer::from(key).into_account()),
				Err(err) => match creditcoin_node_runtime::AccountId::from_ss58check(key) {
					Ok(account_id) => Ok(account_id),
					Err(e) => {
						let msg = format!("Invalid mining key, failed to interpret it as an ECDSA public key (error: {err}) and as an account ID (error: {e})");
						log::error!("{}", msg);
						Err(msg)
					},
				},
			}
		}
	} else {
		Err("The node is configured for mining but is missing a mining key".into())
	}
}

/// Builds a new service for a full client.
pub fn new_full(mut config: Configuration, cli: Cli) -> Result<TaskManager, ServiceError> {
	let Cli {
		rpc_mapping, mining_key, mining_threads, monitor_nonce: monitor_nonce_account, ..
	} = cli;

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		mut keystore_container,
		select_chain,
		transaction_pool,
		other: (babe_link, grandpa_link, pow_block_import, switch_notif, mut telemetry),
	} = new_partial(&config)?;

	let client: Arc<FullClient> = client;

	if let Some(url) = &config.keystore_remote {
		match remote_keystore(url) {
			Ok(k) => keystore_container.set_remote_keystore(k),
			Err(e) => {
				return Err(ServiceError::Other(format!(
					"Error hooking up remote keystore for {url}: {e}"
				)))
			},
		};
	}

	let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);

	config
		.network
		.extra_sets
		.push(sc_consensus_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));

	let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None, // TODO: figure out if it's possible to use warp sync only after switched to pos
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
		if let Some(mapping) = rpc_mapping {
			let storage = backend.offchain_storage().unwrap();
			let mut offchain_db = sc_offchain::OffchainDb::new(storage);
			for (chain, uri) in mapping {
				let mut key = Vec::from(chain.as_bytes());
				key.extend("-rpc-uri".bytes());
				offchain_db.local_storage_set(
					sp_core::offchain::StorageKind::PERSISTENT,
					&key,
					&uri.encode(),
				);
			}
		}
	}

	if std::env::var("GRANDPA_HACK").is_ok() {
		// const AUTH_SET_KEY: &[u8] = b"grandpa_voters";
		// sc_finality_grandpa::AuthoritySet::<runtime::Hash, runtime::BlockNumber>::decode(input)

		// backend
		// 	.insert_aux(insert, delete)
	}
	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let tokio_handle = config.tokio_handle.clone();
	let mining_metrics = primitives::metrics::MiningMetrics::new(prometheus_registry.as_ref())?;

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();

		let mining_metrics = mining_metrics.clone();
		Box::new(move |deny_unsafe, _| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				mining_metrics: mining_metrics.clone(),
			};

			crate::rpc::create_full(deps).map_err(Into::into)
		})
	};

	let dev_key_seed = config.dev_key_seed.clone();
	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		client: client.clone(),
		backend: backend.clone(),
		task_manager: &mut task_manager,
		keystore: keystore_container.sync_keystore(),
		transaction_pool: transaction_pool.clone(),
		rpc_builder: rpc_extensions_builder,
		network: network.clone(),
		system_rpc_tx,
		tx_handler_controller,
		sync_service: sync_service.clone(),
		telemetry: telemetry.as_mut(),
	})?;

	if let Some(monitor_target) = monitor_nonce_account {
		if let Some(registry) = prometheus_registry.clone() {
			task_manager.spawn_handle().spawn("nonce_metrics", None, {
				nonce_monitor::task(nonce_monitor::TaskArgs {
					registry,
					monitor_target,
					handlers: rpc_handlers,
					backend,
					keystore: keystore_container.keystore(),
				})
			});
		}
	}

	if switched_to_pos(&client, client.chain_info().best_hash) {
		switch_notif.notify_one();
	}

	AuthorshipSwitcher {
		is_authority: role.is_authority(),
		task_manager: &task_manager,
		tokio_handle,
		pow_params: PowAuthorshipParams {
			block_import: pow_block_import,
			client: client.clone(),
			keystore: keystore_container.local_keystore(),
			select_chain: select_chain.clone(),
			transaction_pool: transaction_pool.clone(),
			mining_metrics,
			pre_runtime: mining_key
				.map(|k| decode_mining_key(Some(&*k)))
				.transpose()?
				.map(|s| s.encode()),
			threads: mining_threads,
			sync_service: sync_service.clone(),
		},
		babe_params: BabeAuthorshipParams {
			client,
			babe_link,
			backoff_authoring_blocks,
			force_authoring,
			grandpa_link,
			grandpa_protocol_name,
			keystore: if role.is_authority() {
				Some(keystore_container.sync_keystore())
			} else {
				None
			},
			name,
			network,
			transaction_pool,
			role,
			select_chain,
			dev_key_seed,
			sync_service,
			enable_grandpa,
			prometheus_registry,
		},
		switch_notif,
	}
	.run();

	network_starter.start_network();
	Ok(task_manager)
}

pub struct LazyInit<T: Init> {
	inner: Option<T>,

	deps: Option<T::Deps>,
}

#[async_trait]
pub trait Init {
	type Deps;
	async fn init(deps: Self::Deps) -> Self;
}

impl<T: Init> LazyInit<T> {
	pub fn new(deps: T::Deps) -> Self {
		Self { inner: None, deps: Some(deps) }
	}

	pub async fn get_mut(&mut self) -> &mut T {
		if self.inner.is_none() {
			log::debug!("Initializing LazyInit <{}>", std::any::type_name::<T>());
			let deps = self.deps.take().expect("LazyInit is initialized only once; qed");
			self.inner = Some(T::init(deps).await);
		}

		self.inner.as_mut().expect("LazyInit is initialized only once; qed")
	}
}

pub type OneshotSender<T> = tokio::sync::oneshot::Sender<T>;

pub type MpscSender<T> = tokio::sync::mpsc::Sender<T>;

struct GrandpaAuthorityProvider {
	client: Arc<FullClient>,
}
impl GrandpaAuthorityProvider {
	fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl sc_consensus_grandpa::GenesisAuthoritySetProvider<BlockTy> for GrandpaAuthorityProvider {
	fn get(&self) -> Result<runtime::GrandpaAuthorityList, sp_blockchain::Error> {
		use sc_client_api::{CallExecutor, ExecutionStrategy, ExecutorProvider, HeaderBackend};
		self.client
			.executor()
			.call(
				self.client.expect_block_hash_from_id(&sp_api::BlockId::Number(
					pos_switch_block(&self.client, self.client.chain_info().best_hash)
						.expect("should have already switched to PoS"),
				))?,
				"GrandpaApi_grandpa_authorities",
				&[],
				ExecutionStrategy::NativeElseWasm,
				sp_core::traits::CallContext::Offchain,
			)
			.and_then(|call_result| {
				parity_scale_codec::Decode::decode(&mut &call_result[..]).map_err(|err| {
					sp_blockchain::Error::CallResultDecode(
						"failed to decode GRANDPA authorities set proof",
						err,
					)
				})
			})
	}
}

fn grandpa_initializer(
	client: Arc<FullClient>,
	select_chain: ChainSelection,
	telemetry: Option<TelemetryHandle>,
	switch_notif: Arc<Notify>,
	spawner: impl SpawnNamed,
) -> GrandpaImportInitializer {
	let (sender, receiver) = tokio::sync::mpsc::channel::<GrandpaImportInitReq>(10);
	spawner.spawn_blocking(
		"grandpa-initializer",
		Some("pos-switcher"),
		Box::pin(async move {
			let mut receiver = receiver;
			let mut grandpa_import: Option<GrandpaImport> = None;
			let mut grandpa_link: TakeOnce<LinkHalf<Block, FullClient, ChainSelection>> =
				TakeOnce::Empty;
			while let Some(req) = receiver.recv().await {
				match &grandpa_import {
					Some(_) => {},
					None => {
						log::debug!("Initializing Grandpa");
						let auth_provider = GrandpaAuthorityProvider::new(client.clone());
						let (import, link) = sc_consensus_grandpa::block_import(
							client.clone(),
							&auth_provider,
							select_chain.clone(),
							telemetry.clone(),
						)
						.unwrap();

						grandpa_import = Some(import.clone());
						grandpa_link = TakeOnce::Full(link);
						log::debug!("Notifying authorship switcher");
						switch_notif.notify_one();
					},
				}
				match req {
					ImportInitReq::Import(import) => {
						log::debug!("Sending grandpa import");

						import.send(grandpa_import.clone().unwrap()).map_err(|_| ()).unwrap();
					},
					ImportInitReq::Link(link) => {
						link.send(grandpa_link.take()).map_err(|_| ()).unwrap();
					},
				}
			}
		}),
	);
	GrandpaImportInitializer::new(sender)
}

enum TakeOnce<T> {
	Empty,
	Taken,
	Full(T),
}

impl<T> TakeOnce<T> {
	fn take(&mut self) -> T {
		match std::mem::replace(self, TakeOnce::Taken) {
			TakeOnce::Empty => panic!("TakeOnce is uninitialized"),
			TakeOnce::Taken => panic!("TakeOnce can only be taken once"),
			TakeOnce::Full(t) => t,
		}
	}
}

fn babe_import_initializer(
	client: Arc<FullClient>,
	mut grandpa_init: GrandpaImportInitializer,
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
						let grandpa_block_import = grandpa_init.request_import().await;
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
							grandpa_block_import,
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

pub enum ImportInitReq<I, L> {
	Import(OneshotSender<I>),
	Link(OneshotSender<L>),
}

pub type BabeImportInitReq = ImportInitReq<BabeImport, BabeLink<Block>>;

pub type BabeImportInitializer = ImportInitializerService<BabeImport, BabeLink<Block>>;

pub type GrandpaImportInitReq =
	ImportInitReq<GrandpaImport, LinkHalf<Block, FullClient, ChainSelection>>;

pub type GrandpaImportInitializer =
	ImportInitializerService<GrandpaImport, LinkHalf<Block, FullClient, ChainSelection>>;

pub struct ImportInitializerService<I, L> {
	inner: MpscSender<ImportInitReq<I, L>>,
}

impl<I, L> Clone for ImportInitializerService<I, L> {
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<I, L> ImportInitializerService<I, L> {
	pub fn new(inner: MpscSender<ImportInitReq<I, L>>) -> Self {
		Self { inner }
	}

	pub async fn request_import(&mut self) -> I {
		let (tx, rx) = oneshot::channel();
		self.inner.send(ImportInitReq::Import(tx)).await.map_err(|_| ()).unwrap();
		rx.await.unwrap()
	}

	pub async fn request_link(&mut self) -> L {
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

pub type GrandpaImport =
	sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>;

pub type BabeImport = BabeBlockImport<Block, FullClient, GrandpaImport>;

pub type PowImport<CIDP> = sc_consensus_pow::PowBlockImport<
	Block,
	Arc<FullClient>,
	FullClient,
	FullSelectChain,
	Sha3Algorithm<FullClient>,
	CIDP,
>;

pub type SyncService = Arc<sc_network_sync::SyncingService<Block>>;

pub struct AuthorshipSwitcher<'a, CIDP> {
	task_manager: &'a TaskManager,
	switch_notif: Arc<tokio::sync::Notify>,
	is_authority: bool,
	pow_params: PowAuthorshipParams<CIDP>,
	tokio_handle: tokio::runtime::Handle,
	babe_params: BabeAuthorshipParams,
}

impl<'a, CIDP> AuthorshipSwitcher<'a, CIDP>
where
	CIDP: CreateInherentDataProviders<Block, ()> + Send + Sync + 'static,
{
	pub fn run(self) {
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

struct BabeAuthorshipParams {
	client: Arc<FullClient>,
	backoff_authoring_blocks: Option<()>,
	force_authoring: bool,
	babe_link: BabeImportInitializer,
	transaction_pool: Arc<FullPool>,
	network: FullNetworkService,
	keystore: Option<SyncCryptoStorePtr>,
	grandpa_link: GrandpaImportInitializer,
	grandpa_protocol_name: ProtocolName,
	select_chain: ChainSelection,
	role: sc_network::config::Role,
	name: String,
	dev_key_seed: Option<String>,
	sync_service: SyncService,
	enable_grandpa: bool,
	prometheus_registry: Option<substrate_prometheus_endpoint::Registry>,
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
		mut grandpa_link,
		grandpa_protocol_name,
		select_chain,
		role,
		name,
		dev_key_seed,
		sync_service,
		enable_grandpa,
		prometheus_registry,
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
			link: grandpa_link.request_link().await,
			network,
			voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state: SharedVoterState::empty(),
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

struct PowAuthorshipParams<CIDP> {
	client: Arc<FullClient>,
	select_chain: FullSelectChain,
	block_import: PowImport<CIDP>,
	transaction_pool: Arc<FullPool>,
	mining_metrics: MiningMetrics,
	pre_runtime: Option<Vec<u8>>,
	keystore: Option<Arc<LocalKeystore>>,
	threads: Option<usize>,
	sync_service: SyncService,
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

pub struct BlockImportSwitcher<CIDP> {
	client: Arc<FullClient>,
	babe_import: LazyInit<BabeImport>,
	pow_import: PowImport<CIDP>,
}

fn switched_to_pos(client: &Arc<FullClient>, at_hash: BlockHash) -> bool {
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
	pub fn new(
		client: Arc<FullClient>,
		babe_import: LazyInit<BabeImport>,
		pow_import: PowImport<CIDP>,
	) -> Self {
		Self { client, babe_import, pow_import }
	}

	pub fn switched_to_pos(&self, at_hash: BlockHash) -> bool {
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

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("Justification import not supported for PoW blocks")]
pub struct JustificationNotSupported;

type BlockHash = <Block as BlockT>::Hash;

pub struct JustificationImportSwitcher {
	client: Arc<FullClient>,
	grandpa_import: LazyInit<GrandpaImport>,
}

impl JustificationImportSwitcher {
	pub fn new(client: Arc<FullClient>, grandpa_import: LazyInit<GrandpaImport>) -> Self {
		Self { client, grandpa_import }
	}

	pub fn switched_to_pos(&self, at_hash: BlockHash) -> bool {
		switched_to_pos(&self.client, at_hash)
	}
}

#[async_trait]
impl sc_consensus::JustificationImport<Block> for JustificationImportSwitcher {
	type Error = sp_consensus::Error;

	async fn on_start(&mut self) -> Vec<(BlockHash, NumberFor<Block>)> {
		log::debug!("justification import on_start");
		if self.switched_to_pos(self.client.chain_info().best_hash) {
			self.grandpa_import.get_mut().await.on_start().await
		} else {
			Vec::new()
		}
	}

	async fn import_justification(
		&mut self,
		hash: BlockHash,
		number: NumberFor<Block>,
		justification: Justification,
	) -> Result<(), Self::Error> {
		log::debug!("Importing justification for block #{number}");
		if self.switched_to_pos(hash) {
			self.grandpa_import
				.get_mut()
				.await
				.import_justification(hash, number, justification)
				.await
		} else {
			Err(sp_consensus::Error::Other(Box::new(JustificationNotSupported)))
		}
	}
}

pub type ChainSelection = LongestChain<FullBackend, Block>;

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

pub struct VerifierSwitcher<CIDP>
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
	pub fn new(
		client: Arc<FullClient>,
		pow_verifier: PowVerifier<Block, Sha3Algorithm<FullClient>>,
		babe_verifier: LazyInit<BabeVerifier<Block, FullClient, ChainSelection, CIDP>>,
	) -> Self {
		Self { client, pow_verifier, babe_verifier }
	}

	pub fn switched_to_pos(&self, at_hash: BlockHash) -> bool {
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
