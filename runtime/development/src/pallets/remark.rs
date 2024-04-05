use crate::*;

impl pallet_remark::Config for Runtime {
	type WeightInfo = pallet_remark::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
}
