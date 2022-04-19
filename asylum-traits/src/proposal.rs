use crate::{
	primitives::{ProposalId, TemplateId},
	Change,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub trait Proposal<AccountId, BoundedInterpretationId, BoundedString, BoundedTag> {
	fn submit_proposal(
		author: AccountId,
		template_id: TemplateId,
		change_set: Vec<Change<BoundedInterpretationId, BoundedString, BoundedTag>>,
	) -> Result<ProposalId, DispatchError>;
}
