#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod implementation;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use asylum_traits::{
		primitives::{GameId, ItemId, ResourceId},
		Item, ItemInfo, Resource, ResourceInfo,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::nonfungibles::{Create, Destroy, Inspect, Mutate},
		transactional,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};

	const MOCK_COLLECTION_ID: u32 = 0;

	pub type MetadataLimitOf<T> = BoundedVec<u8, <T as Config>::MetadataLimit>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Nft: Inspect<Self::AccountId, ClassId = u32, InstanceId = ItemId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Mutate<Self::AccountId>;

		#[pallet::constant]
		type MetadataLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn item_index)]
	pub type NextItemId<T: Config> = StorageValue<_, ItemId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn resource_index)]
	pub type NextResourceId<T: Config> = StorageValue<_, ResourceId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items)]
	pub type Items<T: Config> =
		StorageMap<_, Twox64Concat, ItemId, ItemInfo<MetadataLimitOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn resources)]
	pub type Resources<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		ItemId,
		Twox64Concat,
		ResourceId,
		ResourceInfo<MetadataLimitOf<T>>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn game_resources)]
	// TODO: Add CONTEXT key to differentiate resource ids binded to one game
	pub type GameResources<T: Config> =
		StorageDoubleMap<_, Twox64Concat, GameId, Twox64Concat, ItemId, ResourceId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ItemMinted { item_id: ItemId, recipient: T::AccountId },
		ItemBurned { item_id: ItemId },
		ResourceAdded { item_id: ItemId, resource_id: ResourceId },
		ResourceRemoved { item_id: ItemId, resource_id: ResourceId },
		ResourceBinded { item_id: ItemId, resource_id: ResourceId, game_id: GameId },
		ResourceUnbinded { item_id: ItemId, game_id: GameId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableItemId,
		NoAvailableResourceId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn initialize_mock_collection(
			origin: OriginFor<T>,
			admin: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			T::Nft::create_class(&MOCK_COLLECTION_ID, &admin, &admin)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint_item(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			metadata: BoundedVec<u8, T::MetadataLimit>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let item_id = Self::item_mint(metadata)?;
			T::Nft::mint_into(&MOCK_COLLECTION_ID, &item_id, &recipient)?;

			Self::deposit_event(Event::ItemMinted { item_id, recipient });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_item(origin: OriginFor<T>, item_id: ItemId) -> DispatchResult {
			ensure_signed(origin)?;

			Self::item_burn(item_id)?;
			T::Nft::burn_from(&MOCK_COLLECTION_ID, &item_id)?;

			Self::deposit_event(Event::ItemBurned { item_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn add_resource(
			origin: OriginFor<T>,
			item_id: ItemId,
			src: Option<MetadataLimitOf<T>>,
			metadata: Option<MetadataLimitOf<T>>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let resource_id = Self::resource_add(item_id, src, metadata)?;

			Self::deposit_event(Event::ResourceAdded { item_id, resource_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn remove_resource(
			origin: OriginFor<T>,
			item_id: ItemId,
			resource_id: ResourceId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			Self::resource_remove(item_id, resource_id)?;

			Self::deposit_event(Event::ResourceRemoved { item_id, resource_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn bind_resource(
			origin: OriginFor<T>,
			item_id: ItemId,
			resource_id: ResourceId,
			game_id: GameId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			Self::item_resource_bind_to_game(item_id, resource_id, game_id)?;

			Self::deposit_event(Event::ResourceBinded { item_id, resource_id, game_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn unbind_resource(
			origin: OriginFor<T>,
			item_id: ItemId,
			game_id: GameId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			Self::item_resource_unbind_from_game(item_id, game_id)?;

			Self::deposit_event(Event::ResourceUnbinded { item_id, game_id });
			Ok(())
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

		pub fn get_next_resource_id() -> Result<ResourceId, Error<T>> {
			// NOTE: Should we have a more sophisticated resource ID generation algorithm?
			NextResourceId::<T>::try_mutate(|id| {
				let current_id = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableResourceId)?;
				Ok(current_id)
			})
		}
	}
}
