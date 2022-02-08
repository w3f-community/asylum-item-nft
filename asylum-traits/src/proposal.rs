use crate::{
	primitives::{ItemTemplateId, ProposalId},
	Change, NameOrId,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub trait Proposal<AccountId, BoundedName> {
	fn submit_proposal(
		author: AccountId,
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		change_set: Vec<Change>,
	) -> Result<ProposalId, DispatchError>;
}
