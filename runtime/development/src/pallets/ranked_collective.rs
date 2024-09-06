use crate::*;

impl pallet_ranked_collective::Config for Runtime {
	type WeightInfo = pallet_ranked_collective::weights::SubstrateWeight<Self>;
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = Self::DemoteOrigin;
	type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type DemoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type ExchangeOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type Polls = RankedPolls;
	type MinRankOfClass = traits::Identity;
	type VoteWeight = pallet_ranked_collective::Geometric;
	type MemberSwappedHandler = (CoreFellowship, Salary);
	type MaxMemberCount = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkSetup = (CoreFellowship, Salary);
}
