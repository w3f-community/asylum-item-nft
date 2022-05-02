use super::*;
use asylum_traits::primitives::TemplateId;
use scale_info::TypeInfo;
use sp_std::collections::btree_set::BTreeSet;
pub(super) type GameDetailsFor<T> =
	GameDetails<<T as SystemConfig>::AccountId, BalanceOf<T>, AssetIdOf<T>>;
pub(super) type TicketDetailsFor<T> = TicketDetails<<T as SystemConfig>::AccountId>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, Default)]
pub struct GameDetails<AccountId, Balance, AssetId> {
	/// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
	pub(super) owner: AccountId,
	/// Can mint tokens.
	pub(super) issuers: BTreeSet<AccountId>,
	/// Can thaw tokens, force transfers and burn tokens from any account.
	pub(super) admins: BTreeSet<AccountId>,
	/// Can freeze tokens.
	pub(super) freezers: BTreeSet<AccountId>,
	/// Game price
	pub(super) price: Option<Balance>,
	/// The total number of outstanding instances of this asset class.
	pub(super) instances: u32,
	/// The total number of outstanding instance metadata of this asset class.
	pub(super) instance_metadatas: u32,
	/// The total number of attributes for this asset class.
	pub(super) attributes: u32,
	/// Whether the asset is frozen for non-admin transfers.
	pub(super) is_frozen: bool,
	/// Set of supported templates
	pub(super) templates: Option<BTreeSet<TemplateId>>, // Maybe we should use Vec here
	/// Assets associated with this game
	pub(super) assets: Option<BTreeSet<AssetId>>, // Maybe we should use Vec here
	/// Allow tickets minting by non-issuer account
	pub(super) allow_unprivileged_mint: bool,
}

/// Witness data for the destroy transactions.
#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DestroyWitness {
	/// The total number of outstanding instances of this asset class.
	#[codec(compact)]
	pub instances: u32,
	/// The total number of outstanding instance metadata of this asset class.
	#[codec(compact)]
	pub instance_metadatas: u32,
	#[codec(compact)]
	/// The total number of attributes for this asset class.
	pub attributes: u32,
}

impl<AccountId, Balance, AssetId> GameDetails<AccountId, Balance, AssetId> {
	pub fn destroy_witness(&self) -> DestroyWitness {
		DestroyWitness {
			instances: self.instances,
			instance_metadatas: self.instance_metadatas,
			attributes: self.attributes,
		}
	}
}

/// Information concerning the ownership of a single unique asset.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct TicketDetails<AccountId> {
	/// The owner of this asset.
	pub(super) owner: AccountId,
	/// The approved transferrer of this asset, if one is set.
	pub(super) approved: Option<AccountId>,
	/// Whether the asset can be transferred or not.
	pub(super) is_frozen: bool,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct GameMetadata<BoundedData, BoundedString> {
	/// General information concerning this asset. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedData,
	pub(super) title: BoundedString,
	pub(super) genre: BoundedString,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct TicketMetadata<BoundedData> {
	/// General information concerning this asset. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedData,
}
