//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use codec::Encode;
use frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE;
use frame_system_rpc_runtime_api::AccountNonceApi;
use futures::prelude::*;
use sc_client_api::BlockBackend;
use sc_consensus::metrics;
use sc_consensus_babe::{self, SlotProportion};
pub use sc_executor::NativeElseWasmExecutor;
use sc_executor::NativeExecutionDispatch;
use sc_network::{
	event::Event, service::traits::NetworkService, NetworkBackend, NetworkEventStream,
};
use sc_network_sync::{strategy::warp::WarpSyncParams, SyncingService};
use sc_service::{
	config::Configuration, error::Error as ServiceError, ChainSpec, RpcHandlers, TaskManager,
};
//use sc_statement_store::Store as StatementStore;
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ConstructRuntimeApi;

use sp_runtime::{
	generic,
	traits::{BlakeTwo256, Block as BlockT},
	SaturatedConversion,
};
use sp_trie::PrefixedMemoryDB;
use std::{path::Path, sync::Arc};

pub use common_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Index};

pub mod chain_spec;
pub mod client;
pub mod rpc;

#[cfg(feature = "with-development-runtime")]
pub use chain_spec::development::*;

pub use client::*;
#[cfg(feature = "with-development-runtime")]
pub use development_runtime;

pub const RUNTIME_NOT_AVAILABLE: &str = "Runtime is not available";
pub const DEVELOPMENT_RUNTIME_NOT_AVAILABLE: &str = "Development runtime is not available";

pub type FullClient<RuntimeApi, Executor> =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type TransactionPool<RuntimeApi, Executor> =
	sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>;
type FullGranpaBlockImport<RuntimeApi, Executor> = grandpa::GrandpaBlockImport<
	FullBackend,
	Block,
	FullClient<RuntimeApi, Executor>,
	FullSelectChain,
>;
/// The minimum period of blocks on which justifications will be
/// imported and generated.
const GRANDPA_JUSTIFICATION_PERIOD: u32 = 512;

#[cfg(feature = "with-development-runtime")]
pub struct DevelopmentExecutor;

#[cfg(feature = "with-development-runtime")]
impl sc_executor::NativeExecutionDispatch for DevelopmentExecutor {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		development_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		development_runtime::native_version()
	}
}

pub trait IdentifyVariant {
	fn is_production(&self) -> bool;
	fn is_development(&self) -> bool;
}
impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_production(&self) -> bool {
		self.id().starts_with("production")
	}
	fn is_development(&self) -> bool {
		self.id().starts_with("dev")
	}
}

/// Builds a new object suitable for chain operations.
#[allow(clippy::type_complexity)]
pub fn new_chain_ops(
	config: &mut Configuration,
) -> Result<
	(Arc<Client>, Arc<FullBackend>, sc_consensus::BasicQueue<Block>, TaskManager),
	ServiceError,
> {
	match &config.chain_spec {
		#[cfg(feature = "with-development-runtime")]
		spec if spec.is_development() =>
			new_chain_ops_inner::<development_runtime::RuntimeApi, DevelopmentExecutor>(config),
		_ => panic!("invalid chain spec"),
	}
}

#[allow(clippy::type_complexity)]
fn new_chain_ops_inner<RuntimeApi, Executor>(
	mut config: &mut Configuration,
) -> Result<
	(Arc<Client>, Arc<FullBackend>, sc_consensus::BasicQueue<Block>, TaskManager),
	ServiceError,
>
where
	Client: From<Arc<crate::FullClient<RuntimeApi, Executor>>>,
	RuntimeApi:
		ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: NativeExecutionDispatch + 'static,
{
	config.keystore = sc_service::config::KeystoreConfig::InMemory;
	let sc_service::PartialComponents { client, backend, import_queue, task_manager, .. } =
		new_partial::<RuntimeApi, Executor>(config)?;
	Ok((Arc::new(Client::from(client)), backend, import_queue, task_manager))
}

