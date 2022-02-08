use sp_runtime::{DispatchError, DispatchResult};

use crate::{primitives::*, NameOrId};

/// Trait for providing basic functionality of Asylum Item
pub trait Item<AccountId, BoundedName, BoundedString> {
	/// Regular Item(NFT) mint
	fn item_mint(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		metadata: Option<BoundedString>,
	) -> Result<(ItemTemplateId, ItemId), DispatchError>; // ensure(template.issuer = sender)

	/// Regular Item(NFT) burn
	fn item_burn(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError>; // ensure(item.owner = sender)

	/// Regular Item(NFT) transfer
	fn item_transfer(
		destination: &AccountId,
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		item_id: ItemId,
	) -> DispatchResult; // ensure(item.owner = sender)

	/// Update item's interpretation' set according to template
	///
	/// # Arguments
	///
	/// * `template_name_or_id` - template name or template id
	/// * `item_id` - id of the item to update
	///
	fn item_update(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		item_id: ItemId,
	) -> DispatchResult; // ensure(item.owner = sender)
}

/// Trait for providing attributes support for Asylum Item
pub trait Properties<AccountId, BoundedName, BoundedString> {
	/// Set property Item(NFT)
	fn set_property(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		item_id: ItemId,
		key: BoundedString,
		value: BoundedString,
		property_owner: AccountId,
	);

	/// Clear property Item(NFT)
	fn clear_property(
		template_name_or_id: NameOrId<BoundedName, ItemTemplateId>,
		item_id: ItemId,
		key: BoundedString,
		property_owner: AccountId,
	);
}
