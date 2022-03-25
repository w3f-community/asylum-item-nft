use crate::{
	primitives::{ItemTemplateId, ProposalId},
	Interpretation,
};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

/// Trait for providing Template(NFT collection) functionality for Asylum
pub trait ItemTemplate<AccountId, BoundedString, BoundedInterpretation> {
	///	Create new item's Template
	///
	/// # Arguments
	///
	/// * `template_name` - Template's name
	/// * `metadata` - A bounded string that hold ifsh hash to metadata
	/// * `interpretations` - vec of pair of (InterpretationTypeName, InterpretationName)
	///
	/// # Return
	///
	/// Ok(id) of newly create template
	fn template_create(
		template_id: ItemTemplateId,
		interpretations: Vec<Interpretation<BoundedString, BoundedInterpretation, BoundedString>>,
	) -> Result<ItemTemplateId, DispatchError>;

	///	Update item's Template according to approved proposal
	///
	/// # Arguments
	///
	/// * `proposal_id` - Template's update proposal id
	/// * `template_name_or_id` - Template's id or name
	fn template_update(proposal_id: ProposalId, template_id: ItemTemplateId) -> DispatchResult;

	///	Destroy empty template
	///
	/// # Arguments
	///
	/// * `template_name` - Template's name
	/// * `metadata` - A bounded string that hold ifsh hash to metadata
	/// * `interpretations` - vec of pair of (InterpretationTypeName, InterpretationName)
	///
	/// # Return
	///
	/// Ok(id) of destroyed template
	fn template_destroy(template_id: ItemTemplateId) -> Result<ItemTemplateId, DispatchError>;
}
