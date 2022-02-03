use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError, DispatchResult};

use crate::primitives::*;

/// Nft info.
#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct ItemInfo<BoundedString> {
	pub metadata: Option<BoundedString>,
}

pub trait Item<AccountId, BoundedString> {
	fn item_mint(
		recipient: AccountId, 
		metadata: Option<BoundedString>,
	) -> Result<ItemId, DispatchError>;
	fn item_burn(item_id: ItemId) -> DispatchResult;
	fn item_transfer(destination: AccountId, item_id: ItemId) -> DispatchResult;
}

pub trait ItemMetadata<BoundedString> {
	fn item_set_metadata(item_id: ItemId, metadata: BoundedString) -> DispatchResult;
	fn item_clear_metadata(item_id: ItemId) -> DispatchResult;
}

pub trait ItemAttributes<BoundedKey, BoundedMetadata>{
	fn item_set_attribute(item_id: ItemId, key: &BoundedKey, metadata: &BoundedMetadata) -> DispatchResult;
	fn item_clear_attribute(item_id: ItemId, key: &BoundedKey) -> DispatchResult;
}
