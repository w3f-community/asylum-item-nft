#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod functions;
mod implementation;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use asylum_traits::{
		primitives::{InterpretationId, InterpretationTypeId, ItemId, ItemTemplateId, ProposalId},
		Change, IntepretationInfo, IntepretationTypeInfo, Interpretable, Item, ItemInfo,
		ItemTemplate, ItemTemplateInfo, NameOrId, Proposal, ProposalInfo, Interpretation,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::nonfungibles::{Create, Destroy, Inspect, Mutate, Transfer},
		transactional,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_std::{vec::Vec, str::Bytes};

	pub type MetadataLimitOf<T> = BoundedVec<u8, <T as Config>::MetadataLimit>;
	pub type KeyLimitOf<T> = BoundedVec<u8, <T as Config>::KeyLimit>;
	pub type NameLimitOf<T> = BoundedVec<u8, <T as Config>::NameLimit>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type ItemNFT: Inspect<Self::AccountId, ClassId = ItemTemplateId, InstanceId = ItemId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Transfer<Self::AccountId>;

		#[pallet::constant]
		type MetadataLimit: Get<u32>;

		#[pallet::constant]
		type KeyLimit: Get<u32>;

		#[pallet::constant]
		type NameLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type NextTemplateId<T: Config> = StorageValue<_, ItemTemplateId, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextItemId<T: Config> = StorageValue<_, ItemId, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextInterpretationTypeId<T: Config> =
		StorageValue<_, InterpretationTypeId, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextInterpretationId<T: Config> = StorageValue<_, InterpretationId, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, ProposalId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn attributes)]
	pub(super) type Properties<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		ItemId,
		Twox64Concat,
		KeyLimitOf<T>,
		MetadataLimitOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Human-readable names of interpretation types
	pub(super) type IntepretationTypeNames<T: Config> =
		StorageMap<_, Blake2_128Concat, NameLimitOf<T>, InterpretationTypeId, OptionQuery>; //genesis config

	#[pallet::storage]
	/// Human-readable names of interpretations
	pub(super) type IntepretationNames<T: Config> =
		StorageMap<_, Blake2_128Concat, NameLimitOf<T>, InterpretationId, OptionQuery>;

	#[pallet::storage]
	/// Human-readable names of templates
	pub(super) type TemplateNames<T: Config> =
		StorageMap<_, Blake2_128Concat, NameLimitOf<T>, ItemTemplateId, OptionQuery>;

	#[pallet::storage]
	/// Interpretation type's infos
	pub(super) type IntepretationTypes<T: Config> = StorageMap<
		_,
		Twox64Concat,
		InterpretationTypeId,
		IntepretationTypeInfo<MetadataLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Interpretation's infos
	pub(super) type Intepretations<T: Config> = StorageMap<
		_,
		Twox64Concat,
		InterpretationId,
		IntepretationInfo<MetadataLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Template's infos
	pub(super) type Templates<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ItemTemplateId,
		ItemTemplateInfo<T::AccountId, MetadataLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Interpretations supported by Items
	pub(super) type ItemIntepretations<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, ItemTemplateId>,
			NMapKey<Twox64Concat, Option<ItemId>>,
			NMapKey<Twox64Concat, InterpretationTypeId>,
		),
		Vec<InterpretationId>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Interpretations supported by Template
	pub(super) type TemplateIntepretations<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		ItemTemplateId,
		Twox64Concat,
		InterpretationTypeId,
		Vec<InterpretationId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn items)]
	/// Item's infos
	pub(super) type Items<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		ItemTemplateId,
		Twox64Concat,
		ItemId,
		ItemInfo<MetadataLimitOf<T>>,
	>;

	#[pallet::storage]
	/// Proposal's infos
	pub(super) type Proposals<T: Config> =
		StorageMap<_, Twox64Concat, ProposalId, ProposalInfo<T::AccountId>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		InterpretationTypeCreated {
			type_name: NameLimitOf<T>,
			type_id: InterpretationTypeId,
		},
		InterpretationCreated {
			interpretation_name: NameLimitOf<T>,
			intepretation_id: InterpretationId,
		},
		TemplateCreated {
			template_name: NameLimitOf<T>,
			template_id: ItemTemplateId,
		},
		TemplateUpdated {
			template_name: NameLimitOf<T>,
		},
		TemplateIssuerChanged {
			template_id: ItemTemplateId,
			new_issuer: T::AccountId,
		},
		TemplateDestroyed {
			template_name: NameLimitOf<T>,
			template_id: ItemTemplateId,
		},
		ItemMinted {
			template_id: ItemTemplateId,
			item_id: ItemId,
			recipient: T::AccountId,
		},
		ItemBurned {
			template_id: ItemTemplateId,
			item_id: ItemId,
		},
		ItemTransfered {
			template_id: ItemTemplateId,
			item_id: ItemId,
			destination: T::AccountId,
		},
		ItemUpdated {
			template_id: ItemTemplateId,
			item_id: ItemId,
		},
		ItemAttributeSet {
			item_id: ItemId,
			key: KeyLimitOf<T>,
		},
		ItemAttributeCleared {
			item_id: ItemId,
			key: KeyLimitOf<T>,
		},
		ProposalSubmitted {
			proposal_id: ProposalId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		InterpretationTypeAlreadyExist,
		InterpretationTypeNotExist,
		InterpretationAlreadyExist,
		InterpretationNotExist,
		TemplateAlreadyExist,
		TemplateNotExist,
		TemplateNotEmpty,
		TemplateAlreadySupportThisType,
		TemplateNotSupportThisType,
		ProposalNotExist,
		ProposalNotApproved,
		ProposalInappropriateTemplate,
		NoAvailableId,
		NoPermission,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new interpretation type.
		///
		/// Origin must be Signed.
		///
		/// - `type_name`: The interpretation type to be created.
		/// - `metadata`: The link to the interpretation type description stored somewhere(for example ipfs).
		///
		/// Emits `InterpretationTypeCreated`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_interpretation_type(
			origin: OriginFor<T>,
			type_name: NameLimitOf<T>,
			metadata: MetadataLimitOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let type_id = Self::interpretation_type_create(&type_name, metadata)?;
			Self::deposit_event(Event::InterpretationTypeCreated { type_name, type_id });
			Ok(())
		}

		/// Create new interpretation.
		///
		/// Origin must be Signed.
		///
		/// - `interpretation_name`: The interpretation to be created.
		/// - `src`: The link to the media file stored somewhere(for example ipfs).
		/// - `metadata`: The link to the interpretation type description stored somewhere(for example ipfs).
		///
		/// Emits `InterpretationCreated`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_interpretation(
			origin: OriginFor<T>,
			interpretation_name: NameLimitOf<T>,
			src: MetadataLimitOf<T>,
			metadata: MetadataLimitOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let intepretation_id =
				Self::interpretation_create(&interpretation_name, src, metadata)?;
			Self::deposit_event(Event::InterpretationCreated {
				interpretation_name,
				intepretation_id,
			});
			Ok(())
		}

		/// Create new template. In Asylum context Template is extended Collection of NFTs, i.e. all Items minted from this Template (in this Collection)
		/// should have the same interpretations.
		///
		/// Origin must be Signed.
		///
		/// - `owner`: The owner of template.
		/// - `template_name`: The template to be created.
		/// - `interpretations`: vec of pairs of supported (interpretation_type_name, interpretation_name).
		/// - `metadata`: The link to the template description stored somewhere(for example ipfs).
		///
		/// Emits `TemplateCreated`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_template(
			origin: OriginFor<T>,
			owner: T::AccountId,
			template_name: NameLimitOf<T>,
			metadata: MetadataLimitOf<T>,
			interpretations: Vec<Interpretation<NameLimitOf<T>>>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let template_id =
				Self::template_create(owner.clone(), &template_name, metadata, interpretations)?;
			T::ItemNFT::create_class(&template_id, &owner, &owner)?;
			Self::deposit_event(Event::TemplateCreated { template_name, template_id });
			Ok(())
		}

		/// Destroy template. In Asylum context Template is extended Collection of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_name`: The template to be destroyed.
		///
		/// Emits `TemplateDestroyed`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn destroy_template(
			origin: OriginFor<T>,
			template_name: NameLimitOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let template_id = Self::template_destroy(&template_name)?;
			let witness = T::ItemNFT::get_destroy_witness(&template_id).unwrap();
			T::ItemNFT::destroy(template_id, witness, T::ItemNFT::class_owner(&template_id))?;
			Self::deposit_event(Event::TemplateDestroyed { template_name, template_id });
			Ok(())
		}

		/// Update template according to proposal. In Asylum context Template is extended Collection of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_name`: The template to be destroyed.
		///
		/// Emits `TemplateUpdated`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn update_template(
			origin: OriginFor<T>,
			template_name: NameLimitOf<T>,
			proposal_id: ProposalId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::template_update(proposal_id, NameOrId::Name(template_name.clone()))?;
			Self::deposit_event(Event::TemplateUpdated { template_name });
			Ok(())
		}

		/// Change template issuer. In Asylum context Template is extended Collection of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_name`: The template, which issuer is changing.
		/// - `new_issuer`: New issuer of the template.
		///
		/// Emits `TemplateIssuerChanged`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn change_template_issuer(
			origin: OriginFor<T>,
			template_name: NameLimitOf<T>,
			new_issuer: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::template_change_issuer(NameOrId::Name(template_name.clone()), new_issuer)?;
			Self::deposit_event(Event::TemplateUpdated { template_name });
			Ok(())
		}

		/// Mint new item from 'template_name_or_id', i.e. mint Item(NFT) with the same set of supported interpretations as 'template_name_or_id' has.
		///
		/// Origin must be Signed and sender must be Issuer of Template.
		///
		/// - `recipient`: The recipient of the item 'template_name_or_id' Template.
		/// - `template_name_or_id`: The template name or id.
		/// - `metadata`: The link to the item description stored somewhere(for example ipfs).
		///
		/// Emits `ItemMinted`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint_item(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			metadata: Option<BoundedVec<u8, T::MetadataLimit>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::get_template_id(template_name_or_id)?;
			ensure!(
				T::ItemNFT::class_owner(&template_id) == Some(sender),
				Error::<T>::NoPermission
			);
			let (_, item_id) = Self::item_mint(NameOrId::Id(template_id), metadata)?;
			T::ItemNFT::mint_into(&template_id, &item_id, &recipient)?;
			Self::deposit_event(Event::ItemMinted { template_id, item_id, recipient });
			Ok(())
		}

		/// Destroy a single asset instance.
		///
		/// Origin must be Signed and the sender should be the Admin of the asset `template_name_or_id`.
		///
		/// - `template_name_or_id`: The template name or id
		/// - `item_id`: The item to be burned
		///
		/// Emits `ItemBurned`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_item(
			origin: OriginFor<T>,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			item_id: ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::get_template_id(template_name_or_id)?;
			ensure!(
				T::ItemNFT::class_owner(&template_id) == Some(sender),
				Error::<T>::NoPermission
			);
			let (_, item_id) = Self::item_burn(NameOrId::Id(template_id), item_id)?;
			T::ItemNFT::burn_from(&template_id, &item_id)?;
			Self::deposit_event(Event::ItemBurned { template_id, item_id });
			Ok(())
		}

		/// Move an asset from the sender account to another.
		///
		/// Origin must be Signed and the signing account must be either:
		/// - the Admin of the asset `template_name_or_id`;
		/// - the Owner of the asset `item_id`;
		/// - the approved delegate for the asset `item_id` (in this case, the approval is reset).
		///
		/// Arguments:
		/// - `template_name_or_id`: The template of the item to be transferred.
		/// - `item_id`: The item to be transferred.
		/// - `destination`: The account to receive ownership of the asset.
		///
		/// Emits `ItemTransferred`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_item(
			origin: OriginFor<T>,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			item_id: ItemId,
			destination: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::get_template_id(template_name_or_id)?;
			ensure!(
				T::ItemNFT::owner(&template_id, &item_id) == Some(sender),
				Error::<T>::NoPermission
			);
			Self::item_transfer(&destination, NameOrId::Id(template_id), item_id)?;
			Self::deposit_event(Event::ItemTransfered { template_id, item_id, destination });
			Ok(())
		}

		/// Update 'item_id' item according to newest version of 'template_name_or_id' template
		///
		/// Origin must be Signed and the sender must be owner of the 'item_id' item
		///
		/// Arguments:
		/// - `template_name_or_id`: The template of the item to be updated.
		/// - `item_id`: The item to be updated.
		///
		/// Emits `ItemUpdated`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn update_item(
			origin: OriginFor<T>,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			item_id: ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::get_template_id(template_name_or_id)?;
			ensure!(
				T::ItemNFT::owner(&template_id, &item_id) == Some(sender),
				Error::<T>::NoPermission
			);
			Self::item_update(NameOrId::Id(template_id), item_id)?;
			Self::deposit_event(Event::ItemUpdated { template_id, item_id });
			Ok(())
		}

		/// Set an property for an item.
		///
		/// Origin must be signed by both `property_owner` and owner of item.
		///
		/// - `property_owner`: The owner of the property.
		/// - `template_name_or_id`: The template of the item whose metadata to set.
		/// - `item_id`: The identifier of the item whose metadata to set.
		/// - `key`: The value to which to set the attribute.
		/// - `metadata`: The value to which to set the attribute.
		///
		/// Emits `PropertySet`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn set_item_property(
			origin: OriginFor<T>,
			property_owner: T::AccountId,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			item_id: ItemId,
			key: BoundedVec<u8, T::KeyLimit>,
			metadata: BoundedVec<u8, T::MetadataLimit>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Ok(())
		}

		/// Clear an property for an item.
		///
		/// Origin must be signed by both `property_owner` and owner of item.
		///
		/// - `property_owner`: The owner of the property.
		/// - `template_name_or_id`: The template of the item whose metadata to clear.
		/// - `item_id`: The identifier of the item whose metadata to clear.
		/// - `key`: The value to which to clear the attribute.
		///
		/// Emits `PropertyCleared`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn clear_item_property(
			origin: OriginFor<T>,
			property_owner: T::AccountId,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			item_id: ItemId,
			key: BoundedVec<u8, T::KeyLimit>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Ok(())
		}

		/// Submit proposal with `template_name_or_id` template change. Proposal may Add/Update/Remove supported interpretations.
		///
		/// - `author`: The author of proposal
		/// - `template_name_or_id`: The template to change
		/// - `change_set`: Add/Update/Remove changes
		///
		/// Emits `ProposalSubmitted`.
		///
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn submit_template_change_proposal(
			origin: OriginFor<T>,
			author: T::AccountId,
			template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
			change_set: Vec<Change>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let proposal_id = Self::submit_proposal(author, template_name_or_id, change_set)?;
			Self::deposit_event(Event::ProposalSubmitted { proposal_id });
			Ok(())
		}
	}
}
