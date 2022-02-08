		
  # Storage structure
    
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
	pub(super) type Proposals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ProposalId,
		ProposalInfo<T::AccountId, MetadataLimitOf<T>>,
		OptionQuery,
	>;

    
  # Dispatchable functions
  
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
  pub fn create_interpretation_type(
    origin: OriginFor<T>,
    type_name: NameLimitOf<T>,
    metadata: MetadataLimitOf<T>,
  ) -> DispatchResult;

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
  pub fn create_interpretation(
    origin: OriginFor<T>,
    interpretation_name: NameLimitOf<T>,
    src: MetadataLimitOf<T>,
    metadata: MetadataLimitOf<T>,
  ) -> DispatchResult;

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
  pub fn create_template(
    origin: OriginFor<T>,
    owner: T::AccountId,
    template_name: NameLimitOf<T>,
    metadata: MetadataLimitOf<T>,
    interpretations: Vec<(NameLimitOf<T>, NameLimitOf<T>)>,
  ) -> DispatchResult;

  /// Destroy template. In Asylum context Template is extended Collection of NFTs.
  ///
  /// Origin must be Signed and sender should be owner of the template.
  ///
  /// - `template_name`: The template to be destroyed.
  ///
  /// Emits `TemplateDestroyed`.
  ///
  #[pallet::weight(10_000)]
  pub fn destroy_template(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
  ) -> DispatchResult;

  /// Update template according to proposal. In Asylum context Template is extended Collection of NFTs.
  ///
  /// Origin must be Signed and sender should be owner of the template.
  ///
  /// - `template_name`: The template to be destroyed.
  ///
  /// Emits `TemplateUpdated`.
  ///
  #[pallet::weight(10_000)]
  pub fn update_template(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
    proposal_id: ProposalId,
  ) -> DispatchResult;

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
  pub fn change_template_issuer(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
    new_issuer: T::AccountId,
  ) -> DispatchResult;

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
  pub fn mint_item(
    origin: OriginFor<T>,
    recipient: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    metadata: Option<BoundedVec<u8, T::MetadataLimit>>,
  ) -> DispatchResult;

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
  pub fn burn_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
  ) -> DispatchResult;

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
  pub fn transfer_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    destination: T::AccountId,
  ) -> DispatchResult;

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
  pub fn update_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
  ) -> DispatchResult ;

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
  pub fn set_item_property(
    origin: OriginFor<T>,
    property_owner: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    key: BoundedVec<u8, T::KeyLimit>,
    metadata: BoundedVec<u8, T::MetadataLimit>,
  ) -> DispatchResult;

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
  pub fn clear_item_property(
    origin: OriginFor<T>,
    property_owner: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    key: BoundedVec<u8, T::KeyLimit>,
  ) -> DispatchResult;

  /// Submit proposal with `template_name_or_id` template change. Proposal may Add/Update/Remove supported interpretations.
  ///
  /// - `author`: The author of proposal
  /// - `template_name_or_id`: The template to change
  /// - `change_set`: Add/Update/Remove changes
  ///
  /// Emits `ProposalSubmitted`.
  ///
  #[pallet::weight(10_000)]
  pub fn submit_template_change_proposal(
    origin: OriginFor<T>,
    author: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    change_set: Vec<Change<MetadataLimitOf<T>>>,
  ) -> DispatchResult;
}