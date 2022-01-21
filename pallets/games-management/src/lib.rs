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
		Game, GameInfo,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::fungibles::{Create, Destroy, Inspect, InspectMetadata, Mutate, Transfer},
		transactional,
	};
	use frame_system::pallet_prelude::*;
	use sp_std::collections::btree_set::BTreeSet;

	pub type MetadataLimitOf<T> = BoundedVec<u8, <T as Config>::MetadataLimit>;
	type BalanceOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Assets: Create<Self::AccountId>
			+ Inspect<Self::AccountId, AssetId = GameId>
			+ InspectMetadata<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Transfer<Self::AccountId>
			+ Mutate<Self::AccountId>;

		#[pallet::constant]
		type MetadataLimit: Get<u32>;

		#[pallet::constant]
		type MinGamesAmount: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn game_index)]
	pub type NextGameId<T: Config> = StorageValue<_, GameId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items)]
	// NOTE: Is it a good idea to use Set(fast insert/remove/contains) here? Should we use BoundedSet?
	pub type GameItems<T: Config> =
		StorageMap<_, Twox64Concat, GameId, BTreeSet<ItemId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn games)]
	pub type Games<T: Config> =
		StorageMap<_, Twox64Concat, GameId, GameInfo<MetadataLimitOf<T>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GameCreated {
			game_id: GameId,
		},
		GameDestroyed {
			game_id: GameId,
		},
		GameMinted {
			who: T::AccountId,
			game_id: GameId,
			amount: BalanceOf<T>,
		},
		GameBurned {
			who: T::AccountId,
			game_id: GameId,
			amount: BalanceOf<T>,
		},
		GameTransfered {
			from: T::AccountId,
			to: T::AccountId,
			game_id: GameId,
			copy_amount: BalanceOf<T>,
		},
		BindItem {
			item_id: ItemId,
			game_id: GameId,
		},
		UnbindItem {
			item_id: ItemId,
			game_id: GameId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableGameId,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_game(
			origin: OriginFor<T>,
			admin: T::AccountId,
			metadata: BoundedVec<u8, T::MetadataLimit>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			let game_id = Self::game_create(metadata)?;
			T::Assets::create(game_id, admin, true, T::MinGamesAmount::get())?;

			Self::deposit_event(Event::GameCreated { game_id });
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn destroy_game(origin: OriginFor<T>, game_id: GameId) -> DispatchResult {
			ensure_signed(origin)?;

			Self::game_burn(game_id)?;

			let destroy_witness = T::Assets::get_destroy_witness(&game_id).unwrap();
			T::Assets::destroy(game_id, destroy_witness, None)?;

			Self::deposit_event(Event::GameDestroyed { game_id });
			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		pub fn get_next_game_id() -> Result<GameId, Error<T>> {
			// NOTE: Should we have a more sophisticated game ID generation algorithm?
			NextGameId::<T>::try_mutate(|id| {
				let current_id = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableGameId)?;
				Ok(current_id)
			})
		}
	}
}
