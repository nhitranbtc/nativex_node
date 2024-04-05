/// Additional logic for the Core Fellowship.
/// This determines salary, registers activity/passivity and handles promotion
/// and demotion periods.
use crate::*;
impl pallet_core_fellowship::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Members = RankedCollective;
	type Balance = Balance;
	type ParamsOrigin = frame_system::EnsureRoot<AccountId>;
	type InductOrigin = pallet_core_fellowship::EnsureInducted<Runtime, (), 1>;
	type ApproveOrigin = frame_system::EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type PromoteOrigin = frame_system::EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type EvidenceSize = ConstU32<16_384>;
}