pub fn new_partial<RuntimeApi, Executor>(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient<RuntimeApi, Executor>,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block>,
		TransactionPool<RuntimeApi, Executor>,
		(
			impl Fn(
				rpc::DenyUnsafe,
				sc_rpc::SubscriptionTaskExecutor,
			) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>,
			(
				sc_consensus_babe::BabeBlockImport<
					Block,
					FullClient<RuntimeApi, Executor>,
					FullGranpaBlockImport<RuntimeApi, Executor>,
				>,
				grandpa::LinkHalf<Block, FullClient<RuntimeApi, Executor>, FullSelectChain>,
				sc_consensus_babe::BabeLink<Block>,
			),
			grandpa::SharedVoterState,
			Option<Telemetry>,
			//Arc<StatementStore>,
		),
	>,
	ServiceError,
>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: NativeExecutionDispatch + 'static,
{
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

	let executor = sc_service::new_native_or_wasm_executor(&config);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
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

	let (grandpa_block_import, grandpa_link) = grandpa::block_import(
		client.clone(),
		GRANDPA_JUSTIFICATION_PERIOD,
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;
	let justification_import = grandpa_block_import.clone();
	let (block_import, babe_link) = sc_consensus_babe::block_import(
		sc_consensus_babe::configuration(&*client)?,
		grandpa_block_import,
		client.clone(),
	)?;

	let slot_duration = babe_link.config().slot_duration();
	let (import_queue, babe_worker_handle) =
		sc_consensus_babe::import_queue(sc_consensus_babe::ImportQueueParams {
			link: babe_link.clone(),
			block_import: block_import.clone(),
			justification_import: Some(Box::new(justification_import)),
			client: client.clone(),
			select_chain: select_chain.clone(),
			create_inherent_data_providers: move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
				let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
				Ok((slot, timestamp))
			},
			spawner: &task_manager.spawn_essential_handle(),
			registry: config.prometheus_registry(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool.clone()),
		})?;
	let import_setup = (block_import, grandpa_link, babe_link);
	let (rpc_extension_builder, rpc_setup) = {
		let (_, grandpa_link, _) = &import_setup;
		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = grandpa::SharedVoterState::empty();
		let shared_voter_state2 = shared_voter_state.clone();
		let finality_proof_provider = grandpa::FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let keystore = keystore_container.keystore();
		let chain_spec = config.chain_spec.cloned_box();

		let rpc_backend = backend.clone();
		//let rpc_statement_store = statement_store.clone();
		let rpc_extensions_builder = move |deny_unsafe, subscription_executor| {
			let deps = rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				chain_spec: chain_spec.cloned_box(),
				babe: rpc::BabeDeps {
					keystore: keystore.clone(),
					babe_worker_handle: babe_worker_handle.clone(),
				},
				deny_unsafe,
				grandpa: rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor,
					finality_provider: finality_proof_provider.clone(),
				},
			};
			rpc::create_full(deps, rpc_backend.clone()).map_err(Into::into)
		};
		(rpc_extensions_builder, shared_voter_state2)
	};

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (rpc_extension_builder, import_setup, rpc_setup, telemetry),
	})
}

