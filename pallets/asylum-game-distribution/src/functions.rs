use super::*;
use frame_support::ensure;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::collections::btree_set::BTreeSet;

impl<T: Config> Pallet<T> {
	pub fn do_transfer(
		game: T::GameId,
		ticket: T::TicketId,
		dest: T::AccountId,
		with_details: impl FnOnce(&GameDetailsFor<T>, &mut TicketDetailsFor<T>) -> DispatchResult,
	) -> DispatchResult {
		let game_details = Game::<T>::get(&game).ok_or(Error::<T>::Unknown)?;
		ensure!(!game_details.is_frozen, Error::<T>::Frozen);

		let mut details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;
		ensure!(!details.is_frozen, Error::<T>::Frozen);
		with_details(&game_details, &mut details)?;

		Account::<T>::remove((&details.owner, &game, &ticket));
		Account::<T>::insert((&dest, &game, &ticket), ());
		let origin = details.owner;
		details.owner = dest;
		Ticket::<T>::insert(&game, &ticket, &details);

		Self::deposit_event(Event::TicketTransferred {
			game,
			ticket,
			from: origin,
			to: details.owner,
		});
		Ok(())
	}

	pub fn do_create_game(
		game: T::GameId,
		owner: T::AccountId,
		admin: T::AccountId,
		price: BalanceOf<T>,
		event: Event<T>,
	) -> DispatchResult {
		ensure!(!Game::<T>::contains_key(game), Error::<T>::InUse);

		Game::<T>::insert(
			game,
			GameDetails {
				owner: owner.clone(),
				issuer: admin.clone(),
				admin: admin.clone(),
				freezer: admin,
				price,
				instances: 0,
				instance_metadatas: 0,
				attributes: 0,
				is_frozen: false,
				templates: BTreeSet::new(),
			},
		);

		GameAccount::<T>::insert(&owner, &game, ());
		Self::deposit_event(event);
		Ok(())
	}

	pub fn do_destroy_game(
		game: T::GameId,
		witness: DestroyWitness,
		maybe_check_owner: Option<T::AccountId>,
	) -> Result<DestroyWitness, DispatchError> {
		Game::<T>::try_mutate_exists(game, |maybe_details| {
			let game_details = maybe_details.take().ok_or(Error::<T>::Unknown)?;
			if let Some(check_owner) = maybe_check_owner {
				ensure!(game_details.owner == check_owner, Error::<T>::NoPermission);
			}
			ensure!(game_details.instances == witness.instances, Error::<T>::BadWitness);
			ensure!(
				game_details.instance_metadatas == witness.instance_metadatas,
				Error::<T>::BadWitness
			);
			ensure!(game_details.attributes == witness.attributes, Error::<T>::BadWitness);

			for (instance, details) in Ticket::<T>::drain_prefix(&game) {
				Account::<T>::remove((&details.owner, &game, &instance));
			}
			TicketMetadataOf::<T>::remove_prefix(&game, None);
			GameMetadataOf::<T>::remove(&game);
			Attribute::<T>::remove_prefix((&game,), None);
			GameAccount::<T>::remove(&game_details.owner, &game);

			Self::deposit_event(Event::GameDestroyed { game });

			Ok(DestroyWitness {
				instances: game_details.instances,
				instance_metadatas: game_details.instance_metadatas,
				attributes: game_details.attributes,
			})
		})
	}

	pub fn do_mint_ticket(
		game: T::GameId,
		ticket: T::TicketId,
		owner: T::AccountId,
		with_details: impl FnOnce(&GameDetailsFor<T>) -> DispatchResult,
	) -> DispatchResult {
		ensure!(!Ticket::<T>::contains_key(game, ticket), Error::<T>::AlreadyExists);

		Game::<T>::try_mutate(&game, |maybe_game_details| -> DispatchResult {
			let game_details = maybe_game_details.as_mut().ok_or(Error::<T>::Unknown)?;

			with_details(game_details)?;

			let instances =
				game_details.instances.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			game_details.instances = instances;

			let owner = owner.clone();
			Account::<T>::insert((&owner, &game, &ticket), ());
			let details = TicketDetails { owner, approved: None, is_frozen: false };
			Ticket::<T>::insert(&game, &ticket, details);
			Ok(())
		})?;

		Self::deposit_event(Event::TicketIssued { game, ticket, owner });
		Ok(())
	}

	pub fn do_burn_ticket(
		game: T::GameId,
		ticket: T::TicketId,
		with_details: impl FnOnce(&GameDetailsFor<T>, &TicketDetailsFor<T>) -> DispatchResult,
	) -> DispatchResult {
		let owner = Game::<T>::try_mutate(
			&game,
			|maybe_class_details| -> Result<T::AccountId, DispatchError> {
				let game_details = maybe_class_details.as_mut().ok_or(Error::<T>::Unknown)?;
				let details = Ticket::<T>::get(&game, &ticket).ok_or(Error::<T>::Unknown)?;
				with_details(game_details, &details)?;

				game_details.instances.saturating_dec();
				Ok(details.owner)
			},
		)?;

		Ticket::<T>::remove(&game, &ticket);
		Account::<T>::remove((&owner, &game, &ticket));

		Self::deposit_event(Event::TicketBurned { game, ticket, owner });
		Ok(())
	}
}
