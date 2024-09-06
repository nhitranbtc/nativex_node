use crate::{AccountId, Balance, Block, Index};
use common_primitives::BlockNumber;
use sp_runtime::{
	generic::SignedBlock,
	traits::{BlakeTwo256, Block as BlockT, NumberFor},
	Justifications,
};
use std::sync::Arc;

use sp_blockchain::{self as blockchain};

#[derive(Clone)]
pub enum Client {
	#[cfg(feature = "with-development-runtime")]
	Development(
		Arc<crate::FullClient<development_runtime::RuntimeApi, crate::DevelopmentExecutor>>,
	),
}

#[cfg(feature = "with-development-runtime")]
impl From<Arc<crate::FullClient<development_runtime::RuntimeApi, crate::DevelopmentExecutor>>>
	for Client
{
	fn from(
		client: Arc<crate::FullClient<development_runtime::RuntimeApi, crate::DevelopmentExecutor>>,
	) -> Self {
		Self::Development(client)
	}
}
use crate::match_client;

impl sc_client_api::BlockBackend<Block> for Client {
	fn block_body(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
		match_client!(self, block_body(hash))
	}
	fn block_indexed_body(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {
		match_client!(self, block_indexed_body(hash))
	}
	fn block(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
		match_client!(self, block(hash))
	}
	fn block_status(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<sp_consensus::BlockStatus> {
		match_client!(self, block_status(hash))
	}
	fn justifications(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Justifications>> {
		match_client!(self, justifications(hash))
	}
	fn block_hash(
		&self,
		number: NumberFor<Block>,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, block_hash(number))
	}
	fn indexed_transaction(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<u8>>> {
		match_client!(self, indexed_transaction(hash))
	}
	fn has_indexed_transaction(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<bool> {
		match_client!(self, has_indexed_transaction(hash))
	}
	fn requires_full_sync(&self) -> bool {
		todo!()
	}
}

impl sp_blockchain::HeaderBackend<Block> for Client {
	fn header(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Header>> {
		//let id = &hash;
		match_client!(self, header(hash))
	}
	fn info(&self) -> sp_blockchain::Info<Block> {
		match_client!(self, info())
	}
	fn status(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
		match_client!(self, status(hash))
	}
	fn number(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> sp_blockchain::Result<
		Option<<<Block as BlockT>::Header as sp_runtime::traits::Header>::Number>,
	> {
		match_client!(self, number(hash))
	}
	fn hash(
		&self,
		number: NumberFor<Block>,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, hash(number))
	}
}

impl sp_blockchain::HeaderMetadata<Block> for Client {
	type Error = sp_blockchain::Error;
	fn header_metadata(
		&self,
		hash: <Block as BlockT>::Hash,
	) -> Result<sp_blockchain::CachedHeaderMetadata<Block>, Self::Error> {
		match_client!(self, header_metadata(hash))
	}
	fn insert_header_metadata(
		&self,
		hash: <Block as BlockT>::Hash,
		header_metadata: sp_blockchain::CachedHeaderMetadata<Block>,
	) {
		match_client!(self, insert_header_metadata(hash, header_metadata))
	}
	fn remove_header_metadata(&self, hash: <Block as BlockT>::Hash) {
		match_client!(self, remove_header_metadata(hash))
	}
}

impl sc_client_api::backend::AuxStore for Client {
	fn get_aux(&self, key: &[u8]) -> sp_blockchain::Result<Option<Vec<u8>>> {
		match_client!(self, get_aux(key))
	}
	fn insert_aux<
		'a,
		'b: 'a,
		'c: 'a,
		I: IntoIterator<Item = &'a (&'c [u8], &'c [u8])>,
		D: IntoIterator<Item = &'a &'b [u8]>,
	>(
		&self,
		insert: I,
		delete: D,
	) -> sp_blockchain::Result<()> {
		match_client!(self, insert_aux(insert, delete))
	}
}

impl sc_client_api::backend::StorageProvider<Block, crate::FullBackend> for Client {
	fn storage(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &sc_client_api::StorageKey,
	) -> sp_blockchain::Result<Option<sc_client_api::StorageData>> {
		match_client!(self, storage(hash, key))
	}
	fn storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		prefix: Option<&sc_client_api::StorageKey>,
		start_key: Option<&sc_client_api::StorageKey>,
	) -> sp_blockchain::Result<
		sc_client_api::KeysIter<
			<crate::FullBackend as sc_client_api::Backend<Block>>::State,
			Block,
		>,
	> {
		match_client!(self, storage_keys(hash, prefix, start_key))
	}
	fn storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &sc_client_api::StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, storage_hash(hash, key))
	}
	fn storage_pairs(
		&self,
		hash: <Block as BlockT>::Hash,
		prefix: Option<&sc_client_api::StorageKey>,
		start_key: Option<&sc_client_api::StorageKey>,
	) -> sp_blockchain::Result<
		sc_client_api::PairsIter<
			<crate::FullBackend as sc_client_api::Backend<Block>>::State,
			Block,
		>,
	> {
		match_client!(self, storage_pairs(hash, prefix, start_key))
	}
	fn child_storage(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &sc_client_api::ChildInfo,
		key: &sc_client_api::StorageKey,
	) -> sp_blockchain::Result<Option<sc_client_api::StorageData>> {
		match_client!(self, child_storage(hash, child_info, key))
	}
	fn child_storage_hash(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &sc_client_api::ChildInfo,
		key: &sc_client_api::StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, child_storage_hash(hash, child_info, key))
	}

	fn child_storage_keys(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: sc_client_api::ChildInfo,
		prefix: Option<&sc_client_api::StorageKey>,
		start_key: Option<&sc_client_api::StorageKey>,
	) -> sp_blockchain::Result<
		sc_client_api::KeysIter<
			<crate::FullBackend as sc_client_api::Backend<Block>>::State,
			Block,
		>,
	> {
		match_client!(self, child_storage_keys(hash, child_info, prefix, start_key))
	}

	fn closest_merkle_value(
		&self,
		hash: <Block as BlockT>::Hash,
		key: &sc_client_api::StorageKey,
	) -> blockchain::Result<Option<sp_trie::MerkleValue<<Block as BlockT>::Hash>>> {
		match_client!(self, closest_merkle_value(hash, key))
	}

	fn child_closest_merkle_value(
		&self,
		hash: <Block as BlockT>::Hash,
		child_info: &sc_client_api::ChildInfo,
		key: &sc_client_api::StorageKey,
	) -> blockchain::Result<Option<sp_trie::MerkleValue<<Block as BlockT>::Hash>>> {
		match_client!(self, child_closest_merkle_value(hash, child_info, key))
	}
}

impl sc_client_api::UsageProvider<Block> for Client {
	fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {
		match_client!(self, usage_info())
	}
}
#[macro_export]
macro_rules! match_client {
	($self:ident, $method:ident($($param:ident),*)) => {
		match $self {
			#[cfg(feature = "with-development-runtime")]
			Self::Development(client) => client.$method($($param),*),
			_ => todo!()
		}
	};
}

/// A set of APIs that bholdus-like runtimes must implement.
///
/// This trait has no methods or associated type. It is a concise marker for all the trait bounds
/// that it contains.
pub trait RuntimeApiCollection:
	sp_api::ApiExt<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
	+ sp_api::Metadata<Block>
	+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_consensus_babe::BabeApi<Block>
	+ grandpa_primitives::GrandpaApi<Block>
	//+ beefy_primitives::BeefyApi<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
	+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
	//+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	//+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
	//+ pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>
	//+ mmr_rpc::MmrRuntimeApi<Block, <Block as sp_runtime::traits::Block>::Hash, BlockNumber>
	//+ fp_rpc::EthereumRuntimeRPCApi<Block>
	//+ fp_rpc::ConvertTransactionRuntimeApi<Block>
// where
// 	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
// 	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashingFor<Block>>,
{
}

impl<Api> RuntimeApiCollection for Api where
	Api: sp_api::ApiExt<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
		+ sp_api::Metadata<Block>
		+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_consensus_babe::BabeApi<Block>
		+ grandpa_primitives::GrandpaApi<Block>
		//+ beefy_primitives::BeefyApi<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance> /* B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	                                                                                 * B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashingFor<Block>>, */
{
}
