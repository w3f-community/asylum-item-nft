use asylum_traits::game::Game;
use asylum_traits::primitives::{GameId, ItemId};
use asylum_traits::GameInfo;
use sp_runtime::{DispatchError, DispatchResult};

use super::*;

impl<T: Config> Game<MetadataLimitOf<T>> for Pallet<T> {
	fn game_create(metadata: MetadataLimitOf<T>) -> Result<GameId, DispatchError> {
		let game_id = Self::get_next_game_id()?;
		let game_info = GameInfo { metadata };
		Games::<T>::insert(game_id, game_info);
		Ok(game_id)
	}

	fn game_burn(game_id: GameId) -> Result<GameId, DispatchError> {
		// What to do with free game ID?
		Games::<T>::remove(game_id);
		Ok(game_id)
	}

	fn game_supports_item(item_id: ItemId, game_id: GameId) -> Result<bool, DispatchError> {
		Ok(Self::items(game_id).contains(&item_id))
	}

	fn game_item_bind_to_game(item_id: ItemId, game_id: GameId) -> DispatchResult {
		GameItems::<T>::try_mutate(game_id, |items| -> DispatchResult {
			items.insert(item_id);
			Ok(())
		})
	}

	fn game_item_unbind_from_game(item_id: ItemId, game_id: GameId) -> Result<(), DispatchError> {
		GameItems::<T>::try_mutate(game_id, |items| -> DispatchResult {
			items.remove(&item_id);
			Ok(())
		})
	}
}
