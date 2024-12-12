/// This pallet allows runtimes that include it to pay for transactions in assets other than
/// the main token of the chain.
use crate::{Assets, Balances, ConvertInto, CreditToBlockAuthor, Instance1, Runtime, RuntimeEvent};

impl pallet_asset_tx_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Fungibles = Assets;
	type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
		pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto, Instance1>,
		CreditToBlockAuthor,
	>;
}