/// Result of [`new_full_base`].
pub struct NewFullBase<RuntimeApi, Executor>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	// RuntimeApi::RuntimeApi:
	// 	RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	Executor: NativeExecutionDispatch + 'static,
{
	/// The task of manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient<RuntimeApi, Executor>>,
	/// The networking service of the node.
	// pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	pub network: Arc<dyn NetworkService>,
	/// The syncing service of the node.
	pub sync: Arc<SyncingService<Block>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool<RuntimeApi, Executor>>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

/// Builds a new service for a full client.
pub fn new_full_base<N: NetworkBackend<Block, <Block as BlockT>::Hash>, RuntimeApi, Executor>(
	config: Configuration,
) -> Result<NewFullBase<RuntimeApi, Executor>, ServiceError>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: NativeExecutionDispatch + 'static,
{
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (rpc_builder, import_setup, rpc_setup, mut telemetry),
	} = new_partial(&config)?;

	let metrics = N::register_notification_metrics(
		config.prometheus_config.as_ref().map(|cfg| &cfg.registry),
	);

	let shared_voter_state = rpc_setup;
	let auth_disc_publish_non_global_ips = config.network.allow_non_globals_in_dht;
	let mut net_config =
		sc_network::config::FullNetworkConfiguration::<_, _, N>::new(&config.network);

	let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
	let peer_store_handle = net_config.peer_store_handle();

	let grandpa_protocol_name = grandpa::protocol_standard_name(&genesis_hash, &config.chain_spec);

	let (grandpa_protocol_config, grandpa_notification_service) =
		grandpa::grandpa_peers_set_config::<_, N>(
			grandpa_protocol_name.clone(),
			metrics.clone(),
			Arc::clone(&peer_store_handle),
		);
	net_config.add_notification_protocol(grandpa_protocol_config);

	// let statement_handler_proto = sc_network_statement::StatementHandlerPrototype::new(
	// 	client
	// 		.block_hash((0u32).into())
	// 		.ok()
	// 		.flatten()
	// 		.expect("Genesis block exist; qed"),
	// 	config.chain_spec.fork_id(),
	// );
	//net_config.add_notification_protocol(statement_handler_proto.set_config());

	let warp_sync = Arc::new(grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		import_setup.1.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: Some(WarpSyncParams::WithProvider(warp_sync)),
			block_relay: None,
			metrics,
		})?;

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	//let backoff_authoring_blocks: Option<()> = None;
	let backoff_authoring_blocks =
		Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let enable_offchain_worker = config.offchain_worker.enabled;

	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend,
		client: client.clone(),
		keystore: keystore_container.keystore(),
		network: network.clone(),
		rpc_builder: Box::new(rpc_builder),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		system_rpc_tx,
		tx_handler_controller,
		sync_service: sync_service.clone(),
		telemetry: telemetry.as_mut(),
	})?;
	let (block_import, grandpa_link, babe_link) = import_setup;
	//(wit_startup_data)(&block_import, &babe_link);

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();
		let babe_config = sc_consensus_babe::BabeParams {
			keystore: keystore_container.keystore(),
			client: client.clone(),
			select_chain,
			env: proposer,
			block_import,
			sync_oracle: sync_service.clone(),
			justification_sync_link: sync_service.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
					let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_slot_duration(*timestamp, slot_duration);
					let storage_proof =
						sp_transaction_storage_proof::registration::new_data_provider(
							&*client_clone,
							&parent,
						)?;
					Ok((slot, timestamp, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};
		let babe = sc_consensus_babe::start_babe(babe_config)?;
		task_manager.spawn_essential_handle().spawn_blocking(
			"babe-proposer",
			Some("block-authroing"),
			babe,
		);
	}

	if role.is_authority() {
		let authority_discovery_role =
			sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore());
		let dht_event_stream =
			network.event_stream("authority-discovery").filter_map(|e| async move {
				match e {
					Event::Dht(e) => Some(e),
					_ => None,
				}
			});
		let (authority_discovery_worker, _service) =
			sc_authority_discovery::new_worker_and_service_with_config(
				sc_authority_discovery::WorkerConfig {
					publish_non_global_ips: auth_disc_publish_non_global_ips,
					..Default::default()
				},
				client.clone(),
				Arc::new(network.clone()),
				Box::pin(dht_event_stream),
				authority_discovery_role,
				prometheus_registry.clone(),
			);
		task_manager.spawn_handle().spawn(
			"authority-discovery-worker",
			Some("networking"),
			authority_discovery_worker.run(),
		);
	}
	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below
	let keystore = if role.is_authority() { Some(keystore_container.keystore()) } else { None };

	let grandpa_config = grandpa::Config {
		// FIXE #1578 make this available through chainspec.
		gossip_duration: std::time::Duration::from_millis(333),
		justification_generation_period: GRANDPA_JUSTIFICATION_PERIOD,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_params = grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network: network.clone(),
			sync: Arc::new(sync_service.clone()),
			notification_service: grandpa_notification_service,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			voting_rule: grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry: prometheus_registry.clone(),
			shared_voter_state,
			offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool.clone()),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			grandpa::run_grandpa_voter(grandpa_params)?,
		);
	}

	// if enable_offchain_worker {
	// 	task_manager.spawn_handle().spawn(
	// 		"offchain-workers-runner",
	// 		"offchain-work",
	// 		sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
	// 			runtime_api_provider: client.clone(),
	// 			keystore: Some(keystore_container.keystore()),
	// 			offchain_db: backend.offchain_storage(),
	// 			transaction_pool: Some(OffchainTransactionPoolFactory::new(
	// 				transaction_pool.clone(),
	// 			)),
	// 			network_provider: network.clone(),
	// 			is_validator: role.is_authority(),
	// 			enable_http_requests: true,
	// 			custom_extensions: move |_| {
	// 				vec![Box::new(statement_store.clone().as_statement_store_ext()) as Box<_>]
	// 			},
	// 		})
	// 		.run(client.clone(), task_manager.spawn_handle())
	// 		.boxed(),
	// 	);
	// }

	network_starter.start_network();
	Ok(NewFullBase {
		task_manager,
		client,
		network,
		sync: sync_service,
		transaction_pool,
		rpc_handlers,
	})
}

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, Executor>(config: Configuration) -> Result<TaskManager, ServiceError>
where
	RuntimeApi:
		ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	Executor: NativeExecutionDispatch + 'static,
{
	// let database_path = config.database.path().map(Path::to_path_buf);

	let task_manager = match config.network.network_backend {
		sc_network::config::NetworkBackendType::Libp2p => {
			let task_manager =
				new_full_base::<sc_network::NetworkWorker<_, _>, RuntimeApi, Executor>(config)
					.map(|NewFullBase { task_manager, .. }| task_manager)?;
			task_manager
		},
		sc_network::config::NetworkBackendType::Litep2p => {
			let task_manager =
				new_full_base::<sc_network::Litep2pNetworkBackend, RuntimeApi, Executor>(config)
					.map(|NewFullBase { task_manager, .. }| task_manager)?;
			task_manager
		},
	};

	Ok(task_manager)
}
