use super::*;
use crate::mock::*;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, Get},
	BoundedVec,
};
use sp_std::prelude::*;

fn tickets() -> Vec<(u64, u32, u32)> {
	let mut r: Vec<_> = Account::<Test>::iter().map(|x| x.0).collect();
	r.sort_unstable();
	let mut s: Vec<_> = Ticket::<Test>::iter().map(|x| (x.2.owner, x.0, x.1)).collect();
	s.sort_unstable();
	assert_eq!(r, s);
	for class in Ticket::<Test>::iter()
		.map(|x| x.0)
		.scan(None, |s, item| {
			if s.map_or(false, |last| last == item) {
				*s = Some(item);
				Some(None)
			} else {
				Some(Some(item))
			}
		})
		.flatten()
	{
		let details = Game::<Test>::get(class).unwrap();
		let instances = Ticket::<Test>::iter_prefix(class).count() as u32;
		assert_eq!(details.instances, instances);
	}
	r
}

fn games() -> Vec<(u64, u32)> {
	let mut r: Vec<_> = GameAccount::<Test>::iter().map(|x| (x.0, x.1)).collect();
	r.sort_unstable();
	let mut s: Vec<_> = Game::<Test>::iter().map(|x| (x.1.owner, x.0)).collect();
	s.sort_unstable();
	assert_eq!(r, s);
	r
}

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn attributes(class: u32) -> Vec<(Option<u32>, Vec<u8>, Vec<u8>)> {
	let mut s: Vec<_> = Attribute::<Test>::iter_prefix((class,))
		.map(|(k, v)| (k.0, k.1.into(), v.into()))
		.collect();
	s.sort();
	s
}

fn bounded<T>(string: &str) -> BoundedVec<u8, T>
where
	T: Get<u32>,
{
	TryInto::<BoundedVec<u8, T>>::try_into(string.as_bytes().to_vec()).unwrap()
}

#[test]
fn basic_setup_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(tickets(), vec![]);
	});
}

#[test]
fn basic_minting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_eq!(games(), vec![(1, 0)]);
		assert_ok!(GameDistribution::set_allow_unpriviledged_mint(Origin::signed(1), 0, true));
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 1),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
		Balances::make_free_balance_be(&2, 1001);
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 1));
		assert_eq!(tickets(), vec![(1, 0, 42)]);
		assert_eq!(Balances::free_balance(&1), 1000);
		assert_eq!(Balances::free_balance(&2), 1);

		assert_ok!(GameDistribution::create_game(Origin::signed(2), 1, vec![2], Some(1000)));
		assert_eq!(games(), vec![(1, 0), (2, 1)]);
		// free mint because game owner and minter are the same account
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 1, 69, 1));
		assert_eq!(tickets(), vec![(1, 0, 42), (1, 1, 69)]);
	});
}

#[test]
fn lifecycle_should_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_eq!(games(), vec![(1, 0)]);
		assert_ok!(GameDistribution::set_game_metadata(
			Origin::signed(1),
			0,
			bounded("ipfs://"),
			bounded("my game"),
			bounded("battle royal")
		));
		assert!(GameMetadataOf::<Test>::contains_key(0));

		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 10));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 69, 20));
		assert_eq!(tickets(), vec![(10, 0, 42), (20, 0, 69)]);
		assert_eq!(Game::<Test>::get(0).unwrap().instances, 2);
		assert_eq!(Game::<Test>::get(0).unwrap().instance_metadatas, 0);

		assert_ok!(GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 42, bvec![42, 42],));
		assert!(TicketMetadataOf::<Test>::contains_key(0, 42));
		assert_ok!(GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 69, bvec![69, 69],));
		assert!(TicketMetadataOf::<Test>::contains_key(0, 69));

		let w = Game::<Test>::get(0).unwrap().destroy_witness();
		assert_eq!(w.instances, 2);
		assert_eq!(w.instance_metadatas, 2);
		assert_ok!(GameDistribution::destroy_game(Origin::signed(1), 0, w));

		assert!(!Game::<Test>::contains_key(0));
		assert!(!Ticket::<Test>::contains_key(0, 42));
		assert!(!Ticket::<Test>::contains_key(0, 69));
		assert!(!GameMetadataOf::<Test>::contains_key(0));
		assert!(!TicketMetadataOf::<Test>::contains_key(0, 42));
		assert!(!TicketMetadataOf::<Test>::contains_key(0, 69));
		assert_eq!(games(), vec![]);
		assert_eq!(tickets(), vec![]);
	});
}

