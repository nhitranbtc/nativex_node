/// An index is a short form of an address. This module handles allocation of indices for a
/// newly created accounts.
use crate::*;

parameter_types! {
	pub const IndexDeposit: Balance = 1 * NATIVEX;
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_indices::weights::SubstrateWeight<Runtime>;
}
