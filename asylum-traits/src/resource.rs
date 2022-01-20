use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::cmp::Eq;

use crate::primitives::*;
use serde::{Deserialize, Serialize};
use sp_std::result::Result;

#[derive(Encode, Decode, Eq, Copy, PartialEq, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceInfo<BoundedString> {
	/// IPFS hash
	pub src: Option<BoundedString>,
	pub metadata: Option<BoundedString>,
}

pub trait Resource<BoundedString, AccountId> {
	fn resource_add(
		sender: AccountId,
		item_id: ItemId,
		src: Option<BoundedString>,
		metadata: Option<BoundedString>,
	) -> Result<ResourceId, DispatchError>;
	fn resource_remove(
		sender: AccountId,
		item_id: ItemId,
		src: Option<BoundedString>,
		metadata: Option<BoundedString>,
	) -> Result<ResourceId, DispatchError>;
}
