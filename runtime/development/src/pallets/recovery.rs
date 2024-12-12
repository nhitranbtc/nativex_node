/// The recovery pallet í an M-of-N social recovery tool for users to gain access to their
/// accounts if the private key or other authentication mechanisim is lost.
use crate::*;

parameter_types! {
	pub const ConfigDepositBase: Balance = 5 * NATIVEX;
	pub const FriendDepositFactor: Balance = 50 * NATIVEX;
	pub const MaxFriends: u16 = 9;
	pub const RecoveryDeposit: Balance = 5 * NATIVEX;
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_recovery::weights::SubstrateWeight<Runtime>;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}
