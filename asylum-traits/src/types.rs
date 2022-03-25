use codec::{Decode, Encode};
use rmrk_traits::ResourceInfo;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

use crate::primitives::*;

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct IntepretationTypeInfo<BoundedString> {
	// ipfs hash
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub struct Interpretation<BoundedName, BoundedInterpretation, BoundedString> {
	pub type_name: BoundedName,
	pub interpretations: Vec<ResourceInfo<BoundedInterpretation, BoundedString>>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub enum Change<BoundedResource, BoundedString> {
	AddOrUpdate { interpretation_type: InterpretationTypeId, interpretations: Vec<ResourceInfo<BoundedResource, BoundedString>> },
	RemoveInterpretation { interpretation_type: InterpretationTypeId, interpretation_id: BoundedResource },
	RemoveInterpretationType { interpretation_type: InterpretationTypeId },
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub enum ProposalState {
	Pending,
	Approved,
	Rejected,
}

impl Default for ProposalState {
	fn default() -> Self {
		ProposalState::Approved
	}
}

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct ProposalInfo<AccountId, BoundedResource, BoundedString>
where
	AccountId: Encode + Decode,
{
	pub author: AccountId,
	pub state: ProposalState,
	pub template_id: ItemTemplateId,
	pub change_set: Vec<Change<BoundedResource, BoundedString>>,
}
