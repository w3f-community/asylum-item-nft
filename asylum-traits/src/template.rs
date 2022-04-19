use crate::{
	primitives::{ProposalId, TemplateId},
	Interpretation,
};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

/// Trait for providing Template(NFT collection) functionality for Asylum
pub trait ItemTemplate<AccountId, BoundedString, BoundedInterpretationId, BoundedTag> {
	/// Create new item's Template
	///
	/// # Arguments
	///
	/// * `template_id` - Collection's id created by rmrk-core-pallet
	/// * `interpretations` - vec of pairs of (InterpretationTypeName, Interpretation)
	///
	/// # Return
	///
	/// Ok(id) of newly create template
	fn template_create(
		template_id: TemplateId,
		interpretations: Vec<Interpretation<BoundedInterpretationId, BoundedString, BoundedTag>>,
	) -> Result<TemplateId, DispatchError>;

	/// Update item's Template according to approved proposal
	///
	/// # Arguments
	///
	/// * `sender` - transaction sender
	/// * `proposal_id` - Template's update proposal id
	/// * `template_id` - Template's id
	fn template_update(
		sender: AccountId,
		proposal_id: ProposalId,
		template_id: TemplateId,
	) -> DispatchResult;

	/// Destroy empty template
	///
	/// # Arguments
	///
	/// * `template_id` - Template's id
	///
	/// # Return
	///
	/// Ok(id) of destroyed template
	fn template_destroy(template_id: TemplateId) -> Result<TemplateId, DispatchError>;
}
