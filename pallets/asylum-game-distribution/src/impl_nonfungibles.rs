use super::*;
use frame_support::{traits::tokens::nonfungibles::*, BoundedSlice};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::prelude::*;

impl<T: Config> Inspect<<T as SystemConfig>::AccountId> for Pallet<T> {
	type InstanceId = T::TicketId;
	type ClassId = T::GameId;

	fn owner(
		class: &Self::ClassId,
		instance: &Self::InstanceId,
	) -> Option<<T as SystemConfig>::AccountId> {
		Ticket::<T>::get(class, instance).map(|a| a.owner)
	}

	fn class_owner(class: &Self::ClassId) -> Option<<T as SystemConfig>::AccountId> {
		Game::<T>::get(class).map(|a| a.owner)
	}

	/// Returns the attribute value of `instance` of `class` corresponding to `key`.
	///
	/// When `key` is empty, we return the instance metadata value.
	///
	/// By default this is `None`; no attributes are defined.
	fn attribute(
		class: &Self::ClassId,
		instance: &Self::InstanceId,
		key: &[u8],
	) -> Option<Vec<u8>> {
		if key.is_empty() {
			// We make the empty key map to the instance metadata value.
			TicketMetadataOf::<T>::get(class, instance).map(|m| m.data.into())
		} else {
			let key = BoundedSlice::<_, _>::try_from(key).ok()?;
			Attribute::<T>::get((class, Some(instance), key)).map(|a| a.into())
		}
	}

	/// Returns the attribute value of `instance` of `class` corresponding to `key`.
	///
	/// When `key` is empty, we return the instance metadata value.
	///
	/// By default this is `None`; no attributes are defined.
	fn class_attribute(class: &Self::ClassId, key: &[u8]) -> Option<Vec<u8>> {
		if key.is_empty() {
			// We make the empty key map to the instance metadata value.
			GameMetadataOf::<T>::get(class).map(|m| m.data.into())
		} else {
			let key = BoundedSlice::<_, _>::try_from(key).ok()?;
			Attribute::<T>::get((class, Option::<T::TicketId>::None, key)).map(|a| a.into())
		}
	}

	/// Returns `true` if the asset `instance` of `class` may be transferred.
	///
	/// Default implementation is that all assets are transferable.
	fn can_transfer(class: &Self::ClassId, instance: &Self::InstanceId) -> bool {
		matches!((Game::<T>::get(class), Ticket::<T>::get(class, instance)), (Some(cd), Some(id)) if !cd.is_frozen && !id.is_frozen)
	}
}

impl<T: Config> Create<<T as SystemConfig>::AccountId> for Pallet<T> {
	/// Create a `class` of nonfungible assets to be owned by `who` and managed by `admin`.
	fn create_class(
		class: &Self::ClassId,
		who: &T::AccountId,
		admin: &T::AccountId,
	) -> DispatchResult {
		let admins = BTreeSet::from([admin.clone()]);
		Self::do_create_game(
			*class,
			who.clone(),
			admins.clone(),
			Default::default(),
			Event::GameCreated { game: *class, owner: who.clone(), admins },
		)
	}
}

impl<T: Config> Destroy<<T as SystemConfig>::AccountId> for Pallet<T> {
	type DestroyWitness = DestroyWitness;

	fn get_destroy_witness(class: &Self::ClassId) -> Option<DestroyWitness> {
		Game::<T>::get(class).map(|a| a.destroy_witness())
	}

	fn destroy(
		class: Self::ClassId,
		witness: Self::DestroyWitness,
		maybe_check_owner: Option<T::AccountId>,
	) -> Result<Self::DestroyWitness, DispatchError> {
		Self::do_destroy_game(class, witness, maybe_check_owner)
	}
}

impl<T: Config> Mutate<<T as SystemConfig>::AccountId> for Pallet<T> {
	fn mint_into(
		class: &Self::ClassId,
		instance: &Self::InstanceId,
		who: &T::AccountId,
	) -> DispatchResult {
		Self::do_mint_ticket(*class, *instance, who.clone(), |_| Ok(()))
	}

	fn burn_from(class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
		Self::do_burn_ticket(*class, *instance, |_, _| Ok(()))
	}
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
	fn transfer(
		class: &Self::ClassId,
		instance: &Self::InstanceId,
		destination: &T::AccountId,
	) -> DispatchResult {
		Self::do_transfer(*class, *instance, destination.clone(), |_, _| Ok(()))
	}
}

impl<T: Config> InspectEnumerable<T::AccountId> for Pallet<T> {
	/// Returns an iterator of the asset classes in existence.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn classes() -> Box<dyn Iterator<Item = Self::ClassId>> {
		Box::new(GameMetadataOf::<T>::iter_keys())
	}

	/// Returns an iterator of the instances of an asset `class` in existence.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn instances(class: &Self::ClassId) -> Box<dyn Iterator<Item = Self::InstanceId>> {
		Box::new(TicketMetadataOf::<T>::iter_key_prefix(class))
	}

	/// Returns an iterator of the asset instances of all classes owned by `who`.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn owned(who: &T::AccountId) -> Box<dyn Iterator<Item = (Self::ClassId, Self::InstanceId)>> {
		Box::new(Account::<T>::iter_key_prefix((who,)))
	}

	/// Returns an iterator of the asset instances of `class` owned by `who`.
	///
	/// NOTE: iterating this list invokes a storage read per item.
	fn owned_in_class(
		class: &Self::ClassId,
		who: &T::AccountId,
	) -> Box<dyn Iterator<Item = Self::InstanceId>> {
		Box::new(Account::<T>::iter_key_prefix((who, class)))
	}
}
