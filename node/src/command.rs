use crate::{
	benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder},
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};
use creditcoin_node_runtime::{Block, ExistentialDeposit};
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::{ChainSpec, Database, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use sp_keyring::Sr25519Keyring;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Creditcoin Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"" => {
				let msg =
					"Please specify the chain with '--chain main' or '--chain test'".to_owned();
				log::error!("{}", msg);
				return Err(msg);
			},
			"dev" => Box::new(chain_spec::development_config()?),
			"local" => Box::new(chain_spec::local_testnet_config()?),
			"test" | "testnet" => Box::new(chain_spec::testnet_config()?),
			"main" | "mainnet" => Box::new(chain_spec::mainnet_config()?),
			path => {
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?)
			},
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&creditcoin_node_runtime::VERSION
	}
}

#[cfg(test)]
pub mod set_db_tests {
	use crate::cli::Cli;
	use crate::command::maybe_set_db;
	use sc_cli::{clap::Parser, Database};

	#[test]
	fn maybe_set_db_changes_config_when_no_db_is_set() {
		let mut cli = Cli::parse_from(Option::<&str>::None);

		// Start with no value set
		assert_eq!(cli.run.import_params.database_params.database, None);
		maybe_set_db(&mut cli);

		// Expect it is now set to ParityDB
		assert_eq!(cli.run.import_params.database_params.database, Some(Database::ParityDb));
	}

	#[test]
	fn maybe_set_db_does_nothing_when_db_already_set() {
		let mut cli = Cli::parse_from(Option::<&str>::None);

		// Set the value to something besides ParityDB
		cli.run.import_params.database_params.database = Some(Database::RocksDb);
		assert_eq!(cli.run.import_params.database_params.database, Some(Database::RocksDb));

		maybe_set_db(&mut cli);

		// Expect that the value is unchanged
		assert_eq!(cli.run.import_params.database_params.database, Some(Database::RocksDb));
	}
}

///Use ParityDB unless the user specified otherwise
fn maybe_set_db(cli: &mut Cli) {
	//Set the DB to ParityDB if needed
	match &mut cli.run.import_params.database_params.database {
		Some(_) => {}, // The user specified a database, so do nothing
		db => {
			// No Database set, default to ParityDB
			*db = Some(Database::ParityDb);
		},
	};
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let mut cli = Cli::from_args();

	//Set the DB to ParityDB if needed
	maybe_set_db(&mut cli);

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, backend, None), task_manager))
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use creditcoin_node_runtime::SLOT_DURATION;
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			use sp_io::SubstrateHostFunctions;
			use try_runtime_cli::block_building_info::substrate_info;

			let runner = cli.create_runner(cmd)?;
			// https://github.com/paritytech/substrate/pull/12896/files#diff-c57da6fbeff8c46ce15f55ea42fedaa5a4684d79578006ce4af01ae04fd6b8f8R245
			// for reference implementation
			let info_provider = substrate_info(SLOT_DURATION);

			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				Ok((
					cmd.run::<Block, ExtendedHostFunctions<
						SubstrateHostFunctions,
						<service::ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
					>, _>(Some(info_provider)),
					task_manager,
				))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(&**cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match &**cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
								You can enable it with `--features runtime-benchmarks`."
									.into(),
							);
						}

						cmd.run::<Block, service::ExecutorDispatch>(config)
					},
					BenchmarkCmd::Block(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						cmd.run(client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						let PartialComponents { client, backend, .. } =
							service::new_partial(&config)?;
						let db = backend.expose_db();
						let storage = backend.expose_storage();

						cmd.run(config, client, db, storage)
					},
					BenchmarkCmd::Overhead(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						let ext_builder = RemarkBuilder::new(client.clone());

						cmd.run(
							config,
							client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_builder,
						)
					},
					BenchmarkCmd::Extrinsic(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						// Register the *Remark* and *TKA* builders.
						let ext_factory = ExtrinsicFactory(vec![
							Box::new(RemarkBuilder::new(client.clone())),
							Box::new(TransferKeepAliveBuilder::new(
								client.clone(),
								Sr25519Keyring::Alice.to_account_id(),
								ExistentialDeposit::get(),
							)),
						]);

						cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
					},
					BenchmarkCmd::Machine(cmd) => {
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
					},
				}
			})
		},
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config, cli).map_err(sc_cli::Error::Service)
			})
		},
	}
}
