use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

use crate::primitives::*;

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct IntepretationTypeInfo<BoundedString> {
	// ipfs hash
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub struct IntepretationInfo<BoundedInterpretationId, BoundedString> {
	pub id: BoundedInterpretationId,
	pub src: Option<BoundedString>,
	pub metadata: Option<BoundedString>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub struct Interpretation<BoundedName, BoundedInterpretation, BoundedString> {
	pub type_name: BoundedName,
	pub interpretations: Vec<IntepretationInfo<BoundedInterpretation, BoundedString>>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub enum Change<BoundedResource, BoundedString> {
	Add {
		interpretation_type: InterpretationTypeId,
		interpretations: Vec<IntepretationInfo<BoundedResource, BoundedString>>,
	},
	Modify {
		interpretation_type: InterpretationTypeId,
		interpretations: Vec<IntepretationInfo<BoundedResource, BoundedString>>,
	},
	RemoveInterpretation {
		interpretation_type: InterpretationTypeId,
		interpretation_id: BoundedResource,
	},
	RemoveInterpretationType {
		interpretation_type: InterpretationTypeId,
	},
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
