use asylum_traits::{primitives::*, Change, IntepretationInfo};
use frame_support::{dispatch::DispatchResult, ensure};
use pallet_rmrk_core::StringLimitOf;
use rmrk_traits::Resource;
use sp_std::vec::Vec;

use super::*;

pub type ChangeOf<T> = Change<BoundedResourceOf<T>, StringLimitOf<T>>;
pub type IntepretationInfoOf<T> = IntepretationInfo<BoundedResourceOf<T>, StringLimitOf<T>>;

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
		+ pallet_rmrk_core::Config,
{
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

	pub fn apply_changes(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		change: ChangeOf<T>,
	) -> DispatchResult {
		match change {
			Change::Add { interpretation_type, interpretations } =>
				Self::add_or_modify_interpretation(
					sender,
					template_id,
					interpretation_type,
					interpretations,
				),
			Change::Modify { interpretation_type, interpretations } => {
				interpretations.iter().try_for_each(|interpretation| -> DispatchResult {
					ensure!(
						TemplateIntepretations::<T>::contains_key((
							template_id,
							interpretation_type,
							&interpretation.id
						)),
						Error::<T>::TemplateDoesntSupportThisInterpretations
					);
					Ok(())
				})?;
				Self::add_or_modify_interpretation(
					sender,
					template_id,
					interpretation_type,
					interpretations,
				)
			},
			Change::RemoveInterpretation { interpretation_type, interpretation_id } =>
				Self::remove_interpretation(
					sender,
					template_id,
					interpretation_type,
					interpretation_id,
				),
			Change::RemoveInterpretationType { interpretation_type } =>
				Self::remove_interpretation_type(sender, template_id, interpretation_type),
		}
	}

	pub fn add_or_modify_interpretation(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		interpretation_type: InterpretationTypeId,
		interpretations: Vec<IntepretationInfoOf<T>>,
	) -> DispatchResult {
		interpretations.into_iter().try_for_each(|interpretation| -> DispatchResult {
			TemplateIntepretations::<T>::insert(
				(template_id, interpretation_type, interpretation.id.clone()),
				interpretation.clone(),
			);
			pallet_rmrk_core::Nfts::<T>::iter_key_prefix(template_id).try_for_each(
				|item_id| -> DispatchResult {
					let res = interpretation.clone();
					pallet_rmrk_core::Pallet::<T>::resource_add(
						sender.clone(),
						template_id,
						item_id,
						res.id.clone(),
						None,
						res.src,
						res.metadata,
						None,
						None,
						None,
						None,
					)?;
					ItemIntepretations::<T>::insert(
						(template_id, item_id, interpretation_type, res.id),
						(),
					);
					Ok(())
				},
			)?;
			Ok(())
		})
	}

	pub fn remove_interpretation(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		interpretation_type: InterpretationTypeId,
		interpretation_id: BoundedResourceOf<T>,
	) -> DispatchResult {
		ensure!(
			TemplateIntepretations::<T>::contains_key((
				template_id,
				interpretation_type,
				&interpretation_id
			)),
			Error::<T>::TemplateDoesntSupportThisType
		);
		TemplateIntepretations::<T>::remove((
			template_id,
			interpretation_type,
			interpretation_id.clone(),
		));
		pallet_rmrk_core::Nfts::<T>::iter_key_prefix(template_id).try_for_each(
			|item_id| -> DispatchResult {
				pallet_rmrk_core::Pallet::<T>::resource_remove(
					sender.clone(),
					template_id,
					item_id,
					interpretation_id.clone(),
				)?;
				ItemIntepretations::<T>::remove((
					template_id,
					item_id,
					interpretation_type,
					&interpretation_id,
				));
				Ok(())
			},
		)
	}

	pub fn remove_interpretation_type(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		interpretation_type: InterpretationTypeId,
	) -> DispatchResult {
		TemplateIntepretations::<T>::iter_key_prefix((template_id,)).try_for_each(
			|(interpretation_type, interpretation_id)| -> DispatchResult {
				Self::remove_interpretation(
					sender.clone(),
					template_id,
					interpretation_type,
					interpretation_id,
				)?;
				Ok(())
			},
		)?;
		TemplateIntepretations::<T>::remove_prefix((template_id, interpretation_type), None);
		Ok(())
	}
}
