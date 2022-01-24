use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError, DispatchResult};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Default, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceInfo<BoundedString> {
	/// IPFS hash
	pub src: Option<BoundedString>,
	pub metadata: Option<BoundedString>,
}

pub trait Resource<AccountId, BoundedString> {
	fn resource_add(
		item_id: ItemId,
		src: Option<BoundedString>,
		metadata: Option<BoundedString>,
	) -> Result<ResourceId, DispatchError>;
	fn resource_remove(
		item_id: ItemId,
		resource_id: ResourceId,
	) -> DispatchResult;
}
