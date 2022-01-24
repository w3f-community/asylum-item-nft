use asylum_traits::primitives::{GameId, ItemId, ResourceId};
use asylum_traits::{Item, ItemInfo, Resource, ResourceInfo};
use sp_runtime::{DispatchError, DispatchResult};

use super::*;

impl<T: Config> Item<T::AccountId, MetadataLimitOf<T>> for Pallet<T> {
	fn item_mint(metadata: MetadataLimitOf<T>) -> Result<ItemId, DispatchError> {
		let item_id = Self::get_next_item_id()?;
		let item_info = ItemInfo { metadata };
		Items::<T>::insert(item_id, item_info);
		Ok(item_id)
	}

	fn item_burn(item_id: ItemId) -> DispatchResult {
		Items::<T>::remove(item_id);
		Ok(())
	}

	fn item_resource_bind_to_game(
		item_id: ItemId,
		resource_id: ResourceId,
		game_id: GameId,
	) -> DispatchResult {
		GameResources::<T>::insert(game_id, item_id, resource_id);
		Ok(())
	}

	fn item_resource_unbind_from_game(item_id: ItemId, game_id: GameId) -> DispatchResult {
		GameResources::<T>::remove(game_id, item_id);
		Ok(())
	}
}

impl<T: Config> Resource<T::AccountId, MetadataLimitOf<T>> for Pallet<T> {
	fn resource_add(
		item_id: ItemId,
		src: Option<MetadataLimitOf<T>>,
		metadata: Option<MetadataLimitOf<T>>,
	) -> Result<ResourceId, DispatchError> {
		let resource_id = Self::get_next_resource_id()?;
		let resource_info = ResourceInfo { src, metadata };
		Resources::<T>::insert(item_id, resource_id, resource_info);
		Ok(resource_id)
	}

	fn resource_remove(item_id: ItemId, resource_id: ResourceId) -> DispatchResult {
		Resources::<T>::remove(item_id, resource_id);
		Ok(())
	}
}
