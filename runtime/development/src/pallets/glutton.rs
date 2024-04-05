/// DO NOT USE ON VALUE-BEARING CHAINS. THIS PALLET IS ONLY INTENDED FOR TESTING USAGE.
use crate::*;

impl pallet_glutton::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_glutton::weights::SubstrateWeight<Runtime>;
}
