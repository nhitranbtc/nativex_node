use crate::{
	AccountId, AllianceMotion, Assets, Authorship, Balances, Hash, NegativeImbalance, Runtime,
	RuntimeCall,
};
use pallet_asset_tx_payment::HandleCredit;

use frame_support::{
	pallet_prelude::*,
	traits::{
		fungibles::{Balanced, Credit},
		Currency, OnUnbalanced,
	},
};
use pallet_alliance::{IdentityVerifier, ProposalIndex, ProposalProvider};
use pallet_society::Judgement;
use pallet_treasury::Proposal;
use sp_std::prelude::*;

pub struct Author;
impl OnUnbalanced<NegativeImbalance> for Author {
	fn on_nonzero_unbalanced(amount: NegativeImbalance) {
		if let Some(author) = Authorship::author() {
			Balances::resolve_creating(&author, amount);
		}
	}
}

/// A `HandleCredit` implementation that naively transfers the fees to the block author.
/// Will drop and burn the assets in case the transfer fails.
pub struct CreditToBlockAuthor;
impl HandleCredit<AccountId, Assets> for CreditToBlockAuthor {
	fn handle_credit(credit: Credit<AccountId, Assets>) {
		if let Some(author) = pallet_authorship::Pallet::<Runtime>::author() {
			// Drop the result which will trigger the `OnDrop` of the imbalance in case of error.
			let _ = Assets::resolve(&author, credit);
		}
	}
}

pub struct AllianceIdentityVerifier;
impl IdentityVerifier<AccountId> for AllianceIdentityVerifier {
	fn has_identity(who: &AccountId, fields: u64) -> bool {
		crate::Identity::has_identity(who, fields)
	}
	fn has_good_judgement(who: &AccountId) -> bool {
		use pallet_identity::Judgement;
		crate::Identity::identity(who)
			.map(|registration| registration.judgements)
			.map_or(false, |judgements| {
				judgements
					.iter()
					.any(|(_, j)| matches!(j, Judgement::KnownGood | Judgement::Reasonable))
			})
	}

	fn super_account_id(who: &AccountId) -> Option<AccountId> {
		crate::Identity::super_of(who).map(|parent| parent.0)
	}
}

pub struct AllianceProposalProvider;
impl ProposalProvider<AccountId, Hash, RuntimeCall> for AllianceProposalProvider {
	fn propose_proposal(
		who: AccountId,
		threshold: u32,
		proposal: Box<RuntimeCall>,
		length_bound: u32,
	) -> Result<(u32, u32), DispatchError> {
		AllianceMotion::do_propose_proposed(who, threshold, proposal, length_bound)
	}
	fn vote_proposal(
		who: AccountId,
		proposal: Hash,
		index: ProposalIndex,
		approve: bool,
	) -> Result<bool, DispatchError> {
		AllianceMotion::do_vote(who, proposal, index, approve)
	}
	fn close_proposal(
		proposal_hash: Hash,
		proposal_index: ProposalIndex,
		proposal_weight_bound: Weight,
		length_bound: u32,
	) -> DispatchResultWithPostInfo {
		AllianceMotion::do_close(proposal_hash, proposal_index, proposal_weight_bound, length_bound)
	}
	fn proposal_of(proposal_hash: Hash) -> Option<RuntimeCall> {
		AllianceMotion::proposal_of(proposal_hash)
	}
}
