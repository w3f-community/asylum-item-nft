use asylum_traits::primitives::{GameId, ItemId};

use super::*;

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