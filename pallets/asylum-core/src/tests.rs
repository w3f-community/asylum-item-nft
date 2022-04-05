use crate::{mock::*, Error};
use asylum_traits::{Change, IntepretationTypeInfo, Interpretation};
use frame_support::{
	assert_noop, assert_ok,
	traits::{tokens::nonfungibles::Inspect, Get},
	BoundedVec,
};
use rmrk_traits::{AccountIdOrCollectionNftTuple, ResourceInfo};

const TYPE_2D: &str = "2D";
const TYPE_3D: &str = "3D";

const PIXEL: &str = "pixel";
const COMICS: &str = "comics";
const ANIME: &str = "anime";

const TYPES: &'static [&str] = &[TYPE_2D, TYPE_3D];
const INTERPRETATIONS: &'static [&str] = &[PIXEL, COMICS, ANIME];

const MOCK_HASH: &str = "ipfs://hash";

fn bounded<T>(string: &str) -> BoundedVec<u8, T>
where
	T: Get<u32>,
{
	TryInto::<BoundedVec<u8, T>>::try_into(string.as_bytes().to_vec()).unwrap()
}

fn concat<T>(string1: &str, string2: &str) -> BoundedVec<u8, T>
where
	T: Get<u32>,
{
	bounded(&(string1.to_owned() + string2))
}

fn resource<T, V>(
	type_id: &str,
	id: &str,
	metadata: &str,
) -> ResourceInfo<BoundedVec<u8, T>, BoundedVec<u8, V>>
where
	T: Get<u32>,
	V: Get<u32>,
{
	let id = concat(type_id, id);
	let metadata = bounded(metadata);
	ResourceInfo {
		id,
		pending: false,
		base: None,
		src: Some(metadata.clone()),
		metadata: Some(metadata.clone()),
		parts: None,
		license: None,
		slot: None,
		thumb: None,
	}
}

fn create_types() {
	let name = bounded(TYPE_2D);
	let metadata = bounded(MOCK_HASH);
	assert_ok!(AsylumCore::create_interpretation_type(
		Origin::signed(ALICE),
		name.clone(),
		metadata.clone()
	));
	let name = bounded(TYPE_3D);
	assert_ok!(AsylumCore::create_interpretation_type(
		Origin::signed(ALICE),
		name.clone(),
		metadata.clone()
	));
}

fn create_template() {
	create_types();
	assert_ok!(AsylumCore::create_template(
		Origin::signed(ALICE),
		bounded("MyTemplate"),
		bounded(MOCK_HASH),
		None,
		vec![
			Interpretation {
				type_name: bounded(TYPE_2D),
				interpretations: vec![
					resource(TYPE_2D, PIXEL, MOCK_HASH),
					resource(TYPE_2D, COMICS, MOCK_HASH),
					resource(TYPE_2D, ANIME, MOCK_HASH)
				]
			},
			Interpretation {
				type_name: bounded(TYPE_3D),
				interpretations: vec![
					resource(TYPE_3D, PIXEL, MOCK_HASH),
					resource(TYPE_3D, COMICS, MOCK_HASH),
					resource(TYPE_3D, ANIME, MOCK_HASH)
				]
			}
		],
	));
}

fn mint_item_from_template() {
	assert_ok!(AsylumCore::mint_item_from_template(
		Origin::signed(ALICE),
		ALICE,
		ALICE,
		0,
		bounded(MOCK_HASH)
	));
}

#[test]
fn should_create_interpretation_type() {
	ExtBuilder::default().build().execute_with(|| {
		let name = bounded(TYPE_2D);
		let metadata = bounded(MOCK_HASH);
		assert_ok!(AsylumCore::create_interpretation_type(
			Origin::signed(ALICE),
			name.clone(),
			metadata.clone()
		));
		let id = AsylumCore::interpretation_type_id(name).unwrap();
		assert_eq!(
			AsylumCore::interpretation_type_info(id).unwrap(),
			IntepretationTypeInfo { metadata }
		);
	});
}

#[test]
fn should_fail_interpretation_type_1() {
	ExtBuilder::default().build().execute_with(|| {
		let name = bounded(TYPE_2D);
		let metadata = bounded(MOCK_HASH);
		assert_ok!(AsylumCore::create_interpretation_type(
			Origin::signed(ALICE),
			name.clone(),
			metadata.clone()
		));
		assert_noop!(
			AsylumCore::create_interpretation_type(
				Origin::signed(ALICE),
				name.clone(),
				metadata.clone()
			),
			Error::<Test>::InterpretationTypeAlreadyExist
		);
	});
}

