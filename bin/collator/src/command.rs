use crate::{
	cli::{Cli, Subcommand},
	local::{self, development_config},
};

use log::{error, info};

use futures::future::BoxFuture;

use crate::local::{chain_spec, service};
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use sp_core::hexdisplay::ascii_format;
use sp_keyring::Sr25519Keyring;

use common_primitives::Block;

#[cfg(feature = "try-runtime")]
use try_runtime_cli::block_building_info::timestamp_with_aura_info;

pub trait IdentifyChain {
	fn is_production(&self) -> bool;
	fn is_development(&self) -> bool;
}
impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_production(&self) -> bool {
		self.id().starts_with("production")
	}
	fn is_development(&self) -> bool {
		self.id().starts_with("dev")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_production(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_production(self)
	}
	fn is_development(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_development(self)
	}
}

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

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		info!("id-arg: {:?}", id);
		let spec = match id {
			"" =>
				return Err(
					"Please specify which chain you want to run, e.g. --dev or --chain=local"
						.into(),
				),

			"dev" => Box::new(development_config()),

			path => {
				let chain_spec = chain_spec::ChainSpec::from_json_file(path.into())?;

				if chain_spec.is_development() {
					Box::new(chain_spec::ChainSpec::from_json_file(path.into())?)
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
	info!("id-arg: {:?}", cli);

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
					{
						return service::new_full::<
							development_runtime::RuntimeApi,
							service::DevelopmentExecutor,
						>(config)
						.map_err(sc_cli::Error::Service);
					}
				} else {
					return Err(service::RUNTIME_NOT_AVAILABLE.into());
				}
			})
		},
	}
}
