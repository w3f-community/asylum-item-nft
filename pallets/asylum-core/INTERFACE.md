  # Config
  ```rust
  pub type MetadataLimitOf<T> = BoundedVec<u8, <T as Config>::MetadataLimit>;
  pub type KeyLimitOf<T> = BoundedVec<u8, <T as Config>::KeyLimit>;
  pub type NameLimitOf<T> = BoundedVec<u8, <T as Config>::NameLimit>;

  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// Nonfungible token functionality
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
  ```

  # Storage structure
  ```rust
  /// Human-readable names of interpretation types
  pub(super) type IntepretationTypeNames<T: Config> =
    StorageMap<_, Blake2_128Concat, NameLimitOf<T>, InterpretationTypeId, OptionQuery>; //genesis config

  /// Human-readable names of interpretations
  pub(super) type IntepretationNames<T: Config> =
    StorageMap<_, Blake2_128Concat, NameLimitOf<T>, InterpretationId, OptionQuery>;

  /// Human-readable names of templates
  pub(super) type TemplateNames<T: Config> =
    StorageMap<_, Blake2_128Concat, NameLimitOf<T>, ItemTemplateId, OptionQuery>;

  /// Interpretation type's infos
  pub(super) type IntepretationTypes<T: Config> = StorageMap<
    _,
    Twox64Concat,
    InterpretationTypeId,
    IntepretationTypeInfo<MetadataLimitOf<T>>,
    OptionQuery,
  >;

  /// Interpretation's infos
  pub(super) type Intepretations<T: Config> = StorageMap<
    _,
    Twox64Concat,
    InterpretationId,
    IntepretationInfo<MetadataLimitOf<T>>,
    OptionQuery,
  >;

  /// Template's infos
  pub(super) type Templates<T: Config> = StorageMap<
    _,
    Twox64Concat,
    ItemTemplateId,
    ItemTemplateInfo<T::AccountId, MetadataLimitOf<T>>,
    OptionQuery,
  >;

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

  /// Item's infos
  pub(super) type Items<T: Config> = StorageDoubleMap<
    _,
    Twox64Concat,
    ItemTemplateId,
    Twox64Concat,
    ItemId,
    ItemInfo<MetadataLimitOf<T>>,
  >;

  /// Proposal's infos
  pub(super) type Proposals<T: Config> = StorageMap<
    _,
    Twox64Concat,
    ProposalId,
    ProposalInfo<T::AccountId, MetadataLimitOf<T>>,
    OptionQuery,
  >;
  ```
  
  # Dispatchable functions
  ```rust
  /// Create new interpretation type.
  ///
  /// Origin must be Signed.
  ///
  /// - `type_name`: The interpretation type to be created.
  /// - `metadata`: The link to the interpretation type description stored somewhere (for example ipfs).
  ///
  /// Emits `InterpretationTypeCreated`.
  ///
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
  /// - `src`: The link to the media file stored somewhere (for example ipfs).
  /// - `metadata`: The link to the interpretation type description stored somewhere (for example ipfs).
  ///
  /// Emits `InterpretationCreated`.
  ///
  pub fn create_interpretation(
    origin: OriginFor<T>,
    interpretation_name: NameLimitOf<T>,
    src: MetadataLimitOf<T>,
    metadata: MetadataLimitOf<T>,
  ) -> DispatchResult;

  /// Create new Template. In Asylum context Template is an extended Collection of NFTs, i.e., all Items minted from this Template (in this Collection)
  /// should have the same interpretations.
  ///
  /// Origin must be Signed.
  ///
  /// - `owner`: The owner of Template.
  /// - `template_name`: The Template to be created.
  /// - `interpretations`: Vec of interpretations
  /// - `metadata`: The link to the Template description stored somewhere(for example ipfs).
  ///
  /// Emits `TemplateCreated`.
  ///
  pub fn create_template(
    origin: OriginFor<T>,
    owner: T::AccountId,
    template_name: NameLimitOf<T>,
    metadata: MetadataLimitOf<T>,
    interpretations: Vec<Interpretation<NameLimitOf<T>>>,
  ) -> DispatchResult;

  /// Destroy Template. In Asylum context Template is an extended Collection of NFTs.
  ///
  /// Origin must be Signed, and the sender should be the Template's owner.
  ///
  /// - `template_name`: The template to be destroyed.
  ///
  /// Emits `TemplateDestroyed`.
  ///
  pub fn destroy_template(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
  ) -> DispatchResult;

  /// Update template according to proposal. In Asylum context Template is an extended Collection of NFTs.
  ///
  /// Origin must be Signed, and the sender should be the Template's owner.
  ///
  /// - `template_name`: The Template to be updated.
  ///
  /// Emits `TemplateUpdated`.
  ///
  pub fn update_template(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
    proposal_id: ProposalId,
  ) -> DispatchResult;

  /// Change Template's issuer. In Asylum context Template is an extended Collection of NFTs.
  ///
  /// Origin must be Signed, and the sender should be the Template's owner.
  ///
  /// - `template_name`: The Template, which issuer is changing.
  /// - `new_issuer`: New issuer of the template.
  ///
  /// Emits `TemplateIssuerChanged`.
  ///
  pub fn change_template_issuer(
    origin: OriginFor<T>,
    template_name: NameLimitOf<T>,
    new_issuer: T::AccountId,
  ) -> DispatchResult;

  /// Mint new item from 'template_name_or_id', i.e. mint Item (NFT) with the same set of supported interpretations as 'template_name_or_id' has.
  ///
  /// Origin must be Signed, and the sender must be the Issuer of the Template.
  ///
  /// - `recipient`: The recipient of the item 'template_name_or_id' Template.
  /// - `template_name_or_id`: The Template name or id.
  /// - `metadata`: The link to the item description stored somewhere (for example ipfs).
  ///
  /// Emits `ItemMinted`.
  ///
  pub fn mint_item(
    origin: OriginFor<T>,
    recipient: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    metadata: MetadataLimitOf<T>,
  ) -> DispatchResult;

  /// Destroy a single item.
  ///
  /// Origin must be Signed, and the sender should be the Admin of the `template_name_or_id` Template.
  ///
  /// - `template_name_or_id`: The Template name or id
  /// - `item_id`: The item to be burned
  ///
  /// Emits `ItemBurned`.
  ///
  pub fn burn_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
  ) -> DispatchResult;

  /// Move an item from the sender account to the `destination` account.
  ///
  /// Origin must be Signed and the signing account must be either:
  /// - the Admin of the Template `template_name_or_id`;
  /// - the Owner of the Item `item_id`;
  ///
  /// Arguments:
  /// - `template_name_or_id`: The Template of the item to be transferred.
  /// - `item_id`: The Item to be transferred.
  /// - `destination`: The account to receive ownership of the asset.
  ///
  /// Emits `ItemTransferred`.
  ///
  pub fn transfer_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    destination: T::AccountId,
  ) -> DispatchResult;

  /// Update 'item_id' Item according to the newest version of 'template_name_or_id' Template.
  ///
  /// Origin must be Signed, and the sender must be the owner of the 'item_id' Item.
  ///
  /// Arguments:
  /// - `template_name_or_id`: The Template of the item to be updated.
  /// - `item_id`: The Item to be updated.
  ///
  /// Emits `ItemUpdated`.
  ///
  pub fn update_item(
    origin: OriginFor<T>,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
  ) -> DispatchResult ;

  /// Set a property for an Item.
  ///
  /// Origin must be signed by both `property_owner` and the owner of the Item.
  ///
  /// - `property_owner`: The owner of the property.
  /// - `template_name_or_id`: The Template of the item whose metadata to set.
  /// - `item_id`: The identifier of the Item whose metadata to set.
  /// - `key`: The value to which to set the attribute.
  /// - `metadata`: The value to which to set the attribute.
  ///
  /// Emits `PropertySet`.
  ///
  pub fn set_item_property(
    origin: OriginFor<T>,
    property_owner: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    key: KeyLimitOf<T>,
    metadata: MetadataLimitOf<T>,
  ) -> DispatchResult;

  /// Clear a property for an Item.
  ///
  /// Origin must be signed by both `property_owner` and the owner of the Item.
  ///
  /// - `property_owner`: The owner of the property.
  /// - `template_name_or_id`: The Template of the Item whose metadata to clear.
  /// - `item_id`: The identifier of the Item whose metadata to clear.
  /// - `key`: The value to which to clear the attribute.
  ///
  /// Emits `PropertyCleared`.
  ///
  pub fn clear_item_property(
    origin: OriginFor<T>,
    property_owner: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    item_id: ItemId,
    key: KeyLimitOf<T>,
  ) -> DispatchResult;

  /// Submit proposal with `template_name_or_id` template change. Proposal may Add/Update/Remove supported interpretations.
  ///
  /// - `author`: The author of proposal
  /// - `template_name_or_id`: The template to change
  /// - `change_set`: Add/Update/Remove changes
  ///
  /// Emits `ProposalSubmitted`.
  ///
  pub fn submit_proposal(
    origin: OriginFor<T>,
    author: T::AccountId,
    template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
    change_set: Vec<Change>,
  ) -> DispatchResult;
  ```