#[test]
fn should_create_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		for type_name in TYPES {
			let type_id = AsylumCore::interpretation_type_id(bounded(type_name)).unwrap();
			for interpretation_id in INTERPRETATIONS {
				assert_eq!(
					AsylumCore::template_interpretations((
						0,
						type_id,
						concat(type_name, interpretation_id)
					)),
					Some(resource(type_name, interpretation_id, MOCK_HASH))
				);
			}
		}
		assert_eq!(Uniques::class_owner(&0), Some(ALICE));
	});
}

#[test]
fn should_fail_create_template_1() {
	ExtBuilder::default().build().execute_with(|| {
		create_types();
		assert_noop!(
			AsylumCore::create_template(
				Origin::signed(ALICE),
				bounded("MyTemplate"),
				bounded(MOCK_HASH),
				None,
				vec![Interpretation {
					type_name: bounded("SomeNonexistentType"),
					interpretations: vec![
						resource(TYPE_2D, PIXEL, MOCK_HASH),
						resource(TYPE_2D, COMICS, MOCK_HASH)
					]
				}]
			),
			Error::<Test>::InterpretationTypeNotExist
		);
	});
}

#[test]
fn should_destroy_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::destroy_template(Origin::signed(ALICE), 0));
	});
}

#[test]
fn should_fail_destroy_template_1() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(BOB), 0),
			pallet_uniques::Error::<Test>::NoPermission
		);
	});
}

#[test]
fn should_fail_destroy_template_2() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(ALICE), 0),
			pallet_rmrk_core::Error::<Test>::CollectionUnknown
		);
	});
}

#[test]
fn should_fail_destroy_template_3() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(ALICE), 0),
			pallet_rmrk_core::Error::<Test>::CollectionNotEmpty
		);
	});
}

#[test]
fn should_mint_item_from_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		for type_name in TYPES {
			let type_id = AsylumCore::interpretation_type_id(bounded(type_name)).unwrap();
			for interpretation_id in INTERPRETATIONS {
				let res_id = concat(type_name, interpretation_id);
				assert_eq!(AsylumCore::item_interpretations((0, 0, type_id, &res_id)), Some(()));
				assert_eq!(
					RmrkCore::resources((0, 0, &res_id)),
					Some(resource(type_name, interpretation_id, MOCK_HASH))
				);
			}
		}
		assert_eq!(AsylumCore::item_interpretations((0, 0, 0, concat(TYPE_2D, PIXEL))), Some(()));
	});
}

#[test]
fn should_transfer_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		for _ in 0..3 {
			mint_item_from_template();
		}

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB)
		));
		assert_eq!(
			RmrkCore::nfts(0, 0).unwrap().owner,
			AccountIdOrCollectionNftTuple::AccountId(BOB)
		);
		assert_eq!(Uniques::owner(0, 0), Some(BOB));

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(BOB),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		));
		assert_eq!(
			RmrkCore::nfts(0, 0).unwrap().owner,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		);
		assert_eq!(Uniques::owner(0, 0), Some(ALICE));

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB)
		));

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		));
		assert_eq!(
			RmrkCore::pending_nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		);
		assert_ok!(RmrkCore::accept_nft(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		));
		assert_eq!(RmrkCore::children((0, 0)), vec![(0, 1)]);

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
		));
		assert_ok!(RmrkCore::accept_nft(
			Origin::signed(BOB),
			0,
			2,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
		));
		assert_eq!(RmrkCore::children((0, 1)), vec![(0, 2)]);
		assert_eq!(
			RmrkCore::nfts(0, 2).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
		);
		assert_eq!(Uniques::owner(0, 2).unwrap(), RmrkCore::nft_to_account_id(0, 1)); // RMRK virtual adress

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		));
		assert_eq!(
			RmrkCore::nfts(0, 2).unwrap().owner,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
		);
		assert_eq!(Uniques::owner(0, 2).unwrap(), RmrkCore::nft_to_account_id(0, 1));
		assert_eq!(
			RmrkCore::nfts(0, 1).unwrap().owner,
			AccountIdOrCollectionNftTuple::AccountId(ALICE)
		);
		assert_eq!(Uniques::owner(0, 1), Some(ALICE));
	});
}

