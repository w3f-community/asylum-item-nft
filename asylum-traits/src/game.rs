use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{DispatchResultWithInfo, RuntimeDebug};
use sp_std::collections::btree_set::BTreeSet;

use crate::primitives::*;

#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct GameInfo {
	pub game_id: GameId,
	pub supported_items: BTreeSet<ItemId>,
}

pub trait Game {
	fn game_supports_item(item_id: ItemId, game_id: GameId) -> DispatchResultWithInfo<bool>;
	fn game_item_bind_to_game(item_id: ItemId, game_id: GameId) -> DispatchResultWithInfo<()>;
	fn game_item_unbind_from_game(item_id: ItemId, game_id: GameId) -> DispatchResultWithInfo<()>;
}
