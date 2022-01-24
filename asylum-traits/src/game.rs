use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError, DispatchResult};

use crate::primitives::*;

#[derive(Default, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct GameInfo<BoundedString> {
	pub metadata: BoundedString,
}

pub trait Game<BoundedString> {
	fn game_create(metadata: BoundedString) -> Result<GameId, DispatchError>;
	fn game_burn(game_id: GameId) -> Result<GameId, DispatchError>;
	fn game_supports_item(item_id: ItemId, game_id: GameId) -> Result<bool, DispatchError>;
	fn game_item_bind_to_game(item_id: ItemId, game_id: GameId) -> DispatchResult;
	fn game_item_unbind_from_game(item_id: ItemId, game_id: GameId) -> DispatchResult;
}