#[test]
fn destroy_with_bad_witness_should_not_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));

		let w = Game::<Test>::get(0).unwrap().destroy_witness();
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));
		assert_noop!(
			GameDistribution::destroy_game(Origin::signed(1), 0, w),
			Error::<Test>::BadWitness
		);
	});
}

#[test]
fn mint_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));

		Balances::make_free_balance_be(&2, 1001);
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(2), 0, 101, 2),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::set_allow_unpriviledged_mint(Origin::signed(2), 0, true),
			Error::<Test>::NoPermission
		);
		assert_ok!(GameDistribution::set_allow_unpriviledged_mint(Origin::signed(1), 0, true));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 101, 2));
		assert_eq!(GameDistribution::owner(0, 42).unwrap(), 1);
		assert_eq!(games(), vec![(1, 0)]);
		assert_eq!(tickets(), vec![(1, 0, 42), (2, 0, 101)]);
	});
}

#[test]
fn transfer_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 2));

		assert_ok!(GameDistribution::transfer(Origin::signed(2), 0, 42, 3));
		assert_eq!(tickets(), vec![(3, 0, 42)]);
		assert_noop!(
			GameDistribution::transfer(Origin::signed(2), 0, 42, 4),
			Error::<Test>::NoPermission
		);

		assert_ok!(GameDistribution::approve_transfer(Origin::signed(3), 0, 42, 2));
		assert_ok!(GameDistribution::transfer(Origin::signed(2), 0, 42, 4));
	});
}

#[test]
fn freezing_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));
		assert_ok!(GameDistribution::freeze_ticket(Origin::signed(1), 0, 42));
		assert_noop!(
			GameDistribution::transfer(Origin::signed(1), 0, 42, 2),
			Error::<Test>::Frozen
		);

		assert_ok!(GameDistribution::thaw_ticket(Origin::signed(1), 0, 42));
		assert_ok!(GameDistribution::freeze_game(Origin::signed(1), 0));
		assert_noop!(
			GameDistribution::transfer(Origin::signed(1), 0, 42, 2),
			Error::<Test>::Frozen
		);

		assert_ok!(GameDistribution::thaw_game(Origin::signed(1), 0));
		assert_ok!(GameDistribution::transfer(Origin::signed(1), 0, 42, 2));
	});
}

#[test]
fn origin_guards_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::set_allow_unpriviledged_mint(Origin::signed(1), 0, true));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));
		assert_noop!(
			GameDistribution::transfer_game_ownership(Origin::signed(2), 0, 2),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::set_game_team(Origin::signed(2), 0, vec![2], vec![2], vec![2]),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::freeze_ticket(Origin::signed(2), 0, 42),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::thaw_ticket(Origin::signed(2), 0, 42),
			Error::<Test>::NoPermission
		);
		Balances::make_free_balance_be(&2, 1001);
		// everybody can mint tickets
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 69, 2));
		assert_noop!(
			GameDistribution::burn_ticket(Origin::signed(2), 0, 42, None),
			Error::<Test>::NoPermission
		);
		let w = Game::<Test>::get(0).unwrap().destroy_witness();
		assert_noop!(
			GameDistribution::destroy_game(Origin::signed(2), 0, w),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn transfer_owner_should_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 100);
		Balances::make_free_balance_be(&3, 100);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(99)));
		assert_eq!(games(), vec![(1, 0)]);
		assert_ok!(GameDistribution::transfer_game_ownership(Origin::signed(1), 0, 2));
		assert_eq!(games(), vec![(2, 0)]);

		assert_noop!(
			GameDistribution::transfer_game_ownership(Origin::signed(1), 0, 1),
			Error::<Test>::NoPermission
		);

		// Mint and set metadata now and make sure that deposit gets transferred back.
		assert_ok!(GameDistribution::set_game_metadata(
			Origin::signed(2),
			0,
			bounded("ipfs://"),
			bounded("my game"),
			bounded("battle royal")
		));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));
		assert_ok!(GameDistribution::set_ticket_metadata(
			Origin::signed(2),
			0,
			42,
			bounded("ipfs://")
		));
		assert_ok!(GameDistribution::transfer_game_ownership(Origin::signed(2), 0, 3));
		assert_eq!(games(), vec![(3, 0)]);
	});
}

