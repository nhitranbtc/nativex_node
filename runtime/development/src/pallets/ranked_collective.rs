use crate::*;

impl pallet_ranked_collective::Config for Runtime {
	type WeightInfo = pallet_ranked_collective::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type DemoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
	type Polls = RankedPolls;
	type MinRankOfClass = traits::Identity;
	type VoteWeight = pallet_ranked_collective::Geometric;
}
