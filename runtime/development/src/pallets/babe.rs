use crate::*;

parameter_types! {
// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
//       Attempting to do so will brick block production.
pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
pub const ReportLongevity: u64 =
	BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
   }

impl pallet_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	type DisabledValidators = Session;
	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
	type MaxNominators = MaxNominatorRewardedPerValidator;
	type KeyOwnerProof =
		<Historical as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::Proof;
	type EquivocationReportSystem =
		pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}
