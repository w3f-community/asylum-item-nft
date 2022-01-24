use crate::{mock::*, Error, Pallet};
use asylum_traits::GameInfo;
use frame_support::{assert_noop, assert_ok};

type Games = Pallet<Test>;

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

#[test]
fn should_create_game() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_eq!(Games::games(0), GameInfo{ metadata: bvec![0u8; 9] });
	});
}

#[test]
fn should_destroy_game() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_ok!(Games::destroy_game(Origin::signed(ALICE), 0));
	});
}

#[test]
fn should_fail_destroy_game_1() {
	ExtBuilder::default().build().execute_with(|| {
        assert_noop!(Games::destroy_game(Origin::signed(ALICE), 0), Error::<Test>::GameNotExists);
	});
}

#[test]
fn should_bind_item() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_ok!(Games::bind_item(Origin::signed(ALICE), 1, 0));
	});
}

#[test]
fn should_fail_bind_item() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_noop!(Games::bind_item(Origin::signed(ALICE), 1, 1), Error::<Test>::GameNotExists);
	});
}

#[test]
fn should_unbind_item() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_ok!(Games::bind_item(Origin::signed(ALICE), 1, 0));
        assert_ok!(Games::unbind_item(Origin::signed(ALICE), 1, 0));
	});
}

#[test]
fn should_fail_unbind_item() {
	ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Games::create_game(Origin::signed(ALICE), ALICE, bvec![0u8; 9]));
        assert_noop!(Games::unbind_item(Origin::signed(ALICE), 1, 1), Error::<Test>::GameNotExists);
	});
}