/// A pallet allow participants of the staking system to unstake quicker,
/// if and only if they meet the condition of not being exposed to any slashes.
use crate::{
	AccountId, Balances, ConstU128, ConstU32, MaxNominatorRewardedPerValidator, Runtime,
	RuntimeEvent, Staking, NATIVEX,
};

impl pallet_fast_unstake::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ControlOrigin = frame_system::EnsureRoot<AccountId>;
	type BatchSize = ConstU32<64>;
	type Deposit = ConstU128<{ NATIVEX }>;
	type Currency = Balances;
	type Staking = Staking;
	type MaxErasToCheckPerBlock = ConstU32<1>;
	#[cfg(feature = "runtime-benchmarks")]
	type MaxBackersPerValidator = MaxNominatorRewardedPerValidator;
	type WeightInfo = ();
}
