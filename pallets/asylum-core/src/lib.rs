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
		Change, IntepretationTypeInfo, Interpretable, Interpretation, Item, ItemTemplate, Proposal,
		ProposalInfo,
	};
	use frame_support::{pallet_prelude::*, traits::tokens::nonfungibles::Destroy, transactional};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use pallet_rmrk_core::{BoundedCollectionSymbolOf, KeyLimitOf, StringLimitOf};
	use rmrk_traits::*;
	use sp_std::vec::Vec;

	pub type BoundedInterpretation<T> =
		BoundedVec<u8, <T as pallet_rmrk_core::Config>::ResourceSymbolLimit>;

	pub type InterpretationInfo<T> = ResourceInfo<BoundedInterpretation<T>, StringLimitOf<T>>;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_rmrk_core::Config + pallet_uniques::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type NextInterpretationTypeId<T: Config> =
		StorageValue<_, InterpretationTypeId, ValueQuery>;

	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, ProposalId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn interpretation_type_id)]
	/// Human-readable names of interpretation types
	pub(super) type IntepretationTypeNames<T: Config> =
		StorageMap<_, Blake2_128Concat, StringLimitOf<T>, InterpretationTypeId, OptionQuery>;

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
	#[pallet::getter(fn item_interpretations)]
	/// Interpretations supported by Items
	pub(super) type ItemIntepretations<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, ItemTemplateId>,
			NMapKey<Twox64Concat, ItemId>,
			NMapKey<Twox64Concat, InterpretationTypeId>,
			NMapKey<Twox64Concat, BoundedInterpretation<T>>,
		),
		(),
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn template_interpretations)]
	/// Interpretations supported by Items
	pub(super) type TemplateIntepretations<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, ItemTemplateId>,
			NMapKey<Blake2_128Concat, InterpretationTypeId>,
			NMapKey<Blake2_128Concat, BoundedInterpretation<T>>,
		),
		InterpretationInfo<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_info)]
	/// Proposal's infos
	pub(super) type Proposals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ProposalId,
		ProposalInfo<T::AccountId, BoundedInterpretation<T>, StringLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub interpretation_types: Vec<(String, String)>,
		_marker: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn new(interpretation_types: Vec<(String, String)>) -> Self {
			Self { interpretation_types, _marker: Default::default() }
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { interpretation_types: Default::default(), _marker: Default::default() }
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
			for (type_name, metadata) in &self.interpretation_types {
				IntepretationTypeNames::<T>::insert(bounded(type_name), i);
				let metadata = bounded(metadata);
				let info = IntepretationTypeInfo { metadata };
				IntepretationTypes::<T>::insert(i, info);
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
			template_name: BoundedCollectionSymbolOf<T>,
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
		T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
			+ pallet_rmrk_core::Config,
	{
		/// Create new interpretation type.
		///
		/// Origin must be Signed.
		///
		/// - `type_name`: The interpretation type to be created.
		/// - `metadata`: The link to the interpretation type description stored somewhere(for
		///   example ipfs).
		///
		/// Emits `InterpretationTypeCreated`.
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

		/// Create new template. In Asylum context Template is extended
		/// Collection of NFTs, i.e. all Items minted from this Template (in
		/// this Collection) should have the same interpretations.
		///
		/// Origin must be Signed.
		///
		/// - `owner`: The owner of template.
		/// - `template_name`: The template to be created.
		/// - `interpretations`: vec of pairs of supported (interpretation_type_name,
		///   interpretation_name).
		/// - `metadata`: The link to the template description stored somewhere(for example ipfs).
		///
		/// Emits `TemplateCreated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_template(
			origin: OriginFor<T>,
			template_name: BoundedCollectionSymbolOf<T>,
			metadata: StringLimitOf<T>,
			max: Option<u32>,
			interpretations: Vec<
				Interpretation<StringLimitOf<T>, BoundedInterpretation<T>, StringLimitOf<T>>,
			>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = pallet_rmrk_core::Pallet::<T>::collection_create(
				sender.clone(),
				metadata,
				max,
				template_name.clone(),
			)?;
			Self::template_create(template_id, interpretations)?;
			pallet_uniques::Pallet::<T>::do_create_class(
				template_id,
				sender.clone(),
				sender.clone(),
				T::ClassDeposit::get(),
				false,
				pallet_uniques::Event::Created {
					class: template_id,
					creator: sender.clone(),
					owner: sender.clone(),
				},
			)?;

			Self::deposit_event(Event::TemplateCreated { template_name, template_id });
			Ok(())
		}

		/// Destroy template. In Asylum context Template is extended Collection
		/// of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_name`: The template to be destroyed.
		///
		/// Emits `TemplateDestroyed`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn destroy_template(
			origin: OriginFor<T>,
			template_id: ItemTemplateId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::template_destroy(template_id)?;
			pallet_rmrk_core::Pallet::<T>::collection_burn(sender.clone(), template_id)?;
			let witness = pallet_uniques::Pallet::<T>::get_destroy_witness(&template_id).unwrap();
			pallet_uniques::Pallet::<T>::do_destroy_class(
				template_id,
				witness,
				sender.clone().into(),
			)?;

			Self::deposit_event(Event::TemplateDestroyed { template_id });
			Ok(())
		}

		/// Update template according to proposal. In Asylum context Template is
		/// extended Collection of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_name`: The template to be destroyed.
		///
		/// Emits `TemplateUpdated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn update_template(
			origin: OriginFor<T>,
			template_id: ItemTemplateId,
			proposal_id: ProposalId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::template_update(sender, proposal_id, template_id)?;
			Self::deposit_event(Event::TemplateUpdated { template_id });
			Ok(())
		}

		/// Mint new item from 'template_name_or_id', i.e. mint Item(NFT) with
		/// the same set of supported interpretations as 'template_name_or_id'
		/// has.
		///
		/// Origin must be Signed and sender must be Issuer of Template.
		///
		/// - `recipient`: The recipient of the item 'template_name_or_id' Template.
		/// - `template_name_or_id`: The template name or id.
		/// - `metadata`: The link to the item description stored somewhere(for example ipfs).
		///
		/// Emits `ItemMinted`.
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
			let (_, item_id) = pallet_rmrk_core::Pallet::<T>::nft_mint(
				sender.clone(),
				owner,
				template_id,
				Some(recipient.clone()),
				None,
				metadata,
			)?;
			pallet_uniques::Pallet::<T>::do_mint(
				template_id,
				item_id,
				sender.clone(),
				|_details| Ok(()),
			)?;
			Self::item_mint_from_template(sender.clone(), template_id, item_id)?;

			Self::deposit_event(Event::ItemMinted { template_id, item_id, recipient });
			Ok(())
		}

		/// Destroy a single asset instance.
		///
		/// Origin must be Signed and the sender should be the Admin of the
		/// asset `template_name_or_id`.
		///
		/// - `template_name_or_id`: The template name or id
		/// - `item_id`: The item to be burned
		///
		/// Emits `ItemBurned`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_item(
			origin: OriginFor<T>,
			template_id: ItemTemplateId,
			item_id: ItemId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::item_burn(template_id, item_id)?;
			let max_recursions = T::MaxRecursions::get();
			pallet_rmrk_core::Pallet::<T>::nft_burn(template_id, item_id, max_recursions)?;
			pallet_uniques::Pallet::<T>::do_burn(template_id, item_id, |_, _| Ok(()))?;
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
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_item(
			origin: OriginFor<T>,
			template_id: ItemTemplateId,
			item_id: ItemId,
			destination: AccountIdOrCollectionNftTuple<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (destination, _) =
				pallet_rmrk_core::Pallet::<T>::nft_send(sender, template_id, item_id, destination)?;
			pallet_uniques::Pallet::<T>::do_transfer(
				template_id,
				item_id,
				destination.clone(),
				|_class_details, _details| Ok(()),
			)?;
			Self::deposit_event(Event::ItemTransfered { template_id, item_id, destination });
			Ok(())
		}

		/// Update 'item_id' item according to newest version of
		/// 'template_name_or_id' template
		///
		/// Origin must be Signed and the sender must be owner of the 'item_id'
		/// item
		///
		/// Arguments:
		/// - `template_name_or_id`: The template of the item to be updated.
		/// - `item_id`: The item to be updated.
		///
		/// Emits `ItemUpdated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn update_item(
			origin: OriginFor<T>,
			template_id: ItemTemplateId,
			item_id: ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::item_update(sender, template_id, item_id)?;
			Self::deposit_event(Event::ItemUpdated { template_id, item_id });
			Ok(())
		}

		/// Submit proposal with `template_name_or_id` template change. Proposal
		/// may Add/Update/Remove supported interpretations.
		///
		/// - `author`: The author of proposal
		/// - `template_name_or_id`: The template to change
		/// - `change_set`: Add/Update/Remove changes
		///
		/// Emits `ProposalSubmitted`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn submit_template_change_proposal(
			origin: OriginFor<T>,
			author: T::AccountId,
			template_id: ItemTemplateId,
			change_set: Vec<Change<BoundedInterpretation<T>, StringLimitOf<T>>>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let proposal_id = Self::submit_proposal(author, template_id, change_set)?;
			Self::deposit_event(Event::ProposalSubmitted { proposal_id });
			Ok(())
		}
	}
}
