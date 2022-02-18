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
		Change, IntepretationInfo, IntepretationTypeInfo, Interpretable, Interpretation, Item,
		ItemTemplate, Proposal, ProposalInfo,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::nonfungibles::{Create, Destroy, Inspect, Mutate, Transfer},
		transactional,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use rmrk_traits::*;
	use sp_std::vec::Vec;

	pub type StringLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;
	pub type KeyLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>;
	pub type ValueLimitOf<T> = BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type ItemNFT: Inspect<Self::AccountId, ClassId = ItemTemplateId, InstanceId = ItemId>
			+ Create<Self::AccountId>
			+ Destroy<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ Transfer<Self::AccountId>;

		// pallet_rmrk_core
		type ItemRMRKCore: Collection<StringLimitOf<Self>, Self::AccountId>
			+ Nft<Self::AccountId, StringLimitOf<Self>>
			+ Resource<StringLimitOf<Self>, Self::AccountId>
			+ Property<KeyLimitOf<Self>, ValueLimitOf<Self>, Self::AccountId>;
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
	#[pallet::getter(fn interpretation_type_id)]
	/// Human-readable names of interpretation types
	pub(super) type IntepretationTypeNames<T: Config> =
		StorageMap<_, Blake2_128Concat, StringLimitOf<T>, InterpretationTypeId, OptionQuery>; //genesis config

	#[pallet::storage]
	#[pallet::getter(fn interpretation_type_info)]
	/// Interpretation type's infos
	pub(super) type IntepretationTypes<T: Config> = StorageMap<
		_,
		Twox64Concat,
		InterpretationTypeId,
		IntepretationTypeInfo<StringLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn interpretation_info)]
	/// Interpretation's infos
	pub(super) type Intepretations<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		InterpretationTypeId,
		Twox64Concat,
		InterpretationId,
		IntepretationInfo<StringLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn item_interpretations)]
	/// Interpretations supported by Items
	pub(super) type ItemIntepretations<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, ItemTemplateId>,
			NMapKey<Twox64Concat, ItemId>,
			NMapKey<Twox64Concat, InterpretationTypeId>,
		),
		Vec<InterpretationId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn template_interpretations)]
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
	#[pallet::getter(fn proposal_info)]
	/// Proposal's infos
	pub(super) type Proposals<T: Config> =
		StorageMap<_, Twox64Concat, ProposalId, ProposalInfo<T::AccountId>, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub interpretations: Vec<(String, String, Vec<(String, String, String)>)>,
		_marker: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn new(interpretations: Vec<(String, String, Vec<(String, String, String)>)>) -> Self {
			Self { interpretations, _marker: Default::default() }
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { interpretations: Default::default(), _marker: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			fn bounded<T>(string: &str) -> BoundedVec<u8, T>
			where
				T: Get<u32>,
			{
				TryInto::<BoundedVec<u8, T>>::try_into(string.as_bytes().to_vec()).unwrap()
			}

			let mut i = 0;
			let mut j = 0;
			for (type_name, metadata, interpretations) in &self.interpretations {
				IntepretationTypeNames::<T>::insert(bounded(type_name), i);
				let metadata = bounded(metadata);
				let info = IntepretationTypeInfo { metadata };
				IntepretationTypes::<T>::insert(i, info);

				for (name, src, metadata) in interpretations {
					let metadata = bounded(&metadata);
					let src = bounded(&src);
					let name = bounded(&name);
					let info = IntepretationInfo { name, src, metadata };
					Intepretations::<T>::insert(i, j, info);
					j += 1;
				}
				i += 1;
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		InterpretationTypeCreated {
			type_name: StringLimitOf<T>,
			type_id: InterpretationTypeId,
		},
		InterpretationCreated {
			interpretation_name: StringLimitOf<T>,
			intepretation_id: InterpretationId,
		},
		TemplateCreated {
			template_name: StringLimitOf<T>,
			template_id: ItemTemplateId,
		},
		TemplateUpdated {
			template_id: ItemTemplateId,
		},
		TemplateIssuerChanged {
			template_id: ItemTemplateId,
			new_issuer: T::AccountId,
		},
		TemplateDestroyed {
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
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>,
	{
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
			type_name: StringLimitOf<T>,
			metadata: StringLimitOf<T>,
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
			type_name: StringLimitOf<T>,
			interpretation_name: StringLimitOf<T>,
			src: StringLimitOf<T>,
			metadata: StringLimitOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let intepretation_id = Self::interpretation_create(
				&type_name,
				interpretation_name.clone(),
				src,
				metadata,
			)?;
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
			template_name: StringLimitOf<T>,
			metadata: StringLimitOf<T>,
			capacity: u32,
			interpretations: Vec<Interpretation<StringLimitOf<T>>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = T::ItemRMRKCore::collection_create(
				sender.clone(),
				metadata,
				capacity,
				template_name.clone(),
			)?;
			Self::template_create(template_id, interpretations)?;
			T::ItemNFT::create_class(&template_id, &sender, &sender)?;
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
			template_id: ItemTemplateId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::template_destroy(template_id)?;
			T::ItemRMRKCore::collection_burn(sender, template_id)?;
			let witness = T::ItemNFT::get_destroy_witness(&template_id).unwrap();
			T::ItemNFT::destroy(template_id, witness, T::ItemNFT::class_owner(&template_id))?;
			Self::deposit_event(Event::TemplateDestroyed { template_id });
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
			template_id: ItemTemplateId,
			proposal_id: ProposalId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::template_update(proposal_id, template_id)?;
			Self::deposit_event(Event::TemplateUpdated { template_id });
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
		pub fn mint_item_from_template(
			origin: OriginFor<T>,
			owner: T::AccountId,
			recipient: T::AccountId,
			template_id: ItemTemplateId,
			metadata: StringLimitOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_, item_id) = T::ItemRMRKCore::nft_mint(
				sender,
				owner,
				template_id,
				Some(recipient.clone()),
				None,
				metadata,
			)?;
			Self::item_mint_from_template(template_id, item_id)?;
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
			template_id: ItemTemplateId,
			item_id: ItemId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::item_burn(template_id, item_id)?;
			T::ItemRMRKCore::nft_burn(template_id, item_id, 10)?;
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
			template_id: ItemTemplateId,
			item_id: ItemId,
			destination: AccountIdOrCollectionNftTuple<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let destination = T::ItemRMRKCore::nft_send(sender, template_id, item_id, destination)?;
			T::ItemNFT::transfer(&template_id, &item_id, &destination)?;
			// TODO: Uniques tranfer
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
			template_id: ItemTemplateId,
			item_id: ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(
				T::ItemNFT::owner(&template_id, &item_id) == Some(sender),
				Error::<T>::NoPermission
			);
			Self::item_update(template_id, item_id)?;
			Self::deposit_event(Event::ItemUpdated { template_id, item_id });
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
			template_id: ItemTemplateId,
			change_set: Vec<Change>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let proposal_id = Self::submit_proposal(author, template_id, change_set)?;
			Self::deposit_event(Event::ProposalSubmitted { proposal_id });
			Ok(())
		}
	}
}