#[test]
fn should_fail_transfer_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		for _ in 0..2 {
			mint_item_from_template();
		}

		assert_noop!(
			AsylumCore::transfer_item(
				Origin::signed(ALICE),
				999,
				999,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
			),
			pallet_rmrk_core::Error::<Test>::NoAvailableNftId
		);

		assert_noop!(
			AsylumCore::transfer_item(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::AccountId(BOB)
			),
			pallet_rmrk_core::Error::<Test>::NoPermission
		);

		assert_noop!(
			AsylumCore::transfer_item(
				Origin::signed(ALICE),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
			),
			pallet_rmrk_core::Error::<Test>::CannotSendToDescendentOrSelf
		);

		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			0,
			AccountIdOrCollectionNftTuple::AccountId(BOB)
		));
		assert_ok!(AsylumCore::transfer_item(
			Origin::signed(ALICE),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		));
		assert_ok!(RmrkCore::accept_nft(
			Origin::signed(BOB),
			0,
			1,
			AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 0)
		));
		assert_noop!(
			AsylumCore::transfer_item(
				Origin::signed(BOB),
				0,
				0,
				AccountIdOrCollectionNftTuple::CollectionAndNftTuple(0, 1)
			),
			pallet_rmrk_core::Error::<Test>::CannotSendToDescendentOrSelf
		);
	});
}

#[test]
fn should_burn_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		mint_item_from_template();
		assert_ok!(AsylumCore::burn_item(Origin::signed(ALICE), 0, 0));
		for type_name in TYPES {
			let type_id = AsylumCore::interpretation_type_id(bounded(type_name)).unwrap();
			for interpretation_id in INTERPRETATIONS {
				assert_eq!(
					AsylumCore::item_interpretations((
						0,
						0,
						type_id,
						concat(type_name, interpretation_id)
					)),
					None
				);
			}
		}
		assert_eq!(RmrkCore::nfts(0, 1), None);
	});
}

#[test]
fn should_fail_burn_item_1() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_noop!(
			AsylumCore::burn_item(Origin::signed(ALICE), 0, 0),
			pallet_uniques::Error::<Test>::Unknown
		);
	});
}

#[test]
fn should_fail_burn_item_2() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AsylumCore::burn_item(Origin::signed(ALICE), 0, 0),
			pallet_rmrk_core::Error::<Test>::CollectionUnknown
		);
	});
}

#[test]
fn should_update_template_and_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::create_interpretation_type(
			Origin::signed(ALICE),
			bounded("NEW"),
			bounded(MOCK_HASH),
		));
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		let add_3D_type = Change::AddOrUpdate {
			interpretation_type: 1,
			interpretations: vec![resource(TYPE_3D, PIXEL, "updated_metadata")],
		};
		let modify_2D_type = Change::AddOrUpdate {
			interpretation_type: 2,
			interpretations: vec![resource("NEW", PIXEL, MOCK_HASH)],
		};
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![add_3D_type, modify_2D_type],
		));
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 0));
		assert_eq!(
			AsylumCore::template_interpretations((0, 1, concat(TYPE_3D, PIXEL))),
			Some(resource(TYPE_3D, PIXEL, "updated_metadata"))
		);
		assert_eq!(
			AsylumCore::template_interpretations((0, 2, concat("NEW", PIXEL))),
			Some(resource("NEW", PIXEL, MOCK_HASH))
		);

		assert_ok!(AsylumCore::update_item(Origin::signed(ALICE), 0, 0));
		assert_eq!(AsylumCore::item_interpretations((0, 0, 1, concat(TYPE_3D, PIXEL))), Some(()));
		assert_eq!(
			RmrkCore::resources((0, 0, concat(TYPE_3D, PIXEL))),
			Some(resource(TYPE_3D, PIXEL, "updated_metadata"))
		);
		assert_eq!(
			RmrkCore::resources((0, 0, concat("NEW", PIXEL))),
			Some(resource("NEW", PIXEL, MOCK_HASH))
		);

		let remove_2D_interpretation = Change::RemoveInterpretation {
			interpretation_type: 0,
			interpretation_id: concat(TYPE_2D, PIXEL),
		};
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![remove_2D_interpretation],
		));
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 1));
		assert_eq!(AsylumCore::template_interpretations((0, 0, concat(TYPE_2D, PIXEL))), None);

		let remove_3D_interpretation_type =
			Change::RemoveInterpretationType { interpretation_type: 1 };
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![remove_3D_interpretation_type],
		));
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 2));
		assert_ok!(AsylumCore::update_item(Origin::signed(ALICE), 0, 0));
		for i in INTERPRETATIONS {
			assert_eq!(AsylumCore::template_interpretations((0, 0, concat(TYPE_3D, i))), None);
			assert_eq!(RmrkCore::resources((0, 0, concat(TYPE_3D, i))), None);
		}
		assert_eq!(RmrkCore::resources((0, 0, concat(TYPE_2D, PIXEL))), None);
	});
}

#[test]
fn should_fail_update_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		let update = Change::AddOrUpdate {
			interpretation_type: 1,
			interpretations: vec![resource(TYPE_3D, PIXEL, "updated_metadata")],
		};
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![update],
		));
		assert_noop!(
			AsylumCore::update_template(Origin::signed(BOB), 0, 0),
			Error::<Test>::NoPermission
		);
	});
}