#[test]
fn set_team_should_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&2, 1001);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::set_game_team(
			Origin::signed(1),
			0,
			vec![2],
			vec![3],
			vec![4]
		));

		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 2));
		assert_ok!(GameDistribution::freeze_ticket(Origin::signed(4), 0, 42));
		assert_ok!(GameDistribution::thaw_ticket(Origin::signed(3), 0, 42));
		assert_ok!(GameDistribution::transfer(Origin::signed(3), 0, 42, 3));
		assert_ok!(GameDistribution::burn_ticket(Origin::signed(3), 0, 42, None));
	});
}

#[test]
fn set_game_metadata_should_work() {
	new_test_ext().execute_with(|| {
		// Cannot add metadata to unknown asset
		assert_noop!(
			GameDistribution::set_game_metadata(
				Origin::signed(1),
				0,
				bounded("ipfs://"),
				bounded("my game"),
				bounded("battle royal")
			),
			Error::<Test>::Unknown,
		);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		// Cannot add metadata to unowned asset
		assert_noop!(
			GameDistribution::set_game_metadata(
				Origin::signed(2),
				0,
				bounded("ipfs://"),
				bounded("my game"),
				bounded("battle royal")
			),
			Error::<Test>::NoPermission,
		);

		// Successfully add metadata
		assert_ok!(GameDistribution::set_game_metadata(
			Origin::signed(1),
			0,
			bounded("ipfs://new"),
			bounded("my game"),
			bounded("battle royal")
		));
		assert!(GameMetadataOf::<Test>::contains_key(0));

		assert_ok!(GameDistribution::set_game_metadata(
			Origin::signed(1),
			0,
			bounded("ipfs://"),
			bounded("my game"),
			bounded("battle royal")
		));

		assert_noop!(
			GameDistribution::clear_game_metadata(Origin::signed(2), 0),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::clear_game_metadata(Origin::signed(1), 1),
			Error::<Test>::Unknown
		);
		assert_ok!(GameDistribution::clear_game_metadata(Origin::signed(1), 0));
		assert!(!GameMetadataOf::<Test>::contains_key(0));
	});
}

#[test]
fn set_ticket_metadata_should_work() {
	new_test_ext().execute_with(|| {
		// Cannot add metadata to unknown asset
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 1));
		// Cannot add metadata to unowned asset
		assert_noop!(
			GameDistribution::set_ticket_metadata(Origin::signed(2), 0, 42, bvec![0u8; 20]),
			Error::<Test>::NoPermission,
		);

		// Successfully add metadata and take deposit
		assert_ok!(
			GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 42, bvec![0u8; 20],)
		);
		assert!(TicketMetadataOf::<Test>::contains_key(0, 42));

		assert_ok!(
			GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 42, bvec![0u8; 15],)
		);
		assert_ok!(
			GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 42, bvec![0u8; 25],)
		);

		// Clear Metadata
		assert_ok!(
			GameDistribution::set_ticket_metadata(Origin::signed(1), 0, 42, bvec![0u8; 15],)
		);
		assert_noop!(
			GameDistribution::clear_ticket_metadata(Origin::signed(2), 0, 42),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::clear_ticket_metadata(Origin::signed(1), 1, 42),
			Error::<Test>::Unknown
		);
		assert_ok!(GameDistribution::clear_ticket_metadata(Origin::signed(1), 0, 42));
		assert!(!TicketMetadataOf::<Test>::contains_key(0, 42));
	});
}

