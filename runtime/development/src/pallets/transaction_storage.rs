/// Transaction storage pallet. Indexes transactions and manages storage proofs.
use crate::*;
impl pallet_transaction_storage::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeCall = RuntimeCall;
	type FeeDestination = ();
	type WeightInfo = pallet_transaction_storage::weights::SubstrateWeight<Runtime>;
	type MaxBlockTransactions =
		ConstU32<{ pallet_transaction_storage::DEFAULT_MAX_BLOCK_TRANSACTIONS }>;
	type MaxTransactionSize =
		ConstU32<{ pallet_transaction_storage::DEFAULT_MAX_TRANSACTION_SIZE }>;
}
