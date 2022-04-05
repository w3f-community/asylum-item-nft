use crate::{
	primitives::{ItemTemplateId, ProposalId},
	Interpretation,
};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

/// Trait for providing Template(NFT collection) functionality for Asylum
pub trait ItemTemplate<AccountId, BoundedString, BoundedInterpretation> {
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
		template_id: ItemTemplateId,
		interpretations: Vec<Interpretation<BoundedString, BoundedInterpretation, BoundedString>>,
	) -> Result<ItemTemplateId, DispatchError>;

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
		template_id: ItemTemplateId,
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
	fn template_destroy(template_id: ItemTemplateId) -> Result<ItemTemplateId, DispatchError>;
}
