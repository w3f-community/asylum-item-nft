use asylum_traits::{
	primitives::{ItemId, ProposalId, TemplateId},
	*,
};
use frame_support::{ensure, traits::tokens::nonfungibles::Inspect};
use pallet_rmrk_core::StringLimitOf;
use rmrk_traits::Resource;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

use super::*;

impl<T: Config> Interpretable<StringLimitOf<T>, TagLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId> + pallet_rmrk_core::Config,
{
	fn interpretation_tag_create(
		tag: &TagLimitOf<T>,
		metadata: StringLimitOf<T>,
	) -> DispatchResult {
		ensure!(!Tags::<T>::contains_key(&tag), Error::<T>::TagAlreadyExists);
		let type_info = TagInfo { metadata };
		Tags::<T>::insert(&tag, type_info);
		Ok(())
	}
}

impl<T: Config> Item<T::AccountId, StringLimitOf<T>, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId> + pallet_rmrk_core::Config,
{
	fn item_mint_from_template(
		sender: T::AccountId,
		template_id: TemplateId,
		item_id: ItemId,
	) -> Result<(TemplateId, ItemId), DispatchError> {
		TemplateIntepretations::<T>::iter_prefix(template_id).try_for_each(|(interpretation_id, (IntepretationInfo { src, metadata, .. }, tags))| -> DispatchResult {
			ItemInterpretationTags::<T>::insert((template_id, item_id, &interpretation_id), tags);
			pallet_rmrk_core::Pallet::<T>::resource_add(
				sender.clone(),
				template_id,
				item_id,
				interpretation_id,
				None,
				src,
				metadata,
				None,
				None,
				None,
				None,
			)?;
			Ok(())
		})?;
		Ok((template_id, item_id))
	}

	fn item_burn(
		template_id: TemplateId,
		item_id: ItemId,
	) -> Result<(TemplateId, ItemId), DispatchError> {
		ItemInterpretationTags::<T>::remove_prefix((template_id, item_id), None);
		Ok((template_id, item_id))
	}

	fn item_accept_update(
		sender: T::AccountId,
		template_id: TemplateId,
		item_id: ItemId,
	) -> DispatchResult {
		ensure!(
			pallet_uniques::Pallet::<T>::owner(template_id, item_id) == Some(sender.clone()),
			Error::<T>::NoPermission
		);
		pallet_rmrk_core::Resources::<T>::iter_key_prefix((template_id, item_id)).try_for_each(
			|interpretation_id| -> DispatchResult {
				let interpretation = pallet_rmrk_core::Pallet::<T>::resources((
					template_id,
					item_id,
					&interpretation_id,
				))
				.unwrap();

				if interpretation.pending_removal {
					pallet_rmrk_core::Pallet::<T>::accept_removal(
						sender.clone(),
						template_id,
						item_id,
						interpretation_id,
					)?;
				} else if interpretation.pending {
					pallet_rmrk_core::Pallet::<T>::accept(
						sender.clone(),
						template_id,
						item_id,
						interpretation_id,
					)?;
				}
				Ok(())
			},
		)
	}
}

impl<T: Config>
	ItemTemplate<T::AccountId, StringLimitOf<T>, BoundedInterpretationOf<T>, TagLimitOf<T>>
	for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId> + pallet_rmrk_core::Config,
{
	fn template_create(
		template_id: TemplateId,
		interpretations: Vec<
			Interpretation<BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>,
		>,
	) -> Result<TemplateId, DispatchError> {
		interpretations.into_iter().try_for_each(
			|Interpretation { tags, interpretation }| -> DispatchResult {
				ensure!(!tags.is_empty(), Error::<T>::EmptyTags);
				tags.iter().try_for_each(|tag| -> DispatchResult {
					ensure!(Tags::<T>::contains_key(tag), Error::<T>::UnknownTag);
					Ok(())
				})?;
				TemplateIntepretations::<T>::insert(
					template_id,
					&interpretation.id,
					(&interpretation, tags),
				);
				Ok(())
			},
		)?;
		Ok(template_id)
	}

	fn template_update(
		sender: T::AccountId,
		proposal_id: ProposalId,
		template_id: TemplateId,
	) -> DispatchResult {
		ensure!(
			pallet_uniques::Pallet::<T>::class_owner(&template_id) == Some(sender.clone()),
			Error::<T>::NoPermission
		);
		let proposal_info = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExist)?;
		ensure!(proposal_info.state == ProposalState::Approved, Error::<T>::ProposalNotApproved);
		ensure!(
			proposal_info.template_id == template_id,
			Error::<T>::ProposalInappropriateTemplate
		);

		proposal_info
			.change_set
			.into_iter()
			.try_for_each(|change| Self::apply_changes(sender.clone(), template_id, change))?;
		Ok(())
	}

	fn template_destroy(template_id: TemplateId) -> Result<TemplateId, DispatchError> {
		TemplateIntepretations::<T>::remove_prefix(template_id, None);
		Ok(template_id)
	}
}

impl<T: Config> Proposal<T::AccountId, BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>
	for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = TemplateId, InstanceId = ItemId> + pallet_rmrk_core::Config,
{
	fn submit_proposal(
		author: T::AccountId,
		template_id: TemplateId,
		change_set: Vec<Change<BoundedInterpretationOf<T>, StringLimitOf<T>, TagLimitOf<T>>>,
	) -> Result<ProposalId, DispatchError> {
		let proposal_id = Self::get_next_proposal_id()?;
		let proposal_info =
			ProposalInfo { author, state: ProposalState::Approved, template_id, change_set };
		Proposals::<T>::insert(proposal_id, proposal_info);
		Ok(proposal_id)
	}
}
