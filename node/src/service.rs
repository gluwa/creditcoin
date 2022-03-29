//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use codec::Encode;
use creditcoin_node_runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{Backend, ExecutorProvider};
pub use sc_executor::NativeElseWasmExecutor;
use sc_keystore::LocalKeystore;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sha3pow::Sha3Algorithm;
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::{app_crypto::Ss58Codec, offchain::DbExternalities, traits::IdentifyAccount};
use std::{sync::Arc, thread, time::Duration};

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

type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_consensus_pow::PowBlockImport<
				Block,
				Arc<FullClient>,
				FullClient,
				FullSelectChain,
				Sha3Algorithm<FullClient>,
				sp_consensus::CanAuthorWithNativeVersion<
					<FullClient as ExecutorProvider<Block>>::Executor,
				>,
				impl CreateInherentDataProviders<Block, ()>,
			>,
			Option<Telemetry>,
		),
	>,
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

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

	let algorithm = Sha3Algorithm::new(client.clone());

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		algorithm.clone(),
		0,
		select_chain.clone(),
		move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
			Ok(timestamp)
		},
		can_author_with,
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import.clone()),
		None,
		algorithm,
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
	)?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (pow_block_import, telemetry),
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
			let key_bytes = hex::decode(&key_without_prefix)
				.map_err(|e| format!("Invalid mining key, expected hex: {}", e))?;
			Ok(creditcoin_node_runtime::Signer::from(
				sp_core::ecdsa::Public::from_full(&*key_bytes)
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
						let msg = format!("Invalid mining key, failed to interpret it as an ECDSA public key (error: {}) and as an account ID (error: {})", err, e);
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
pub fn new_full(
	config: Configuration,
	mining_key: Option<&str>,
	mining_threads: Option<usize>,
	rpc_mapping: Option<impl IntoIterator<Item = (String, String)>>,
) -> Result<TaskManager, ServiceError> {
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		mut keystore_container,
		select_chain,
		transaction_pool,
		other: (pow_block_import, mut telemetry),
	} = new_partial(&config)?;

	if let Some(url) = &config.keystore_remote {
		match remote_keystore(url) {
			Ok(k) => keystore_container.set_remote_keystore(k),
			Err(e) => {
				return Err(ServiceError::Other(format!(
					"Error hooking up remote keystore for {}: {}",
					url, e
				)))
			},
		};
	}

	let (network, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: None,
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

	let role = config.role.clone();
	let _force_authoring = config.force_authoring;
	let _backoff_authoring_blocks: Option<()> = None;
	let _name = config.network.node_name.clone();
	let _enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let backend = backend.clone();
		let task_executor =
			Arc::new(sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle()));

		Box::new(move |deny_unsafe, _| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				backend: backend.clone(),
				executor: task_executor.clone(),
				deny_unsafe,
			};

			Ok(crate::rpc::create_full(deps))
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder,
		backend,
		system_rpc_tx,
		config,
		telemetry: telemetry.as_mut(),
	})?;

	if role.is_authority() {
		let mining_key = decode_mining_key(mining_key)?;
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let algorithm = Sha3Algorithm::new(client.clone());

		let (worker, worker_task) = sc_consensus_pow::start_mining_worker(
			Box::new(pow_block_import),
			client.clone(),
			select_chain,
			algorithm,
			proposer_factory,
			network.clone(),
			network,
			Some(mining_key.encode()),
			move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
				Ok(timestamp)
			},
			Duration::from_secs(10),
			Duration::from_secs(10),
			can_author_with,
		);

		task_manager
			.spawn_essential_handle()
			.spawn_blocking("pow", "pow_group", worker_task);

		let threads = mining_threads.unwrap_or_else(num_cpus::get);
		for _ in 0..threads {
			if let Some(keystore) = keystore_container.local_keystore() {
				let worker = worker.clone();
				let client = client.clone();

				thread::spawn(move || loop {
					let metadata = worker.metadata();
					let version = worker.version();
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
								Ok(None) => {},
								Err(e) => eprintln!("Mining error: {}", e),
							}
							if version != worker.version() {
								break;
							}
						}
					} else {
						thread::sleep(Duration::from_millis(500));
					}
				});
			}
		}
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let _keystore =
		if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	network_starter.start_network();
	Ok(task_manager)
}
