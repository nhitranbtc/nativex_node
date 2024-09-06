use crate::chain_spec::{authority_keys_from_seed, get_account_id_from_seed};
use common_primitives::{AccountId, Balance};
use development_runtime::{
	wasm_binary_unwrap, Block, MaxNominations, RuntimeGenesisConfig, SessionKeys, Signature,
	StakerStatus, BABE_GENESIS_EPOCH_CONFIG, NATIVEX, TOKEN_DECIMALS, TOKEN_SYMBOL, WASM_BINARY,
};
use grandpa_primitives::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{ChainType, Properties};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
//use sp_consensus_beefy::crypto::AuthorityId as BeefyId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

const DEFAULT_PROTOCOL_ID: &str = "nativex";

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 10_000_000 * NATIVEX;
const STASH: Balance = ENDOWMENT / 1000;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

pub fn get_properties() -> Properties {
	let mut properties = Properties::new();
	//properties.insert("ss58Format".into(), development_runtime::SS58Prefix::get().into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

fn configure_accounts(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	endowed_accounts: Option<Vec<AccountId>>,
	stash: Balance,
) -> (
	Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
	Vec<AccountId>,
	usize,
	Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), stash, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), stash, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	(initial_authorities, endowed_accounts, num_endowed_accounts, stakers)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		//BeefyId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> serde_json::Value {
	let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) =
		configure_accounts(initial_authorities, initial_nominators, endowed_accounts, STASH);

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect::<Vec<_>>(),
		},
		"session": {
			"keys": initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),

						),
					)
				})
				.collect::<Vec<_>>(),
		},

		"staking": {
			"validatorCount": initial_authorities.len() as u32,
			"minimumValidatorCount": initial_authorities.len() as u32,
			"invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
			"slashRewardFraction": Perbill::from_percent(10),
			"stakers": stakers.clone(),
		},
		"elections": {
			"members": endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect::<Vec<_>>(),
		},

		"technicalCommittee": {
			"members": endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect::<Vec<_>>(),
		},
		"sudo": { "key": Some(root_key.clone()) },
		"babe": {
			"epochConfig": Some(BABE_GENESIS_EPOCH_CONFIG),
		},

		"society": { "pot": 0 },
		"assets": {
			// This asset is used by the NIS pallet as counterpart currency.
			"assets": vec![(9, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
		},
		"nominationPools": {
			"minCreateBond": 10 * NATIVEX,
			"minJoinBond": 1 * NATIVEX,
		},
	})
}

fn development_config_genesis_json() -> serde_json::Value {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		Some(vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob/stash"),
		]),
	)
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
	ChainSpec::builder(wasm_binary_unwrap(), Default::default())
		.with_name("Development")
		.with_id("dev")
		.with_properties(get_properties())
		.with_chain_type(ChainType::Development)
		.with_genesis_config_patch(development_config_genesis_json())
		.build()
}
