use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{RuntimeDebug, DispatchError};

use crate::primitives::*;

#[derive(Default, Encode, Decode, RuntimeDebug, TypeInfo, PartialEq, Eq)]
pub struct GameInfo<BoundedString> {
	pub metadata: BoundedString,
}

pub trait Game<AccountId, BoundedString> {
	fn game_mint(metadata: BoundedString) -> Result<GameId, DispatchError>;
	fn game_burn(game_id: GameId) -> Result<GameId, DispatchError>;
}