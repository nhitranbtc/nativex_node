/// A simple, secure module for dealing with non-fungible item
use crate::*;
use pallet_nfts::PalletFeatures;
parameter_types! {
	pub const ItemDeposit: Balance = 1 * NATIVEX;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
	pub const MaxAttributesPerCall: u32 = 10;
}

impl pallet_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositBase;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = <Signature as traits::Verify>::Signer;
	type WeightInfo = pallet_nfts::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Locker = ();
}
