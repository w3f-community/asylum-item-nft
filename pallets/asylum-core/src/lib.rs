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
		primitives::{GameId, ItemId},
		Game, GameInfo, Item, ItemInfo,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::nonfungibles::{Create, Destroy, Inspect, Mutate, Transfer},
		transactional,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};

	pub type MetadataLimitOf<T> = BoundedVec<u8, <T as Config>::MetadataLimit>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type ItemNFT: Inspect<Self::AccountId, ClassId = u32, InstanceId = ItemId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Transfer<Self::AccountId>;

		type GameNFT: Inspect<Self::AccountId, ClassId = u32, InstanceId = GameId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Transfer<Self::AccountId>;

		#[pallet::constant]
		type MetadataLimit: Get<u32>;

		#[pallet::constant]
		type ItemsClassId: Get<u32>;

		#[pallet::constant]
		type GamesClassId: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn item_index)]
	pub type NextItemId<T: Config> = StorageValue<_, ItemId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn game_index)]
	pub type NextGameId<T: Config> = StorageValue<_, GameId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items)]
	pub type Items<T: Config> =
		StorageMap<_, Twox64Concat, ItemId, ItemInfo<MetadataLimitOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn games)]
	pub type Games<T: Config> =
		StorageMap<_, Twox64Concat, GameId, GameInfo<MetadataLimitOf<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ItemMinted { item_id: ItemId, recipient: T::AccountId },
		ItemBurned { item_id: ItemId },
		ItemTransfered { item_id: ItemId, destination: T::AccountId },
		ItemMetadataSet{item_id: ItemId},
		ItemMetadataCleared{item_id: ItemId},
		GameMinted { game_id: GameId, recipient: T::AccountId },
		GameBurned { game_id: GameId },
		GameTransfered { game_id: GameId, destination: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableItemId,
		NoAvailableGameId,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn initialize_items_collection(
			origin: OriginFor<T>,
			who: T::AccountId,
			admin: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			T::ItemNFT::create_class(&T::ItemsClassId::get(), &who, &admin)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn initialize_games_collection(
			origin: OriginFor<T>,
			who: T::AccountId,
			admin: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;

			T::GameNFT::create_class(&T::GamesClassId::get(), &who, &admin)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint_item(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			metadata: Option<BoundedVec<u8, T::MetadataLimit>>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			let item_id = Self::item_mint(metadata)?;
			T::ItemNFT::mint_into(&T::ItemsClassId::get(), &item_id, &recipient)?;

			Self::deposit_event(Event::ItemMinted { item_id, recipient });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_item(origin: OriginFor<T>, item_id: ItemId) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			Self::item_burn(item_id)?;
			T::ItemNFT::burn_from(&T::ItemsClassId::get(), &item_id)?;

			Self::deposit_event(Event::ItemBurned { item_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_item(
			origin: OriginFor<T>,
			item_id: ItemId,
			destination: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			T::ItemNFT::transfer(&T::ItemsClassId::get(), &item_id, &destination)?;

			Self::deposit_event(Event::ItemTransfered { item_id, destination });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn set_metadata_item(
			origin: OriginFor<T>,
			item_id: ItemId,
			metadata: BoundedVec<u8, T::MetadataLimit>
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			Self::item_set_metadata(item_id, metadata)?;

			Self::deposit_event(Event::ItemMetadataSet { item_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn clear_metadata_item(
			origin: OriginFor<T>,
			item_id: ItemId
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			Self::item_clear_metadata(item_id)?;

			Self::deposit_event(Event::ItemMetadataCleared { item_id });
			Ok(())
		}


		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint_game(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			metadata: BoundedVec<u8, T::MetadataLimit>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			let game_id = Self::game_mint(metadata)?;
			T::GameNFT::mint_into(&T::GamesClassId::get(), &game_id, &recipient)?;

			Self::deposit_event(Event::GameMinted { game_id, recipient });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_game(origin: OriginFor<T>, game_id: GameId) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			Self::game_burn(game_id)?;
			T::GameNFT::burn_from(&T::GamesClassId::get(), &game_id)?;

			Self::deposit_event(Event::GameBurned { game_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_game(
			origin: OriginFor<T>,
			game_id: GameId,
			destination: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: add permission checks if needed

			T::GameNFT::transfer(&T::GamesClassId::get(), &game_id, &destination)?;

			Self::deposit_event(Event::GameTransfered { game_id, destination });
			Ok(())
		}
	}
}
