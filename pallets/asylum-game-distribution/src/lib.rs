#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

mod functions;
mod impl_nonfungibles;
mod types;

use asylum_traits::primitives::TemplateId;
use codec::{Decode, Encode, HasCompact};
use frame_support::traits::{tokens::{nonfungibles::Inspect as NFTInspect, fungibles::Inspect as FungibleInspect}, Currency, ExistenceRequirement};
use frame_system::Config as SystemConfig;
use sp_runtime::{
	traits::{Saturating, StaticLookup, Zero},
	ArithmeticError, RuntimeDebug,
};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;

pub use pallet::*;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub type AssetIdOf<T> = <T as Config>::AssetId;	

	pub type BoundedDataOf<T> = BoundedVec<u8, <T as Config>::DataLimit>;
	pub type BoundedKeyOf<T> = BoundedVec<u8, <T as Config>::KeyLimit>;
	pub type BoundedValueOf<T> = BoundedVec<u8, <T as Config>::ValueLimit>;
	pub type BoundedStringOf<T> = BoundedVec<u8, <T as Config>::StringLimit>;

	pub type BoundedDataOf<T> = BoundedVec<u8, <T as Config>::DataLimit>;
	pub type BoundedKeyOf<T> = BoundedVec<u8, <T as Config>::KeyLimit>;
	pub type BoundedValueOf<T> = BoundedVec<u8, <T as Config>::ValueLimit>;
	pub type BoundedStringOf<T> = BoundedVec<u8, <T as Config>::StringLimit>;

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Inspect pallet uniques to check if supported templates really exist
		type Uniques: NFTInspect<Self::AccountId, ClassId = TemplateId>;

		/// Additional data to be stored with an account's asset balance.
		type AssetId: Parameter
		+ Member
		+ MaybeSerializeDeserialize
		+ Ord
		+ MaxEncodedLen
		+ Copy;

		/// Inspect pallet assets to check if game's assets really exist
		type Assets: FungibleInspect<Self::AccountId, AssetId = Self::AssetId>;

		/// Identifier for the class of asset.
		type GameId: Member + Parameter + Default + Copy + HasCompact;

		/// The type used to identify a unique asset within an asset class.
		type TicketId: Member + Parameter + Default + Copy + HasCompact + From<u16>;

		type Currency: Currency<Self::AccountId>;

		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type DataLimit: Get<u32>;

		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;

		/// The maximum length of an attribute key.
		#[pallet::constant]
		type KeyLimit: Get<u32>;

		/// The maximum length of an attribute value.
		#[pallet::constant]
		type ValueLimit: Get<u32>;
	}

	#[pallet::storage]
	/// Details of an asset class.
	pub(super) type Game<T: Config> = StorageMap<_, Blake2_128Concat, T::GameId, GameDetailsFor<T>>;

	#[pallet::storage]
	/// The assets held by any given account; set out this way so that assets owned by a single
	/// account can be enumerated.
	pub(super) type Account<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // owner
			NMapKey<Blake2_128Concat, T::GameId>,
			NMapKey<Blake2_128Concat, T::TicketId>,
		),
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	/// The classes owned by any given account; set out this way so that classes owned by a single
	/// account can be enumerated.
	pub(super) type GameAccount<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::GameId,
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	/// The assets in existence and their ownership details.
	pub(super) type Ticket<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::GameId,
		Blake2_128Concat,
		T::TicketId,
		TicketDetailsFor<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Metadata of an asset class.
	pub(super) type GameMetadataOf<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::GameId,
		GameMetadata<BoundedDataOf<T>, BoundedStringOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Metadata of an asset instance.
	pub(super) type TicketMetadataOf<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::GameId,
		Blake2_128Concat,
		T::TicketId,
		TicketMetadata<BoundedDataOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Metadata of an asset class.
	pub(super) type Attribute<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::GameId>,
			NMapKey<Blake2_128Concat, Option<T::TicketId>>,
			NMapKey<Blake2_128Concat, BoundedKeyOf<T>>,
		),
		BoundedValueOf<T>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GameCreated {
			game: T::GameId,
			creator: T::AccountId,
			owner: T::AccountId,
		},
		GameDestroyed {
			game: T::GameId,
		},
		TicketIssued {
			game: T::GameId,
			ticket: T::TicketId,
			owner: T::AccountId,
		},
		TicketTransferred {
			game: T::GameId,
			ticket: T::TicketId,
			from: T::AccountId,
			to: T::AccountId,
		},
		TicketBurned {
			game: T::GameId,
			ticket: T::TicketId,
			owner: T::AccountId,
		},
		TicketFrozen {
			game: T::GameId,
			ticket: T::TicketId,
		},
		TicketThawed {
			game: T::GameId,
			ticket: T::TicketId,
		},
		GameFrozen {
			game: T::GameId,
		},
		GameThawed {
			game: T::GameId,
		},
		OwnerChanged {
			game: T::GameId,
			new_owner: T::AccountId,
		},
		TeamChanged {
			game: T::GameId,
			issuer: T::AccountId,
			admin: T::AccountId,
			freezer: T::AccountId,
		},
		ApprovedTransfer {
			game: T::GameId,
			ticket: T::TicketId,
			owner: T::AccountId,
			delegate: T::AccountId,
		},
		ApprovalCancelled {
			game: T::GameId,
			ticket: T::TicketId,
			owner: T::AccountId,
			delegate: T::AccountId,
		},
		GameMetadataSet {
			game: T::GameId,
			data: BoundedDataOf<T>,
			title: BoundedStringOf<T>,
			genre: BoundedStringOf<T>,
		},
		GameMetadataCleared {
			game: T::GameId,
		},
		TicketMetadataSet {
			game: T::GameId,
			ticket: T::TicketId,
			data: BoundedDataOf<T>,
		},
		TicketMetadataCleared {
			game: T::GameId,
			ticket: T::TicketId,
		},
		AttributeSet {
			game: T::GameId,
			maybe_ticket: Option<T::TicketId>,
			key: BoundedKeyOf<T>,
			value: BoundedValueOf<T>,
		},
		AttributeCleared {
			game: T::GameId,
			maybe_ticket: Option<T::TicketId>,
			key: BoundedKeyOf<T>,
		},
		GameAddTemplateSupport {
			game: T::GameId,
			template_id: TemplateId,
		},
		GameRemoveTemplateSupport {
			game: T::GameId,
			template_id: TemplateId,
		},
		AllowUnprivilegedMint {
			game: T::GameId,
			allow: bool,
		},
		SetPrice {
			game: T::GameId,
			price: BalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The signing account has no permission to do the operation.
		NoPermission,
		/// The given asset ID is unknown.
		Unknown,
		/// The asset instance ID has already been used for an asset.
		AlreadyExists,
		/// The owner turned out to be different to what was expected.
		WrongOwner,
		/// Invalid witness data given.
		BadWitness,
		/// The asset ID is already taken.
		InUse,
		/// The asset instance or class is frozen.
		Frozen,
		/// The delegate turned out to be different to what was expected.
		WrongDelegate,
		/// There is no delegate approved.
		NoDelegate,
		/// No approval exists that would allow the transfer.
		Unapproved,
	}

	impl<T: Config> Pallet<T> {
		/// Get the owner of the asset instance, if the asset exists.
		pub fn owner(class: T::GameId, instance: T::TicketId) -> Option<T::AccountId> {
			Ticket::<T>::get(class, instance).map(|i| i.owner)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_game(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			admin: <T::Lookup as StaticLookup>::Source,
			price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let admin = T::Lookup::lookup(admin)?;

			Self::do_create_game(
				game,
				owner.clone(),
				admin.clone(),
				price,
				Event::GameCreated { game, creator: owner, owner: admin },
			)
		}

		#[pallet::weight(10_000)]
		pub fn destroy_game(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			witness: DestroyWitness,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;
			let _details = Self::do_destroy_game(game, witness, Some(check_owner))?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn mint_ticket(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			owner: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let owner = T::Lookup::lookup(owner)?;
			let game_details = Game::<T>::get(game).ok_or(Error::<T>::Unknown)?;
			ensure!(
				game_details.allow_unprivileged_mint || game_details.issuer == sender,
				Error::<T>::NoPermission
			);
			if let Some(price) = game_details.price {
				T::Currency::transfer(
					&sender,
					&game_details.owner,
					price,
					ExistenceRequirement::KeepAlive,
				)?;
			}

			Self::do_mint_ticket(game, ticket, owner, |_| Ok(()))
		}

		#[pallet::weight(10_000)]
		pub fn burn_ticket(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			check_owner: Option<<T::Lookup as StaticLookup>::Source>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let check_owner = check_owner.map(T::Lookup::lookup).transpose()?;

			Self::do_burn_ticket(game, ticket, |class_details, details| {
				let is_permitted = class_details.admin == origin || details.owner == origin;
				ensure!(is_permitted, Error::<T>::NoPermission);
				ensure!(check_owner.map_or(true, |o| o == details.owner), Error::<T>::WrongOwner);
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let dest = T::Lookup::lookup(dest)?;

			Self::do_transfer(game, ticket, dest, |class_details, details| {
				if details.owner != origin && class_details.admin != origin {
					let approved = details.approved.take().map_or(false, |i| i == origin);
					ensure!(approved, Error::<T>::NoPermission);
				}
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn freeze_ticket(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			let mut details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;
			let game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(game_details.freezer == origin, Error::<T>::NoPermission);

			details.is_frozen = true;
			Ticket::<T>::insert(&game, &ticket, &details);

			Self::deposit_event(Event::<T>::TicketFrozen { game, ticket });
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn thaw_ticket(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			let mut details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;
			let game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(game_details.admin == origin, Error::<T>::NoPermission);

			details.is_frozen = false;
			Ticket::<T>::insert(&game, &ticket, &details);

			Self::deposit_event(Event::<T>::TicketThawed { game, ticket });
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn freeze_game(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.freezer, Error::<T>::NoPermission);

				details.is_frozen = true;

				Self::deposit_event(Event::<T>::GameFrozen { game });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn thaw_game(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.admin, Error::<T>::NoPermission);

				details.is_frozen = false;

				Self::deposit_event(Event::<T>::GameThawed { game });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn transfer_game_ownership(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			owner: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let owner = T::Lookup::lookup(owner)?;

			Game::<T>::try_mutate(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);

				if details.owner == owner {
					return Ok(())
				}

				GameAccount::<T>::remove(&details.owner, &game);
				GameAccount::<T>::insert(&owner, &game, ());
				details.owner = owner.clone();

				Self::deposit_event(Event::OwnerChanged { game, new_owner: owner });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn set_game_team(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			issuer: <T::Lookup as StaticLookup>::Source,
			admin: <T::Lookup as StaticLookup>::Source,
			freezer: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let issuer = T::Lookup::lookup(issuer)?;
			let admin = T::Lookup::lookup(admin)?;
			let freezer = T::Lookup::lookup(freezer)?;

			Game::<T>::try_mutate(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);

				details.issuer = issuer.clone();
				details.admin = admin.clone();
				details.freezer = freezer.clone();

				Self::deposit_event(Event::TeamChanged { game, issuer, admin, freezer });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn approve_transfer(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			delegate: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let check = ensure_signed(origin)?;
			let delegate = T::Lookup::lookup(delegate)?;

			let game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			let mut details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;

			let permitted = check == game_details.admin || check == details.owner;
			ensure!(permitted, Error::<T>::NoPermission);

			details.approved = Some(delegate);
			Ticket::<T>::insert(&game, &ticket, &details);

			let delegate = details.approved.expect("set as Some above; qed");
			Self::deposit_event(Event::ApprovedTransfer {
				game,
				ticket,
				owner: details.owner,
				delegate,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn cancel_approval(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			maybe_check_delegate: Option<<T::Lookup as StaticLookup>::Source>,
		) -> DispatchResult {
			let check = ensure_signed(origin)?;
			let game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			let mut details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;

			let permitted = check == game_details.admin || check == details.owner;
			ensure!(permitted, Error::<T>::NoPermission);

			let maybe_check_delegate = maybe_check_delegate.map(T::Lookup::lookup).transpose()?;
			let old = details.approved.take().ok_or(Error::<T>::NoDelegate)?;
			if let Some(check_delegate) = maybe_check_delegate {
				ensure!(check_delegate == old, Error::<T>::WrongDelegate);
			}

			Ticket::<T>::insert(&game, &ticket, &details);
			Self::deposit_event(Event::ApprovalCancelled {
				game,
				ticket,
				owner: details.owner,
				delegate: old,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_attribute(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			maybe_ticket: Option<T::TicketId>,
			key: BoundedKeyOf<T>,
			value: BoundedValueOf<T>,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let mut game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(check_owner == game_details.owner, Error::<T>::NoPermission);

			let attribute = Attribute::<T>::get((game, maybe_ticket, &key));
			if attribute.is_none() {
				game_details.attributes.saturating_inc();
			}

			Attribute::<T>::insert((&game, maybe_ticket, &key), &value);
			Game::<T>::insert(game, &game_details);
			Self::deposit_event(Event::AttributeSet { game, maybe_ticket, key, value });
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn clear_attribute(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			maybe_ticket: Option<T::TicketId>,
			key: BoundedKeyOf<T>,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let mut game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(check_owner == game_details.owner, Error::<T>::NoPermission);

			if Attribute::<T>::take((game, maybe_ticket, &key)).is_some() {
				game_details.attributes.saturating_dec();
				Game::<T>::insert(game, &game_details);
				Self::deposit_event(Event::AttributeCleared { game, maybe_ticket, key });
			}
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_ticket_metadata(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
			data: BoundedDataOf<T>,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let mut game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;

			ensure!(check_owner == game_details.owner, Error::<T>::NoPermission);

			TicketMetadataOf::<T>::try_mutate_exists(game, ticket, |metadata| {
				if metadata.is_none() {
					game_details.instance_metadatas.saturating_inc();
				}

				*metadata = Some(TicketMetadata { data: data.clone() });

				Game::<T>::insert(&game, &game_details);
				Self::deposit_event(Event::TicketMetadataSet { game, ticket, data });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn clear_ticket_metadata(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] ticket: T::TicketId,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let mut game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(check_owner == game_details.owner, Error::<T>::NoPermission);

			TicketMetadataOf::<T>::try_mutate_exists(game, ticket, |metadata| {
				if metadata.is_some() {
					game_details.instance_metadatas.saturating_dec();
				}
				metadata.take();
				Game::<T>::insert(&game, &game_details);
				Self::deposit_event(Event::TicketMetadataCleared { game, ticket });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn set_game_metadata(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			data: BoundedDataOf<T>,
			title: BoundedStringOf<T>,
			genre: BoundedStringOf<T>,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(check_owner == details.owner, Error::<T>::NoPermission);

			GameMetadataOf::<T>::try_mutate_exists(game, |metadata| {
				Game::<T>::insert(&game, details);

				*metadata = Some(GameMetadata {
					data: data.clone(),
					title: title.clone(),
					genre: genre.clone(),
				});

				Self::deposit_event(Event::GameMetadataSet { game, data, title, genre });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn clear_game_metadata(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
		) -> DispatchResult {
			let check_owner = ensure_signed(origin)?;

			let details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
			ensure!(check_owner == details.owner, Error::<T>::NoPermission);

			GameMetadataOf::<T>::try_mutate_exists(game, |metadata| {
				metadata.take();
				Self::deposit_event(Event::GameMetadataCleared { game });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn set_allow_unpriviledged_mint(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			allow: bool,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);

				details.allow_unprivileged_mint = allow;
				Self::deposit_event(Event::AllowUnprivilegedMint { game, allow });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn set_price(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.admin, Error::<T>::NoPermission);

				details.price = Some(price);
				Self::deposit_event(Event::SetPrice { game, price });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn add_template_support(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] template_id: TemplateId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);
				ensure!(T::Uniques::class_owner(&template_id).is_some(), Error::<T>::Unknown);
				if let Some(templates) = &mut details.templates {
					templates.insert(template_id);
				} else {
					details.templates = Some(BTreeSet::from([template_id]));
				}
				
				Self::deposit_event(Event::GameAddTemplateSupport { game, template_id });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn remove_template_support(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			#[pallet::compact] template_id: TemplateId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);
				if let Some(templates) = &mut details.templates {
					templates.remove(&template_id);
				}

				Self::deposit_event(Event::GameRemoveTemplateSupport { game, template_id });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn add_asset_support(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			asset_id: T::AssetId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);
				ensure!(T::Assets::total_issuance(asset_id).is_zero(), Error::<T>::Unknown);
				if let Some(assets) = &mut details.assets {
					assets.insert(asset_id);
				} else {
					details.assets = Some(BTreeSet::from([asset_id]));
				}
				
				//Self::deposit_event(Event::GameAddTemplateSupport { game, template_id });
				Ok(())
			})
		}

		#[pallet::weight(10_000)]
		pub fn remove_asset_support(
			origin: OriginFor<T>,
			#[pallet::compact] game: T::GameId,
			asset_id: T::AssetId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Game::<T>::try_mutate_exists(game, |maybe_details| {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;
				ensure!(origin == details.owner, Error::<T>::NoPermission);
				if let Some(assets) = &mut details.assets {
					assets.remove(&asset_id);
				}
				//Self::deposit_event(Event::GameRemoveTemplateSupport { game, template_id });
				Ok(())
			})
		}
	}
}