#[test]
fn set_attribute_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));

		assert_ok!(GameDistribution::set_attribute(Origin::signed(1), 0, None, bvec![0], bvec![0]));
		assert_ok!(GameDistribution::set_attribute(
			Origin::signed(1),
			0,
			Some(0),
			bvec![0],
			bvec![0]
		));
		assert_ok!(GameDistribution::set_attribute(
			Origin::signed(1),
			0,
			Some(0),
			bvec![1],
			bvec![0]
		));
		assert_eq!(
			attributes(0),
			vec![
				(None, bvec![0], bvec![0]),
				(Some(0), bvec![0], bvec![0]),
				(Some(0), bvec![1], bvec![0]),
			]
		);

		assert_ok!(GameDistribution::set_attribute(
			Origin::signed(1),
			0,
			None,
			bvec![0],
			bvec![0; 10]
		));
		assert_eq!(
			attributes(0),
			vec![
				(None, bvec![0], bvec![0; 10]),
				(Some(0), bvec![0], bvec![0]),
				(Some(0), bvec![1], bvec![0]),
			]
		);

		assert_ok!(GameDistribution::clear_attribute(Origin::signed(1), 0, Some(0), bvec![1]));
		assert_eq!(
			attributes(0),
			vec![(None, bvec![0], bvec![0; 10]), (Some(0), bvec![0], bvec![0]),]
		);

		let w = Game::<Test>::get(0).unwrap().destroy_witness();
		assert_ok!(GameDistribution::destroy_game(Origin::signed(1), 0, w));
		assert_eq!(attributes(0), vec![]);
	});
}

#[test]
fn burn_works() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 201);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(100)));
		assert_ok!(GameDistribution::set_game_team(
			Origin::signed(1),
			0,
			vec![2],
			vec![3],
			vec![4]
		));

		assert_noop!(
			GameDistribution::burn_ticket(Origin::signed(5), 0, 42, Some(5)),
			Error::<Test>::Unknown
		);
		assert_ok!(GameDistribution::set_allow_unpriviledged_mint(Origin::signed(1), 0, true));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 5));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 69, 5));

		assert_noop!(
			GameDistribution::burn_ticket(Origin::signed(0), 0, 42, None),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::burn_ticket(Origin::signed(5), 0, 42, Some(6)),
			Error::<Test>::WrongOwner
		);

		assert_ok!(GameDistribution::burn_ticket(Origin::signed(5), 0, 42, Some(5)));
		assert_ok!(GameDistribution::burn_ticket(Origin::signed(3), 0, 69, Some(5)));
	});
}

#[test]
fn approval_lifecycle_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 2));
		assert_ok!(GameDistribution::approve_transfer(Origin::signed(2), 0, 42, 3));
		assert_ok!(GameDistribution::transfer(Origin::signed(3), 0, 42, 4));
		assert_noop!(
			GameDistribution::transfer(Origin::signed(3), 0, 42, 3),
			Error::<Test>::NoPermission
		);
		assert!(Ticket::<Test>::get(0, 42).unwrap().approved.is_none());

		assert_ok!(GameDistribution::approve_transfer(Origin::signed(4), 0, 42, 2));
		assert_ok!(GameDistribution::transfer(Origin::signed(2), 0, 42, 2));
	});
}

#[test]
fn cancel_approval_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 2));

		assert_ok!(GameDistribution::approve_transfer(Origin::signed(2), 0, 42, 3));
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(2), 1, 42, None),
			Error::<Test>::Unknown
		);
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(2), 0, 43, None),
			Error::<Test>::Unknown
		);
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(3), 0, 42, None),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(2), 0, 42, Some(4)),
			Error::<Test>::WrongDelegate
		);

		assert_ok!(GameDistribution::cancel_approval(Origin::signed(2), 0, 42, Some(3)));
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(2), 0, 42, None),
			Error::<Test>::NoDelegate
		);
	});
}

