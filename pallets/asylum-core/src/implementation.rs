use asylum_traits::primitives::{
	InterpretationId, InterpretationTypeId, ItemId, ItemTemplateId, ProposalId,
};
use asylum_traits::*;
use frame_support::{ensure, traits::tokens::nonfungibles::Transfer};
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::vec::Vec;

use super::*;

impl<T: Config> Interpretable<NameLimitOf<T>, MetadataLimitOf<T>> for Pallet<T> {
	fn interpretation_type_create(
		type_name: &NameLimitOf<T>,
		metadata: MetadataLimitOf<T>,
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
		interpretation_name: &NameLimitOf<T>,
		src: MetadataLimitOf<T>,
		metadata: MetadataLimitOf<T>,
	) -> Result<InterpretationId, DispatchError> {
		ensure!(
			!IntepretationNames::<T>::contains_key(&interpretation_name),
			Error::<T>::InterpretationAlreadyExist
		);
		let id = Self::get_next_interpretation_id()?;
		let info = IntepretationInfo { src, metadata };
		IntepretationNames::<T>::insert(interpretation_name, id);
		Intepretations::<T>::insert(id, info);
		Ok(id)
	}
}

impl<T: Config> Item<T::AccountId, NameLimitOf<T>, MetadataLimitOf<T>> for Pallet<T> {
	fn item_mint(
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		metadata: Option<MetadataLimitOf<T>>,
	) -> Result<(ItemTemplateId, ItemId), DispatchError> {
		let template_id = Self::get_template_id(template_name_or_id)?;
		let item_id = Self::get_next_item_id()?;
		let info = ItemInfo { metadata };
		Items::<T>::insert(template_id, item_id, info);
		for (type_id, interpretation_id) in TemplateIntepretations::<T>::iter_prefix(template_id) {
			ItemIntepretations::<T>::insert(
				(template_id, Some(item_id), type_id),
				interpretation_id,
			);
		}
		Ok((template_id, item_id))
	}

	fn item_burn(
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		item_id: ItemId,
	) -> Result<(ItemTemplateId, ItemId), DispatchError> {
		let template_id = Self::get_template_id(template_name_or_id)?;
		Items::<T>::remove(template_id, item_id);
		ItemIntepretations::<T>::remove_prefix((template_id, Some(item_id)), None);
		Ok((template_id, item_id))
	}

	fn item_update(
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		item_id: ItemId,
	) -> DispatchResult {
		let template_id = Self::get_template_id(template_name_or_id)?;
		let interpretation_types: Vec<_> =
			ItemIntepretations::<T>::iter_key_prefix((template_id, Some(item_id))).collect();
		for interpretation_type in interpretation_types {
			ItemIntepretations::<T>::try_mutate_exists(
				(template_id, Some(item_id), interpretation_type),
				|interpretations| -> DispatchResult {
					if !TemplateIntepretations::<T>::contains_key(template_id, interpretation_type)
					{
						*interpretations = None;
					} else {
						*interpretations =
							TemplateIntepretations::<T>::get(template_id, interpretation_type);
					}
					Ok(())
				},
			)?
		}
		Ok(())
	}

	fn item_transfer(
		destination: &T::AccountId,
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		item_id: ItemId,
	) -> DispatchResult {
		let template_id = Self::get_template_id(template_name_or_id)?;
		T::ItemNFT::transfer(&template_id, &item_id, destination)
	}
}

impl<T: Config> ItemTemplate<T::AccountId, NameLimitOf<T>, MetadataLimitOf<T>> for Pallet<T> {
	fn template_create(
		owner: T::AccountId,
		template_name: &NameLimitOf<T>,
		metadata: MetadataLimitOf<T>,
		interpretations: Vec<Interpretation<NameLimitOf<T>>>,
	) -> Result<ItemTemplateId, DispatchError> {
		ensure!(
			!TemplateNames::<T>::contains_key(&template_name),
			Error::<T>::TemplateAlreadyExist
		);
		let template_id = Self::get_next_template_id()?;
		let info = ItemTemplateInfo { owner, metadata };
		TemplateNames::<T>::insert(template_name, template_id);
		Templates::<T>::insert(template_id, info);
		for Interpretation { type_name, interpretation_names } in interpretations {
			let type_id = IntepretationTypeNames::<T>::get(type_name)
				.ok_or(Error::<T>::InterpretationTypeNotExist)?;
			let interpretations = interpretation_names
				.iter()
				.map(|name| {
					IntepretationNames::<T>::get(name).ok_or(Error::<T>::InterpretationNotExist)
				})
				.collect::<Result<Vec<_>, _>>()?;
			TemplateIntepretations::<T>::insert(template_id, type_id, interpretations);
		}
		Ok(template_id)
	}

	fn template_update(
		proposal_id: ProposalId,
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
	) -> DispatchResult {
		let template_id = Self::get_template_id(template_name_or_id)?;
		let proposal_info = Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotExist)?;
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

	fn template_change_issuer(
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		new_issuer: T::AccountId,
	) -> DispatchResult {
		let template_id = Self::get_template_id(template_name_or_id)?;
		Templates::<T>::try_mutate(template_id, |info| -> DispatchResult {
			if let Some(template_info) = info {
				template_info.owner = new_issuer;
			}
			Ok(())
		})
	}

	fn template_destroy(template_name: &NameLimitOf<T>) -> Result<ItemTemplateId, DispatchError> {
		let template_id =
			TemplateNames::<T>::take(template_name).ok_or(Error::<T>::TemplateNotExist)?;
		ensure!(Items::<T>::iter_prefix(template_id).count() == 0, Error::<T>::TemplateNotEmpty);
		Templates::<T>::remove(template_id);
		TemplateIntepretations::<T>::remove_prefix(template_id, None);
		Ok(template_id)
	}
}

impl<T: Config> Proposal<T::AccountId, NameLimitOf<T>> for Pallet<T> {
	fn submit_proposal(
		author: T::AccountId,
		template_name_or_id: NameOrId<NameLimitOf<T>, ItemTemplateId>,
		change_set: Vec<Change>,
	) -> Result<ProposalId, DispatchError> {
		let proposal_id = Self::get_next_proposal_id()?;
		let template_id = Self::get_template_id(template_name_or_id)?;
		let proposal_info =
			ProposalInfo { author, state: ProposalState::Approved, template_id, change_set };
		Proposals::<T>::insert(proposal_id, proposal_info);
		Ok(proposal_id)
	}
}
