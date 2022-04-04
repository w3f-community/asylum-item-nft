use super::*;
use frame_support::{traits::Get, BoundedVec};
use scale_info::TypeInfo;

pub(super) type GameDetailsFor<T> = GameDetails<<T as SystemConfig>::AccountId, BalanceOf<T>>;
pub(super) type TicketDetailsFor<T> = TicketDetails<<T as SystemConfig>::AccountId>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct GameDetails<AccountId, Balance> {
	/// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
	pub(super) owner: AccountId,
	/// Can mint tokens.
	pub(super) issuer: AccountId,
	/// Can thaw tokens, force transfers and burn tokens from any account.
	pub(super) admin: AccountId,
	/// Can freeze tokens.
	pub(super) freezer: AccountId,
	/// Game price
	pub(super) price: Balance,
	/// The total number of outstanding instances of this asset class.
	pub(super) instances: u32,
	/// The total number of outstanding instance metadata of this asset class.
	pub(super) instance_metadatas: u32,
	/// The total number of attributes for this asset class.
	pub(super) attributes: u32,
	/// Whether the asset is frozen for non-admin transfers.
	pub(super) is_frozen: bool,
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

impl<AccountId, Balance> GameDetails<AccountId, Balance> {
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
#[scale_info(skip_type_params(StringLimit))]
pub struct GameMetadata<StringLimit: Get<u32>> {
	/// General information concerning this asset. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedVec<u8, StringLimit>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
#[scale_info(skip_type_params(StringLimit))]
pub struct TicketMetadata<StringLimit: Get<u32>> {
	/// General information concerning this asset. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedVec<u8, StringLimit>,
}
