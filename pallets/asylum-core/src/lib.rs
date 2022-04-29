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
		primitives::{ItemId, ProposalId, TemplateId},
		Change, IntepretationInfo, Interpretable, Interpretation, Item, ItemTemplate, Proposal,
		ProposalInfo, TagInfo,
	};
	use frame_support::{pallet_prelude::*, traits::tokens::nonfungibles::Destroy, transactional};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use pallet_rmrk_core::{BoundedCollectionSymbolOf, KeyLimitOf, StringLimitOf};
	use rmrk_traits::*;
	use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

	pub type BoundedInterpretationOf<T> =
		BoundedVec<u8, <T as pallet_rmrk_core::Config>::ResourceSymbolLimit>;

	pub type ResourceInfoOf<T> = ResourceInfo<BoundedInterpretationOf<T>, StringLimitOf<T>>;

	pub type TagLimitOf<T> = BoundedVec<u8, <T as Config>::TagLimit>;
	pub type TagsOf<T> = BTreeSet<TagLimitOf<T>>;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_rmrk_core::Config + pallet_uniques::Config
	{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type TagLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type NextProposalId<T: Config> = StorageValue<_, ProposalId, ValueQuery>;

	#[pallet::storage]
	/// Interpretation tag's infos
	#[pallet::getter(fn tags)]
	pub(super) type Tags<T: Config> =
		StorageMap<_, Twox64Concat, TagLimitOf<T>, TagInfo<StringLimitOf<T>>, OptionQuery>;

	#[pallet::storage]
	/// Interpretations supported by Items
	#[pallet::getter(fn item_interpretation_tags)]
	pub(super) type ItemInterpretationTags<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Twox64Concat, TemplateId>,
			NMapKey<Twox64Concat, ItemId>,
			NMapKey<Twox64Concat, BoundedInterpretationOf<T>>,
		),
		TagsOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	/// Interpretations supported by Items
	#[pallet::getter(fn template_interpretations)]
	pub(super) type TemplateIntepretations<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		TemplateId,
		Blake2_128Concat,
		BoundedInterpretationOf<T>,
		(IntepretationInfo<BoundedInterpretationOf<T>, StringLimitOf<T>>, TagsOf<T>),
		OptionQuery,
	>;

	#[pallet::storage]
	/// Proposal's infos
	pub(super) type Proposals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ProposalId,
		ProposalInfo<T::AccountId, BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>,
		OptionQuery,
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub interpretation_tags: Vec<(String, String)>,
		_marker: PhantomData<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn new(interpretation_tags: Vec<(String, String)>) -> Self {
			Self { interpretation_tags, _marker: Default::default() }
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { interpretation_tags: Default::default(), _marker: Default::default() }
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

			for (tag, metadata) in self.interpretation_tags.iter() {
				let metadata = bounded(metadata);
				let info = TagInfo { metadata };
				Tags::<T>::insert(bounded(tag), info);
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		InterpretationTagCreated { tag: TagLimitOf<T> },
		TemplateCreated { template_name: BoundedCollectionSymbolOf<T>, template_id: TemplateId },
		TemplateUpdated { template_id: TemplateId },
		TemplateIssuerChanged { template_id: TemplateId, new_issuer: T::AccountId },
		TemplateDestroyed { template_id: TemplateId },
		ItemMinted { template_id: TemplateId, item_id: ItemId },
		ItemBurned { template_id: TemplateId, item_id: ItemId },
		ItemTransfered { template_id: TemplateId, item_id: ItemId, destination: T::AccountId },
		ItemUpdated { template_id: TemplateId, item_id: ItemId },
		ItemAttributeSet { item_id: ItemId, key: KeyLimitOf<T> },
		ItemAttributeCleared { item_id: ItemId, key: KeyLimitOf<T> },
		ProposalSubmitted { proposal_id: ProposalId },
	}

	#[pallet::error]
	pub enum Error<T> {
		TagAlreadyExists,
		UnknownTag,
		TemplateDoesntSupportThisInterpretation,
		EmptyTags,
		ProposalNotExist,
		ProposalNotApproved,
		ProposalInappropriateTemplate,
		NoAvailableId,
		NoPermission,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId>
			+ pallet_rmrk_core::Config,
	{
		/// Create new interpretation tag.
		///
		/// Origin must be Signed.
		///
		/// - `tag`: The interpretation tag to be created.
		/// - `metadata`: The link to the interpretation tag's metadata
		///
		/// Emits `InterpretationTagCreated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create_interpretation_tag(
			origin: OriginFor<T>,
			tag: TagLimitOf<T>,
			metadata: StringLimitOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			Self::interpretation_tag_create(&tag, metadata)?;
			Self::deposit_event(Event::InterpretationTagCreated { tag });
			Ok(())
		}

		/// Create new template. In Asylum context Template is extended
		/// Collection of NFTs, i.e. all Items minted from this Template (in
		/// this Collection) should have the same interpretations.
		///
		/// Origin must be Signed.
		///
		/// - `template_name`: The RMRK Collection's symbol.
		/// - `metadata`: The RMRK Collection's metadata.
		/// - `max`: The RMRK Collection's max.
		/// - `interpretations`: vec of pairs of Interpretations.
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
				Interpretation<BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>,
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
					owner: sender,
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
		/// - `template_id`: The template to be destroyed.
		///
		/// Emits `TemplateDestroyed`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn destroy_template(origin: OriginFor<T>, template_id: TemplateId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let template_id = Self::template_destroy(template_id)?;
			pallet_rmrk_core::Pallet::<T>::collection_burn(sender.clone(), template_id)?;
			let witness = pallet_uniques::Pallet::<T>::get_destroy_witness(&template_id).unwrap();
			pallet_uniques::Pallet::<T>::do_destroy_class(template_id, witness, sender.into())?;

			Self::deposit_event(Event::TemplateDestroyed { template_id });
			Ok(())
		}

		/// Update template according to proposal. In Asylum context Template is
		/// extended Collection of NFTs.
		///
		/// Origin must be Signed and sender should be owner of the template.
		///
		/// - `template_id`: The template to be destroyed.
		/// - `proposal_id`: The template update proposal id.
		///
		/// Emits `TemplateUpdated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn update_template(
			origin: OriginFor<T>,
			template_id: TemplateId,
			proposal_id: ProposalId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::template_update(sender, proposal_id, template_id)?;
			Self::deposit_event(Event::TemplateUpdated { template_id });
			Ok(())
		}

		/// Mint new item from 'template_id', i.e. mint Item(NFT) with
		/// the same set of supported interpretations as 'template_id'
		/// has.
		///
		/// Origin must be Signed and sender must be Issuer of Template.
		///
		/// - `owner`: The owner of the minted item.
		/// - `template_id`: The template name or id.
		/// - `metadata`: The link to the item description stored somewhere(for example ipfs).
		///
		/// Emits `ItemMinted`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn mint_item_from_template(
			origin: OriginFor<T>,
			owner: T::AccountId,
			template_id: TemplateId,
			metadata: StringLimitOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let (_, item_id) = pallet_rmrk_core::Pallet::<T>::nft_mint(
				sender.clone(),
				owner.clone(),
				template_id,
				None,
				None,
				metadata,
			)?;
			pallet_uniques::Pallet::<T>::do_mint(template_id, item_id, owner, |_details| Ok(()))?;
			Self::item_mint_from_template(sender, template_id, item_id)?;

			Self::deposit_event(Event::ItemMinted { template_id, item_id });
			Ok(())
		}

		/// Destroy an item.
		///
		/// Origin must be Signed and the sender should be the Admin of the
		/// asset `template_id`.
		///
		/// - `template_id`: The template name or id
		/// - `item_id`: The item to be burned
		///
		/// Emits `ItemBurned`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn burn_item(
			origin: OriginFor<T>,
			template_id: TemplateId,
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

		/// Move an item from the sender account to another.
		///
		/// Origin must be Signed and the signing account must be owner of the item
		///
		/// Arguments:
		/// - `template_id`: The template of the item to be transferred.
		/// - `item_id`: The item to be transferred.
		/// - `destination`: The account to receive ownership of the asset.
		///
		/// Emits `ItemTransferred`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer_item(
			origin: OriginFor<T>,
			template_id: TemplateId,
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
		/// 'template_id' template
		///
		/// Origin must be Signed and the sender must be owner of the 'item_id'
		/// item
		///
		/// Arguments:
		/// - `template_id`: The template of the item to be updated.
		/// - `item_id`: The item to be updated.
		///
		/// Emits `ItemUpdated`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn accept_item_update(
			origin: OriginFor<T>,
			template_id: TemplateId,
			item_id: ItemId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			Self::item_accept_update(sender, template_id, item_id)?;
			Self::deposit_event(Event::ItemUpdated { template_id, item_id });
			Ok(())
		}

		/// Submit proposal with `template_id` template change. Proposal
		/// may Add/Update/Remove supported interpretations.
		///
		/// - `author`: The author of proposal
		/// - `template_id`: The template to change
		/// - `change_set`: AddOrUpdate/RemoveInterpretation/RemoveInterpretationType changes
		///
		/// Emits `ProposalSubmitted`.
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn submit_template_change_proposal(
			origin: OriginFor<T>,
			author: T::AccountId,
			template_id: TemplateId,
			change_set: Vec<Change<BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let proposal_id = Self::submit_proposal(author, template_id, change_set)?;
			Self::deposit_event(Event::ProposalSubmitted { proposal_id });
			Ok(())
		}
	}
}
