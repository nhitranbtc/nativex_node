use crate::*;

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	/// The voter bags-list is loosely kepts up to date, and the real source of truth for the score
	/// of each node is the staking pallet.
	type ScoreProvider = Staking;
	type BagThresholds = BagThreshold;
	type Score = VoteWeight;
	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
}
