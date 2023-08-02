//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

mod consensus_switcher;
mod nonce_monitor;

use crate::cli::Cli;
use babe::BabeVerifier;
use creditcoin_node_runtime::{self, self as runtime, opaque::Block, RuntimeApi};

use parity_scale_codec::Encode;

use sc_client_api::{Backend, BlockBackend};

use sc_consensus_babe as babe;

use sc_consensus_pow::PowVerifier;
pub use sc_executor::NativeElseWasmExecutor;
use sc_keystore::LocalKeystore;

use sc_service::{
	error::Error as ServiceError, Configuration, TaskManager, TransactionPoolOptions,
};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool::PoolLimit;
use sha3pow::Sha3Algorithm;

use sp_consensus_babe::SlotDuration;
use sp_inherents::CreateInherentDataProviders;

use sp_runtime::{
	app_crypto::Ss58Codec,
	traits::{BlakeTwo256, Block as BlockT},
	OpaqueExtrinsic,
	{offchain::DbExternalities, traits::IdentifyAccount},
};
use std::sync::Arc;
use tokio::sync::Notify;

use self::consensus_switcher::{
	switched_to_pos, AuthorshipSwitcher, BabeAuthorshipParams, BabeImport, BabeImportInitializer,
	BlockImportSwitcher, GrandpaLink, LazyInit, PowAuthorshipParams, PowImport, VerifierSwitcher,
};

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
	(BabeImportInitializer, GrandpaLink, PowImport<CIDP>, Arc<Notify>, Option<Telemetry>),
>;

type BlockHash = <Block as BlockT>::Hash;

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

pub(crate) fn new_partial(
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
	let telemetry_handle = telemetry.as_ref().map(|telemetry| telemetry.handle());

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

	let initial_authorities = sc_chain_spec::get_extension::<
		super::chain_spec::GrandpaInitialAuthorities,
	>(config.chain_spec.extensions())
	.unwrap();

	log::debug!("initial grandpa authorities: {initial_authorities:?}");
	let auth_provider = {
		let grandpa_authorities = initial_authorities
			.grandpa_initial_authorities
			.clone()
			.expect("No initial authorities configured for GRANDPA");
		#[cfg(feature = "fast-runtime")]
		{
			consensus_switcher::GrandpaAuthorityProvider::with_client(
				client.clone(),
				grandpa_authorities,
			)
		}
		#[cfg(not(feature = "fast-runtime"))]
		{
			consensus_switcher::GrandpaAuthorityProvider::new(grandpa_authorities)
		}
	};
	let (grandpa_import, grandpa_link) = sc_consensus_grandpa::block_import(
		client.clone(),
		&auth_provider,
		select_chain.clone(),
		telemetry_handle,
	)
	.unwrap();

	let switch_notif = Arc::new(Notify::new());

	let babe_init = consensus_switcher::babe_import_initializer(
		client.clone(),
		grandpa_import.clone(),
		switch_notif.clone(),
		task_manager.spawn_handle(),
	);
	let bi = LazyInit::<BabeImport>::new(babe_init.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		grandpa_import.clone(),
		client.clone(),
		algorithm.clone(),
		0,
		select_chain.clone(),
		Box::new(move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
			Ok(timestamp)
		}),
	);

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

	let switcher_verifier = VerifierSwitcher::new(client.clone(), pow_verifier, babe_verifier);

	let import_queue = sc_consensus::BasicQueue::new(
		switcher_verifier,
		Box::new(switcher_block_import),
		Some(Box::new(grandpa_import)),
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
		other: (babe_init, grandpa_link, pow_block_import, switch_notif, telemetry),
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
