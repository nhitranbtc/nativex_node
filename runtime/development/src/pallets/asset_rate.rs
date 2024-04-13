/// The AssetRate pallet provides means of setting conversion rates for some asset to native
/// balance.
use crate::*;

impl pallet_asset_rate::Config for Runtime {
	type CreateOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Balance = Balance;
	type Currency = Balances;
	type AssetId = u32;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_asset_rate::weights::SubstrateWeight<Runtime>;
}
