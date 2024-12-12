use crate::*;

parameter_types! {
	pub const MaxWellKnownNodes: u32 = 8;
	pub const MaxPeerIdLength: u32 = 128;
}

impl pallet_node_authorization::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxWellKnownNodes = MaxWellKnownNodes;
	type MaxPeerIdLength = MaxPeerIdLength;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}
