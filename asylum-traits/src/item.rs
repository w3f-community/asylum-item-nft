use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::result::Result;

use crate::primitives::*;

/// Nft info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct ItemInfo<AccountId, BoundedString> {
	pub owner: AccountId,
	/// Arbitrary data about an instance, e.g. IPFS hash
	pub metadata: BoundedString,
}

pub trait Item<AccountId, BoundedString> {
	fn item_mint(
		sender: AccountId,
		owner: AccountId,
		metadata: BoundedString,
	) -> Result<ItemId, DispatchError>;
	fn item_burn(item_id: ItemId) -> Result<ItemId, DispatchError>;
	fn item_send(
		sender: AccountId,
		item_id: ItemId,
		new_owner: AccountId,
	) -> Result<AccountId, DispatchError>;
	fn item_resource_bind_to_game(
		item_id: ItemId,
		resource_id: ResourceId,
		game_id: GameId,
	) -> Result<bool, DispatchError>;
	fn item_resource_unbind_from_game(
		item_id: ItemId,
		resource_id: ResourceId,
		game_id: GameId,
	) -> Result<bool, DispatchError>;
}