#[test]
fn cancel_approval_works_with_admin() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 42, 2));

		assert_ok!(GameDistribution::approve_transfer(Origin::signed(2), 0, 42, 3));
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(1), 1, 42, None),
			Error::<Test>::Unknown
		);
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(1), 0, 43, None),
			Error::<Test>::Unknown
		);
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(1), 0, 42, Some(4)),
			Error::<Test>::WrongDelegate
		);

		assert_ok!(GameDistribution::cancel_approval(Origin::signed(1), 0, 42, Some(3)));
		assert_noop!(
			GameDistribution::cancel_approval(Origin::signed(1), 0, 42, None),
			Error::<Test>::NoDelegate
		);
	});
}

#[test]
fn templates_support() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&2, 1000);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(Uniques::create(Origin::signed(2), 101, 2));
		assert_ok!(Uniques::create(Origin::signed(2), 102, 2));
		assert_noop!(
			GameDistribution::add_template_support(Origin::signed(1), 0, 100),
			Error::<Test>::Unknown
		);
		assert_ok!(GameDistribution::add_template_support(Origin::signed(1), 0, 101));
		assert_ok!(GameDistribution::add_template_support(Origin::signed(1), 0, 102));
		assert_eq!(Game::<Test>::get(0).unwrap().templates, Some(BTreeSet::from([101, 102])));
		assert_ok!(GameDistribution::remove_template_support(Origin::signed(1), 0, 101));
		assert_ok!(GameDistribution::remove_template_support(Origin::signed(1), 0, 102));
		assert_eq!(Game::<Test>::get(0).unwrap().templates, Some(BTreeSet::default()));
	});
}

#[test]
fn assets_support() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&2, 1000);
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], Some(1000)));
		assert_ok!(Uniques::create(Origin::signed(2), 101, 2));
		assert_ok!(Uniques::create(Origin::signed(2), 102, 2));
		assert_noop!(
			GameDistribution::add_template_support(Origin::signed(1), 0, 100),
			Error::<Test>::Unknown
		);
		assert_ok!(GameDistribution::add_asset_support(Origin::signed(1), 0, 101));
		assert_ok!(GameDistribution::add_asset_support(Origin::signed(1), 0, 102));
		assert_eq!(Game::<Test>::get(0).unwrap().assets, Some(BTreeSet::from([101, 102])));
		assert_ok!(GameDistribution::remove_asset_support(Origin::signed(1), 0, 101));
		assert_ok!(GameDistribution::remove_asset_support(Origin::signed(1), 0, 102));
		assert_eq!(Game::<Test>::get(0).unwrap().assets, Some(BTreeSet::default()));
	});
}

#[test]
fn set_price() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![1], None));
		assert_ok!(GameDistribution::set_allow_unpriviledged_mint(Origin::signed(1), 0, true));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 2));
		assert_ok!(GameDistribution::set_price(Origin::signed(1), 0, 100));
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 2),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
		Balances::make_free_balance_be(&2, 101);
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 101, 2));
	});
}

#[test]
fn several_admins_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(GameDistribution::create_game(Origin::signed(1), 0, vec![2, 3], None));
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(4), 0, 42, 2),
			Error::<Test>::NoPermission
		);
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(2), 0, 42, 2));
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(3), 0, 101, 3));
		assert_ok!(GameDistribution::set_game_team(
			Origin::signed(1),
			0,
			vec![1],
			vec![2],
			vec![3]
		));
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(2), 0, 102, 2),
			Error::<Test>::NoPermission
		);
		assert_noop!(
			GameDistribution::mint_ticket(Origin::signed(3), 0, 103, 3),
			Error::<Test>::NoPermission
		);
		assert_ok!(GameDistribution::mint_ticket(Origin::signed(1), 0, 104, 1));
	});
}
