//use crate::benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder};

use crate::cli::{Cli, Subcommand};

use futures::future::BoxFuture;
use service::{chain_spec, IdentifyVariant};

use development_runtime::Block;

use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use sp_core::hexdisplay::ascii_format;
use sp_keyring::Sr25519Keyring;

#[cfg(feature = "try-runtime")]
use try_runtime_cli::block_building_info::timestamp_with_aura_info;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Nativex Node".into()
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
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),
			#[cfg(feature = "with-development-runtime")]
			"dev" => Box::new(chain_spec::development::development_config()),
			#[cfg(feature = "with-development-runtime")]
			"local" => Box::new(chain_spec::development::development_config()),
			#[cfg(feature = "with-development-runtime")]
			"staging" => Box::new(chain_spec::development::staging_testnet_config()),
			//path => Box::new(chain_spec::development::ChainSpec::from_json_file(
			//	std::path::PathBuf::from(path),
			//)?),
			path => {
				let path = std::path::PathBuf::from(path);
				let chain_spec =
					Box::new(chain_spec::development::ChainSpec::from_json_file(path.clone())?)
						as Box<dyn sc_service::ChainSpec>;
				if chain_spec.is_development() {
					#[cfg(feature = "with-development-runtime")]
					{
						Box::new(chain_spec::development::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-development-runtime"))]
					return Err(service::DEVELOPMENT_RUNTIME_NOT_AVAILABLE.into());
				} else {
					return Err(service::RUNTIME_NOT_AVAILABLE.into());
				}
			},
		};
		Ok(spec)
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&mut config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config)?;
				let aux_revert = Box::new(|client, _, blocks| {
					grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			todo!()
		},
		// Some(Subcommand::Benchmark(cmd)) => {
		// 	let runner = cli.create_runner(cmd)?;

		// 	runner.sync_run(|config| {
		// 		// This switch needs to be in the client, since the client decides
		// 		// which sub-commands it wants to support.
		// 		match cmd {
		// 			BenchmarkCmd::Pallet(cmd) => {
		// 				if !cfg!(feature = "runtime-benchmarks") {
		// 					return Err(
		// 						"Runtime benchmarking wasn't enabled when building the node. \
		// 					You can enable it with `--features runtime-benchmarks`."
		// 							.into(),
		// 					);
		// 				} else {
		// 					let chain_spec = &config.chain_spec;
		// 					if chain_spec.is_development() {
		// 						#[cfg(feature = "with-development-runtime")]
		// 						return cmd
		// 							.run::<service::development_runtime::Block, service::DevelopmentExecutor>(
		// 								config,
		// 				);
		// 			#[cfg(not(feature = "with-development-runtime"))]
		// 			return Err(service::RUNTIME_NOT_AVAILABLE.into());
		// 		} else {
		// 			return Err(service::RUNTIME_NOT_AVAILABLE.into());
		// 		}
		// 	}

		// 	//cmd.run::<Block, service::ExecutorDispatch>(config)
		// },
		// BenchmarkCmd::Block(cmd) => {
		// 	let PartialComponents { client, .. } = service::new_partial(&config)?;
		// 	cmd.run(client)
		// },
		// #[cfg(not(feature = "runtime-benchmarks"))]
		// BenchmarkCmd::Storage(_) => Err(
		// 	"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
		// 		.into(),
		// ),
		// #[cfg(feature = "runtime-benchmarks")]
		// BenchmarkCmd::Storage(cmd) => {
		// 	let PartialComponents { client, backend, .. } =
		// 		service::new_partial(&config)?;
		// 	let db = backend.expose_db();
		// 	let storage = backend.expose_storage();

		// 	cmd.run(config, client, db, storage)
		// },
		// BenchmarkCmd::Overhead(cmd) => {
		// 	let PartialComponents { client, .. } = service::new_partial(&config)?;
		// 	let ext_builder = RemarkBuilder::new(client.clone());

		// 	cmd.run(
		// 		config,
		// 		client,
		// 		inherent_benchmark_data()?,
		// 		Vec::new(),
		// 		&ext_builder,
		// 	)
		// },
		// BenchmarkCmd::Extrinsic(cmd) => {
		// 	let PartialComponents { client, .. } = service::new_partial(&config)?;
		// 	// Register the *Remark* and *TKA* builders.
		// 	let ext_factory = ExtrinsicFactory(vec![
		// 		Box::new(RemarkBuilder::new(client.clone())),
		// 		Box::new(TransferKeepAliveBuilder::new(
		// 			client.clone(),
		// 			Sr25519Keyring::Alice.to_account_id(),
		// 			EXISTENTIAL_DEPOSIT,
		// 		)),
		// 	]);

		// 	cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
		// },
		// BenchmarkCmd::Machine(cmd) => {
		// 	cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
		// 			// },
		// 		}
		// 	})
		// },
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use crate::service::ExecutorDispatch;
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				let info_provider = timestamp_with_aura_info(6000);

				Ok((
					cmd.run::<Block, ExtendedHostFunctions<
						sp_io::SubstrateHostFunctions,
						<ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
					>, _>(Some(info_provider)),
					task_manager,
				))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				let chain_spec = &config.chain_spec;
				if chain_spec.is_development() {
					#[cfg(feature = "with-development-runtime")]
					{
						return service::new_full::<
							service::development_runtime::RuntimeApi,
							service::DevelopmentExecutor,
						>(config)
						.map_err(sc_cli::Error::Service);
					}
					#[cfg(not(feature = "with-development-runtime"))]
					return Err(service::RUNTIME_NOT_AVAILABLE.into());
				} else {
					return Err(service::RUNTIME_NOT_AVAILABLE.into());
				}
			})
		},
	}
}
