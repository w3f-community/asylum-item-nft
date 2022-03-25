use sp_runtime::{DispatchError, DispatchResult};

use crate::primitives::{ItemId, ItemTemplateId};

/// Trait for providing basic functionality of Asylum Item
pub trait Item<AccountId, BoundedName, BoundedString> {
	/// Regular Item(NFT) mint
	fn item_mint_from_template(
		sender: AccountId,
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError>; // ensure(template.issuer = sender)

	/// Regular Item(NFT) burn
	fn item_burn(
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError>; // ensure(item.owner = sender)

	/// Update item's interpretation' set according to template
	///
	/// # Arguments
	///
	/// * `template_name_or_id` - template name or template id
	/// * `item_id` - id of the item to update
	fn item_update(sender: AccountId, template_id: ItemTemplateId, item_id: ItemId) -> DispatchResult; // ensure(item.owner = sender)
}

/// Trait for providing attributes support for Asylum Item
pub trait Properties<AccountId, BoundedName, BoundedString> {
	/// Set property Item(NFT)
	fn set_property(
		template_id: ItemTemplateId,
		item_id: ItemId,
		key: BoundedString,
		value: BoundedString,
		property_owner: AccountId,
	);

	/// Clear property Item(NFT)
	fn clear_property(
		template_id: ItemTemplateId,
		item_id: ItemId,
		key: BoundedString,
		property_owner: AccountId,
	);
}
