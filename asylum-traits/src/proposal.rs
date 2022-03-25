use crate::{
	primitives::{ItemTemplateId, ProposalId},
	Change,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub trait Proposal<AccountId, BoundedResource, BoundedString> {
	fn submit_proposal(
		author: AccountId,
		template_id: ItemTemplateId,
		change_set: Vec<Change<BoundedResource, BoundedString>>,
	) -> Result<ProposalId, DispatchError>;
}
