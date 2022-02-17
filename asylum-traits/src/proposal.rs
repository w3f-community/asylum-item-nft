use crate::{
	primitives::{ItemTemplateId, ProposalId},
	Change,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub trait Proposal<AccountId> {
	fn submit_proposal(
		author: AccountId,
		template_id: ItemTemplateId,
		change_set: Vec<Change>,
	) -> Result<ProposalId, DispatchError>;
}
