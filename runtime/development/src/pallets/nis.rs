use pallet_nis::WithMaximumOf;

/// A pallet allowing accounts to auction for being frozen and receive open-ended inflation-protection in return.
use crate::*;

parameter_types! {
	pub const QueueCount: u32 = 300;
	pub const MaxQueueLen: u32 = 1000;
	pub const FifoQueueLen: u32 = 500;
	pub const NisBasePeriod: BlockNumber = 30 * DAYS;
	pub const MinBid: Balance = 100 * NATIVEX;
	pub const MinReceipt: Perquintill = Perquintill::from_percent(1);
	pub const IntakePeriod: BlockNumber = 10;
	pub MaxIntakeWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 10;
	pub const ThawThrottle: (Perquintill, BlockNumber) = (Perquintill::from_percent(25), 5);
	pub Target: Perquintill = Perquintill::zero();
	pub const NisPalletId: PalletId = PalletId(*b"py/nis  ");
	pub const NisHoldReason: HoldReason = HoldReason::Nis;
}

impl pallet_nis::Config for Runtime {
	type WeightInfo = pallet_nis::weights::SubstrateWeight<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type FundOrigin = frame_system::EnsureSigned<AccountId>;
	type Counterpart = ItemOf<Assets, ConstU32<9u32>, AccountId>;
	type CounterpartAmount = WithMaximumOf<ConstU128<21_000_000_000_000_000_000u128>>;
	type Deficit = ();
	type IgnoredIssuance = ();
	type Target = Target;
	type PalletId = NisPalletId;
	type QueueCount = QueueCount;
	type MaxQueueLen = MaxQueueLen;
	type FifoQueueLen = FifoQueueLen;
	type BasePeriod = NisBasePeriod;
	type MinBid = MinBid;
	type MinReceipt = MinReceipt;
	type IntakePeriod = IntakePeriod;
	type MaxIntakeWeight = MaxIntakeWeight;
	type ThawThrottle = ThawThrottle;
	type HoldReason = NisHoldReason;
}
