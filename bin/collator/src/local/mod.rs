use common_primitives::{AccountId, Signature};
use grandpa_primitives::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
//use sp_consensus_beefy::crypto::AuthorityId as BeefyId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Local development service.
pub mod service;

/// Development chain specs.
pub mod chain_spec;

pub use chain_spec::*;


