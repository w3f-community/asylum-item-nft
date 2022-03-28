use asylum_traits::{
	primitives::{InterpretationTypeId, ItemId, ItemTemplateId, ProposalId},
	*,
};
use frame_support::{ensure, traits::tokens::nonfungibles::Inspect};
use pallet_rmrk_core::StringLimitOf;
use rmrk_traits::{Resource, ResourceInfo};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

use super::*;

impl<T: Config> Interpretable<StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
		+ pallet_rmrk_core::Config,
{
	fn interpretation_type_create(
		type_name: &StringLimitOf<T>,
		metadata: StringLimitOf<T>,
	) -> Result<InterpretationTypeId, DispatchError> {
		ensure!(
			!IntepretationTypeNames::<T>::contains_key(&type_name),
			Error::<T>::InterpretationTypeAlreadyExist
		);
		let type_id = Self::get_next_interpretation_type_id()?;
		let type_info = IntepretationTypeInfo { metadata };
		IntepretationTypeNames::<T>::insert(type_name, type_id);
		IntepretationTypes::<T>::insert(type_id, type_info);
		Ok(type_id)
	}
}

impl<T: Config> Item<T::AccountId, StringLimitOf<T>, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
		+ pallet_rmrk_core::Config,
{
	fn item_mint_from_template(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError> {
		for (
			(type_id, _),
			ResourceInfo { id, pending: _, parts, base, src, metadata, slot, license, thumb },
		) in TemplateIntepretations::<T>::iter_prefix((template_id,))
		{
			pallet_rmrk_core::Pallet::<T>::resource_add(
				sender.clone(),
				template_id,
				item_id,
				id.clone(),
				base,
				src,
				metadata,
				slot,
				license,
				thumb,
				parts,
			)?;
			ItemIntepretations::<T>::insert((template_id, item_id, type_id, &id), ());
		}
		Ok((template_id, item_id))
	}

	fn item_burn(
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError> {
		ItemIntepretations::<T>::remove_prefix((template_id, item_id), None);
		Ok((template_id, item_id))
	}

	fn item_update(
		sender: T::AccountId,
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> DispatchResult {
		ensure!(
			pallet_uniques::Pallet::<T>::owner(template_id, item_id) == Some(sender.clone()),
			Error::<T>::NoPermission
		);

		let current_item_state: BTreeSet<_> =
			ItemIntepretations::<T>::iter_key_prefix((template_id, item_id)).collect();
		let current_template_state: BTreeSet<_> =
			TemplateIntepretations::<T>::iter_key_prefix((template_id,)).collect();
		let types_to_remove = current_item_state.difference(&current_template_state);
		for (type_id, interpretation_id) in types_to_remove {
			ItemIntepretations::<T>::remove((template_id, item_id, type_id, interpretation_id));
			pallet_rmrk_core::Pallet::<T>::resource_remove(
				sender.clone(),
				template_id,
				item_id,
				interpretation_id.clone(),
			)?;
		}
		for (type_id, interpretation_id) in current_template_state {
			let ResourceInfo { id, pending: _, parts, base, src, metadata, slot, license, thumb } =
				TemplateIntepretations::<T>::get((template_id, type_id, interpretation_id.clone()))
					.unwrap();
			pallet_rmrk_core::Pallet::<T>::resource_add(
				sender.clone(),
				template_id,
				item_id,
				id.clone(),
				base,
				src,
				metadata,
				slot,
				license,
				thumb,
				parts,
			)?;
			ItemIntepretations::<T>::insert((template_id, item_id, type_id, interpretation_id), ());
		}
		Ok(())
	}
}

impl<T: Config> ItemTemplate<T::AccountId, StringLimitOf<T>, BoundedInterpretation<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
		+ pallet_rmrk_core::Config,
{
	fn template_create(
		template_id: ItemTemplateId,
		interpretations: Vec<
			Interpretation<StringLimitOf<T>, BoundedInterpretation<T>, StringLimitOf<T>>,
		>,
	) -> Result<ItemTemplateId, DispatchError> {
		for Interpretation { type_name, interpretations } in interpretations {
			let type_id = IntepretationTypeNames::<T>::get(type_name)
				.ok_or(Error::<T>::InterpretationTypeNotExist)?;

			interpretations.into_iter().for_each(|interpretation| {
				TemplateIntepretations::<T>::insert(
					(template_id, type_id, interpretation.id.clone()),
					interpretation,
				);
			});
		}
		Ok(template_id)
	}

	fn template_update(
		sender: T::AccountId,
		proposal_id: ProposalId,
		template_id: ItemTemplateId,
	) -> DispatchResult {
		ensure!(
			pallet_uniques::Pallet::<T>::class_owner(&template_id) == Some(sender),
			Error::<T>::NoPermission
		);
		let proposal_info = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExist)?;
		ensure!(proposal_info.state == ProposalState::Approved, Error::<T>::ProposalNotApproved);
		ensure!(
			proposal_info.template_id == template_id,
			Error::<T>::ProposalInappropriateTemplate
		);

		for change in proposal_info.change_set {
			match change {
				Change::AddOrUpdate { interpretation_type, interpretations } => {
					interpretations.into_iter().for_each(|interpretation| {
						TemplateIntepretations::<T>::insert(
							(template_id, interpretation_type, interpretation.id.clone()),
							interpretation,
						);
					});
				},
				Change::RemoveInterpretation { interpretation_type, interpretation_id } => {
					ensure!(
						TemplateIntepretations::<T>::contains_key((
							template_id,
							interpretation_type,
							&interpretation_id
						)),
						Error::<T>::TemplateNotSupportThisType
					);
					TemplateIntepretations::<T>::remove((
						template_id,
						interpretation_type,
						interpretation_id,
					));
				},
				Change::RemoveInterpretationType { interpretation_type } => {
					TemplateIntepretations::<T>::remove_prefix(
						(template_id, interpretation_type),
						None,
					);
				},
			}
		}
		Ok(())
	}

	fn template_destroy(template_id: ItemTemplateId) -> Result<ItemTemplateId, DispatchError> {
		TemplateIntepretations::<T>::remove_prefix((template_id,), None);
		Ok(template_id)
	}
}

impl<T: Config> Proposal<T::AccountId, BoundedInterpretation<T>, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>
		+ pallet_rmrk_core::Config,
{
	fn submit_proposal(
		author: T::AccountId,
		template_id: ItemTemplateId,
		change_set: Vec<Change<BoundedInterpretation<T>, StringLimitOf<T>>>,
	) -> Result<ProposalId, DispatchError> {
		let proposal_id = Self::get_next_proposal_id()?;
		let proposal_info =
			ProposalInfo { author, state: ProposalState::Approved, template_id, change_set };
		Proposals::<T>::insert(proposal_id, proposal_info);
		Ok(proposal_id)
	}
}
