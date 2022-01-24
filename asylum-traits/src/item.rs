use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError, DispatchResult};

use crate::primitives::*;

/// Nft info.
#[derive(Encode, Decode, Default, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct ItemInfo<BoundedString> {
	pub metadata: BoundedString,
}

pub trait Item<AccountId, BoundedString> {
	fn item_mint(
		metadata: BoundedString,
	) -> Result<ItemId, DispatchError>;
	fn item_burn(item_id: ItemId) -> DispatchResult;
	fn item_resource_bind_to_game(
		item_id: ItemId,
		resource_id: ResourceId,
		game_id: GameId,
	) -> DispatchResult;
	fn item_resource_unbind_from_game(
		item_id: ItemId,
		game_id: GameId,
	) -> DispatchResult;
}
