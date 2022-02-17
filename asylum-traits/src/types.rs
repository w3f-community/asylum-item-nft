use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

use crate::primitives::*;

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone, Copy)]
pub struct IntepretationInfo<BoundedString> {
	pub name: BoundedString,
	// media
	pub src: BoundedString,
	//
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct IntepretationTypeInfo<BoundedString> {
	// ipfs hash
	pub metadata: BoundedString,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub struct Interpretation<BoundedName> {
	pub type_name: BoundedName,
	pub interpretation_ids: Vec<InterpretationId>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub enum Change {
	Add { interpretation_type: InterpretationTypeId, interpretation_ids: Vec<InterpretationId> },
	Update { interpretation_type: InterpretationTypeId, interpretation_ids: Vec<InterpretationId> },
	Remove { interpretation_type: InterpretationTypeId },
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
pub struct ProposalInfo<AccountId>
where
	AccountId: Encode + Decode,
{
	pub author: AccountId,
	pub state: ProposalState,
	pub template_id: ItemTemplateId,
	pub change_set: Vec<Change>,
}
