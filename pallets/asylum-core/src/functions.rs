use asylum_traits::{primitives::*, Change, IntepretationInfo};
use frame_support::{dispatch::DispatchResult, ensure};
use pallet_rmrk_core::StringLimitOf;
use rmrk_traits::Resource;
use sp_std::vec::Vec;

use super::*;

pub type ChangeOf<T> = Change<BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>;
pub type IntepretationInfoOf<T> = IntepretationInfo<BoundedInterpretationOf<T>, StringLimitOf<T>>;

impl<T: Config> Pallet<T>
where
	T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId> + pallet_rmrk_core::Config,
{
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
		template_id: TemplateId,
		change: ChangeOf<T>,
	) -> DispatchResult {
		match change {
			Change::Add { interpretations } =>
				Self::add_interpretation(sender, template_id, interpretations),
			Change::Modify { interpretations } =>
				Self::modify_interpretation(sender, template_id, interpretations),
			Change::ModifyTags { interpretation_id, tags } =>
				Self::modify_interpretation_tags(template_id, interpretation_id, tags),
			Change::RemoveInterpretation { interpretation_id } =>
				Self::remove_interpretation(sender, template_id, interpretation_id),
		}
	}

	pub fn add_interpretation(
		sender: T::AccountId,
		template_id: TemplateId,
		interpretations: Vec<(IntepretationInfoOf<T>, TagsOf<T>)>,
	) -> DispatchResult {
		interpretations
			.into_iter()
			.try_for_each(|(interpretation, tags)| -> DispatchResult {
				TemplateIntepretations::<T>::insert(
					template_id,
					&interpretation.id,
					(&interpretation, &tags),
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
						ItemInterpretationTags::<T>::insert((template_id, item_id, res.id), &tags);
						Ok(())
					},
				)?;
				Ok(())
			})
	}

	pub fn modify_interpretation(
		sender: T::AccountId,
		template_id: TemplateId,
		interpretations: Vec<IntepretationInfoOf<T>>,
	) -> DispatchResult {
		interpretations.iter().try_for_each(|interpretation| -> DispatchResult {
			ensure!(
				TemplateIntepretations::<T>::contains_key(template_id, &interpretation.id),
				Error::<T>::TemplateDoesntSupportThisInterpretation
			);
			Ok(())
		})?;
		interpretations.into_iter().try_for_each(|interpretation| -> DispatchResult {
			TemplateIntepretations::<T>::try_mutate(
				template_id,
				&interpretation.id,
				|value| -> DispatchResult {
					if let Some(inter) = value {
						inter.0 = interpretation.clone();
					}
					Ok(())
				},
			)?;
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
					Ok(())
				},
			)?;
			Ok(())
		})
	}

	pub fn modify_interpretation_tags(
		template_id: TemplateId,
		interpretation_id: BoundedInterpretationOf<T>,
		tags: TagsOf<T>,
	) -> DispatchResult {
		TemplateIntepretations::<T>::try_mutate(
			template_id,
			&interpretation_id,
			|value| -> DispatchResult {
				if let Some(inter) = value {
					inter.1 = tags.clone();
				}
				Ok(())
			},
		)?;
		pallet_rmrk_core::Nfts::<T>::iter_key_prefix(template_id).try_for_each(
			|item_id| -> DispatchResult {
				ItemInterpretationTags::<T>::try_mutate(
					(template_id, item_id, &interpretation_id),
					|interpretation_tags| -> DispatchResult {
						if let Some(inter) = interpretation_tags {
							*inter = tags.clone();
						}
						Ok(())
					},
				)?;
				Ok(())
			},
		)?;
		Ok(())
	}

	pub fn remove_interpretation(
		sender: T::AccountId,
		template_id: TemplateId,
		interpretation_id: BoundedInterpretationOf<T>,
	) -> DispatchResult {
		ensure!(
			TemplateIntepretations::<T>::contains_key(template_id, &interpretation_id),
			Error::<T>::TemplateDoesntSupportThisInterpretation
		);
		TemplateIntepretations::<T>::remove(template_id, &interpretation_id);
		pallet_rmrk_core::Nfts::<T>::iter_key_prefix(template_id).try_for_each(
			|item_id| -> DispatchResult {
				pallet_rmrk_core::Pallet::<T>::resource_remove(
					sender.clone(),
					template_id,
					item_id,
					interpretation_id.clone(),
				)?;
				ItemInterpretationTags::<T>::remove((template_id, item_id, &interpretation_id));
				Ok(())
			},
		)
	}
}
