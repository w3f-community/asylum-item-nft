use asylum_traits::primitives::{GameId, ItemId};
use asylum_traits::{Game, GameInfo, Item, ItemInfo};
use sp_runtime::{DispatchError, DispatchResult};

use super::*;

impl<T: Config> Item<T::AccountId, MetadataLimitOf<T>> for Pallet<T> {
	fn item_mint(metadata: Option<MetadataLimitOf<T>>) -> Result<ItemId, DispatchError> {
		let item_id = Self::get_next_item_id()?;
		let item_info = ItemInfo { metadata };
		Items::<T>::insert(item_id, item_info);
		Ok(item_id)
	}

	fn item_burn(item_id: ItemId) -> DispatchResult {
		Items::<T>::remove(item_id);
		Ok(())
	}

	fn item_set_metadata(item_id: ItemId, metadata: MetadataLimitOf<T>) -> DispatchResult {
		Items::<T>::try_mutate(item_id, |info| -> DispatchResult {
			info.metadata = Some(metadata);
			Ok(())
		})
	}

	fn item_clear_metadata(item_id: ItemId) -> DispatchResult {
		Items::<T>::try_mutate(item_id, |info| -> DispatchResult {
			info.metadata = None;
			Ok(())
		})
	}
}

impl<T: Config> Game<T::AccountId, MetadataLimitOf<T>> for Pallet<T> {
	fn game_mint(metadata: MetadataLimitOf<T>) -> Result<GameId, DispatchError> {
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
}

impl<T: Config> Pallet<T> {
	pub fn get_next_item_id() -> Result<ItemId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextItemId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableItemId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_game_id() -> Result<GameId, Error<T>> {
		// NOTE: Should we have a more sophisticated game ID generation algorithm?
		NextGameId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableGameId)?;
			Ok(current_id)
		})
	}
}
