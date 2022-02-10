use crate::{primitives::*, Interpretation, NameOrId};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

/// Trait for providing Template(NFT collection) functionality for Asylum
pub trait ItemTemplate<AccountId, BoundedName, BoundedString> {
	///	Create new item's Template
	///
	/// # Arguments
	///
	/// * `owner` - Template's owner
	/// * `template_name` - Template's name
	/// * `metadata` - A bounded string that hold ifsh hash to metadata
	/// * `interpretations` - vec of pair of (InterpretationTypeName, InterpretationName)
	///
	/// # Return
	///
	/// Ok(id) of newly create template
	fn template_create(
		owner: AccountId,
		template_name: &BoundedName,
		metadata: BoundedString,
		interpretations: Vec<Interpretation<BoundedName>>,
	) -> Result<ItemTemplateId, DispatchError>;

	///	Update item's Template according to approved proposal
	///
	/// # Arguments
	///
	/// * `proposal_id` - Template's update proposal id
	/// * `template_name_or_id` - Template's id or name
	///
	fn template_update(
		proposal_id: ProposalId,
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
	) -> DispatchResult;

	///	Create new item's Template
	///
	/// # Arguments
	///
	/// * `template_name_or_id` - Template's id or name
	/// * `new_issuer` - Template's new issuer
	///
	fn template_change_issuer(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		new_issuer: AccountId,
	) -> DispatchResult;

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
	fn template_destroy(template_name: &BoundedName) -> Result<ItemTemplateId, DispatchError>;
}
