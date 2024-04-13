#![allow(unused_imports)]
use crate::*;
use frame_support::{match_types, traits::InstanceFilter};

parameter_types! {
	// One storage item; key size 32, value size 8;
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);

}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	Staking,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				RuntimeCall::Balances(..)
					// | RuntimeCall::Assets(..)
					// | RuntimeCall::Uniques(..)
					// | RuntimeCall::Nfts(..)
					// | RuntimeCall::Vesting(pallet_vesting::Call::vested_transfer { .. })
					| RuntimeCall::Indices(pallet_indices::Call::transfer { .. })
			),
			ProxyType::Governance => matches!(
				c,
				//RuntimeCall::Demoracy(..)
				|RuntimeCall::Council(..) /* | RuntimeCall::Society(..)
				                           * | RuntimeCall::TechnicalCommittee(..)
				                           *	| RuntimeCall::Elections(..) */| RuntimeCall::Treasury(
					..
				)
			),
			ProxyType::Staking => {
				matches!(
					c,
					RuntimeCall::Staking(..) // | RuntimeCall::FastUnstake(..)
				)
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => true,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}
