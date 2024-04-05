/// Reads and writes all keys and values in the entire state in a systematic way.
/// This is useful for upgrading a chain to [sp-core::StateVersion::V1], where all keys need to be touched.
use crate::*;

parameter_types! {
	pub const MigrationSignedDepositPerItem: Balance = 1 * CENTS;
	pub const MigrationSignedDepositBase: Balance = 1 * NATIVEX;
	pub const MigrationMaxKeyLen: u32 = 512;
}
impl pallet_state_trie_migration::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ControlOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type MaxKeyLen = MigrationMaxKeyLen;
	type SignedDepositPerItem = MigrationSignedDepositPerItem;
	type SignedDepositBase = MigrationSignedDepositBase;
	// Warning: this is not advised, as it might allow the chain to be temporary DOS-ed.
	// Preferably, if the chain's governance/maintenance team is planning on using a specific
	// account for the migration, put it here to make sure only that account can trigger the signed migrations.
	type SignedFilter = EnsureSigned<Self::AccountId>;
	type WeightInfo = ();
}
