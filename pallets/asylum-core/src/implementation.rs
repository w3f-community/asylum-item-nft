use std::collections::BTreeSet;

use asylum_traits::primitives::{
	InterpretationId, InterpretationTypeId, ItemId, ItemTemplateId, ProposalId,
};
use asylum_traits::*;
use frame_support::ensure;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

use super::*;

impl<T: Config> Interpretable<StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>,
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

	fn interpretation_create(
		type_name: &StringLimitOf<T>,
		interpretation_name: StringLimitOf<T>,
		src: StringLimitOf<T>,
		metadata: StringLimitOf<T>,
	) -> Result<InterpretationId, DispatchError> {
		let type_id = Self::interpretation_type_id(type_name)
			.ok_or(Error::<T>::InterpretationTypeNotExist)?;
		let id = Self::get_next_interpretation_id()?;
		let info = IntepretationInfo { name: interpretation_name, src, metadata };
		Intepretations::<T>::insert(type_id, id, info);
		Ok(id)
	}
}

impl<T: Config> Item<T::AccountId, StringLimitOf<T>, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>,
{
	fn item_mint_from_template(
		template_id: ItemTemplateId,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError> {
		for (type_id, interpretation_id) in TemplateIntepretations::<T>::iter_prefix(template_id) {
			ItemIntepretations::<T>::insert((template_id, item_id, type_id), interpretation_id);
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

	fn item_update(template_id: ItemTemplateId, item_id: ItemId) -> DispatchResult {
		let current_item_state: BTreeSet<_> =
			ItemIntepretations::<T>::iter_key_prefix((template_id, item_id)).collect();
		let current_template_state: BTreeSet<_> =
			TemplateIntepretations::<T>::iter_key_prefix(template_id).collect();
		let types_to_remove = current_item_state.difference(&current_template_state);
		for interpretation_type in types_to_remove {
			ItemIntepretations::<T>::remove((template_id, item_id, interpretation_type));
		}
		for interpretation_type in current_template_state {
			ItemIntepretations::<T>::try_mutate(
				(template_id, item_id, interpretation_type),
				|interpretations| -> DispatchResult {
					*interpretations =
						TemplateIntepretations::<T>::get(template_id, interpretation_type);
					Ok(())
				},
			)?;
		}
		Ok(())
	}
}

impl<T: Config> ItemTemplate<T::AccountId, StringLimitOf<T>> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>,
{
	fn template_create(
		template_id: ItemTemplateId,
		interpretations: Vec<Interpretation<StringLimitOf<T>>>,
	) -> Result<ItemTemplateId, DispatchError> {
		for Interpretation { type_name, interpretation_ids } in interpretations {
			let type_id = IntepretationTypeNames::<T>::get(type_name)
				.ok_or(Error::<T>::InterpretationTypeNotExist)?;
			// maybe check for interpretation existance
			TemplateIntepretations::<T>::insert(template_id, type_id, interpretation_ids);
		}
		Ok(template_id)
	}

	fn template_update(proposal_id: ProposalId, template_id: ItemTemplateId) -> DispatchResult {
		let proposal_info = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExist)?;
		// TODO: check owner
		ensure!(proposal_info.state == ProposalState::Approved, Error::<T>::ProposalNotApproved);
		ensure!(
			proposal_info.template_id == template_id,
			Error::<T>::ProposalInappropriateTemplate
		);

		for change in proposal_info.change_set {
			match change {
				Change::Add { interpretation_type, interpretation_ids } => {
					ensure!(
						!TemplateIntepretations::<T>::contains_key(
							template_id,
							interpretation_type
						),
						Error::<T>::TemplateAlreadySupportThisType
					);
					TemplateIntepretations::<T>::insert(
						template_id,
						interpretation_type,
						interpretation_ids,
					);
				},
				Change::Update { interpretation_type, interpretation_ids } => {
					ensure!(
						TemplateIntepretations::<T>::contains_key(template_id, interpretation_type),
						Error::<T>::TemplateNotSupportThisType
					);
					TemplateIntepretations::<T>::try_mutate(
						template_id,
						interpretation_type,
						|interpretations| -> DispatchResult {
							if let Some(interpretation) = interpretations {
								*interpretation = interpretation_ids;
							}
							Ok(())
						},
					)?;
				},
				Change::Remove { interpretation_type } => {
					ensure!(
						TemplateIntepretations::<T>::contains_key(template_id, interpretation_type),
						Error::<T>::TemplateNotSupportThisType
					);
					TemplateIntepretations::<T>::remove(template_id, interpretation_type);
				},
			}
		}
		Ok(())
	}

	fn template_destroy(template_id: ItemTemplateId) -> Result<ItemTemplateId, DispatchError> {
		TemplateIntepretations::<T>::remove_prefix(template_id, None);
		Ok(template_id)
	}
}

impl<T: Config> Proposal<T::AccountId> for Pallet<T>
where
	T: pallet_uniques::Config<ClassId = ItemTemplateId, InstanceId = ItemId>,
{
	fn submit_proposal(
		author: T::AccountId,
		template_id: ItemTemplateId,
		change_set: Vec<Change>,
	) -> Result<ProposalId, DispatchError> {
		let proposal_id = Self::get_next_proposal_id()?;
		let proposal_info =
			ProposalInfo { author, state: ProposalState::Approved, template_id, change_set };
		Proposals::<T>::insert(proposal_id, proposal_info);
		Ok(proposal_id)
	}
}
