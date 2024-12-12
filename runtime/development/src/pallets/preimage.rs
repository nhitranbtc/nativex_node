/// The preimage pallet allows for the users and the runtime to store the preimage of hash on
/// chain. This can be used by other pallets for storing and managing large byte-blobs.
use crate::*;

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub const PreimageBaseDeposit: Balance = 1 * NATIVEX;
	// One cent: $10,000/ MB.
	pub const PreimageByteDeposit: Balance = 1 * NATIVEX;
}

parameter_types! {
	pub const PreimageHoldReason: RuntimeHoldReason = RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = pallet_preimage::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
	>;
}
