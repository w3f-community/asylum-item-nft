use asylum_traits::primitives::*;

use super::*;

impl<T: Config> Pallet<T> {
	pub fn get_next_interpretation_type_id() -> Result<InterpretationTypeId, Error<T>> {
		// NOTE: Should we have a more sophisticated item ID generation algorithm?
		NextInterpretationTypeId::<T>::try_mutate(|id| {
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
}
