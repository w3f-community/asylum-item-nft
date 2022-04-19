use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

use crate::primitives::*;

#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct TagInfo<BoundedString> {
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
pub struct Interpretation<BoundedInterpretationId, BoundedString, BoundedTag> {
	pub tags: BTreeSet<BoundedTag>,
	pub interpretation: IntepretationInfo<BoundedInterpretationId, BoundedString>,
}

#[derive(Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq, Clone)]
pub enum Change<BoundedInterpretationId, BoundedString, BoundedTag> {
	Add {
		interpretations:
			Vec<(IntepretationInfo<BoundedInterpretationId, BoundedString>, BTreeSet<BoundedTag>)>,
	},
	Modify {
		interpretations: Vec<IntepretationInfo<BoundedInterpretationId, BoundedString>>,
	},
	ModifyTags {
		interpretation_id: BoundedInterpretationId,
		tags: BTreeSet<BoundedTag>,
	},
	RemoveInterpretation {
		interpretation_id: BoundedInterpretationId,
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
pub struct ProposalInfo<AccountId, BoundedInterpretationId, BoundedString, BoundedTag>
where
	AccountId: Encode + Decode,
{
	pub author: AccountId,
	pub state: ProposalState,
	pub template_id: TemplateId,
	pub change_set: Vec<Change<BoundedInterpretationId, BoundedString, BoundedTag>>,
}
