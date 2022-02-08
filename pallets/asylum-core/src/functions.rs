use asylum_traits::{primitives::*, NameOrId};

use super::*;

impl<T: Config> Pallet<T> {
	pub fn get_next_template_id() -> Result<ItemTemplateId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextTemplateId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_interpretation_type_id() -> Result<InterpretationTypeId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextInterpretationTypeId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_interpretation_id() -> Result<InterpretationId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextInterpretationId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_item_id() -> Result<ItemId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextItemId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_proposal_id() -> Result<ItemId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextProposalId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(1).ok_or(Error::<T>::NoAvailableId)?;
			Ok(current_id)
		})
	}

	pub fn get_template_id(
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
	) -> Result<ItemTemplateId, Error<T>> {
		match template_name_or_id {
			NameOrId::Name(name) => {
				TemplateNames::<T>::get(name).ok_or(Error::<T>::TemplateNotExist)
			},
			NameOrId::Id(id) => Ok(id),
		}
	}
}
