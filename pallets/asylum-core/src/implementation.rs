use asylum_traits::primitives::{GameId, ItemId};
use asylum_traits::{Game, GameInfo, Item, ItemMetadata, ItemAttributes, ItemInfo};
use sp_runtime::{DispatchError, DispatchResult};
use frame_support::{pallet_prelude::Get, traits::tokens::nonfungibles::{Mutate, Transfer}};

use super::*;

impl<T: Config> Item<T::AccountId, MetadataLimitOf<T>> for Pallet<T> {
	fn item_mint(recipient: T::AccountId, metadata: Option<MetadataLimitOf<T>>) -> Result<ItemId, DispatchError> {
		let item_id = Self::get_next_item_id()?;
		let item_info = ItemInfo { metadata };
		Items::<T>::insert(item_id, item_info);
		T::ItemNFT::mint_into(&T::ItemsClassId::get(), &item_id, &recipient)?;
		Ok(item_id)
	}

	fn item_burn(item_id: ItemId) -> DispatchResult {
		Items::<T>::remove(item_id);
		T::ItemNFT::burn_from(&T::ItemsClassId::get(), &item_id)
	}

	fn item_transfer(destination: T::AccountId, item_id: ItemId) -> DispatchResult {
		T::ItemNFT::transfer(&T::ItemsClassId::get(), &item_id, &destination)
	}
}

impl<T: Config> ItemMetadata<MetadataLimitOf<T>> for Pallet<T> {
	fn item_set_metadata(item_id: ItemId, metadata: MetadataLimitOf<T>) -> DispatchResult {
		Items::<T>::try_mutate(item_id, |info| -> DispatchResult {
			*info = Some(ItemInfo { metadata: Some(metadata) });
			Ok(())
		})
	}

	fn item_clear_metadata(item_id: ItemId) -> DispatchResult {
		Items::<T>::remove(item_id);
		Ok(())
	}
}

impl<T: Config> ItemAttributes<KeyLimitOf<T>, MetadataLimitOf<T>> for Pallet<T> {
	fn item_set_attribute(item_id: ItemId, key: &KeyLimitOf<T>, metadata: &MetadataLimitOf<T>) -> DispatchResult {
		Attributes::<T>::insert(item_id, key,metadata);
		Ok(())
	}

	fn item_clear_attribute(item_id: ItemId, key: &KeyLimitOf<T>) -> DispatchResult {
		Attributes::<T>::remove(item_id, key);
		Ok(())
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