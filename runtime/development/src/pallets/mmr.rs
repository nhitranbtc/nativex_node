use crate::*;
impl pallet_mmr::Config for Runtime {
	const INDEXING_PREFIX: &'static [u8] = b"mmr";
	type Hashing = <Runtime as frame_system::Config>::Hashing;
	type LeafData = pallet_mmr::ParentNumberAndHash<Self>;
	type OnNewRoot = ();
	type BlockHashProvider = pallet_mmr::DefaultBlockHashProvider<Runtime>;
	type WeightInfo = ();
}
