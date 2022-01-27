use crate::mock::*;
use asylum_traits::{GameInfo, ItemInfo};
use frame_support::assert_ok;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn initialize_collections() {
	assert_ok!(AsylumCore::initialize_games_collection(Origin::signed(ALICE), ALICE, ALICE));
	assert_ok!(AsylumCore::initialize_items_collection(Origin::signed(ALICE), ALICE, ALICE));
}

#[test]
fn should_mint_item() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_item(Origin::signed(ALICE), BOB, Some(bvec![0u8; 9])));
		assert_eq!(AsylumCore::items(0), ItemInfo { metadata: Some(bvec![0u8; 9]) });
	});
}

#[test]
fn should_burn_item() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_item(Origin::signed(ALICE), BOB, Some(bvec![0u8; 9])));
		assert_ok!(AsylumCore::burn_item(Origin::signed(ALICE), 0));
	});
}

#[test]
fn should_transfer_item() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_item(Origin::signed(ALICE), BOB, Some(bvec![0u8; 9])));
		assert_ok!(AsylumCore::transfer_item(Origin::signed(BOB), 0, CHARLIE));
		assert_eq!(Uniques::owner(1, 0), Some(CHARLIE));
	});
}

#[test]
fn should_mint_game() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_game(Origin::signed(ALICE), BOB, bvec![0u8; 9]));
		assert_eq!(AsylumCore::games(0), GameInfo { metadata: bvec![0u8; 9] });
	});
}

#[test]
fn should_burn_game() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_game(Origin::signed(ALICE), BOB, bvec![0u8; 9]));
		assert_ok!(AsylumCore::burn_game(Origin::signed(ALICE), 0));
	});
}

#[test]
fn should_transfer_game() {
	ExtBuilder::default().build().execute_with(|| {
		initialize_collections();
		assert_ok!(AsylumCore::mint_game(Origin::signed(ALICE), BOB, bvec![0u8; 9]));
		assert_ok!(AsylumCore::transfer_game(Origin::signed(BOB), 0, CHARLIE));
		assert_eq!(Uniques::owner(2, 0), Some(CHARLIE));
	});
}
