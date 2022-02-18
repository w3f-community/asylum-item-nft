use crate::{mock::*, Error};
use asylum_traits::{Change, IntepretationInfo, IntepretationTypeInfo, Interpretation};
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};

fn bounded<T>(string: &str) -> BoundedVec<u8, T>
where
	T: Get<u32>,
{
	TryInto::<BoundedVec<u8, T>>::try_into(string.as_bytes().to_vec()).unwrap()
}

const INTERPRETATION_TYPE_2D: &str = "2D";
const INTERPRETATION_TYPE_3D: &str = "3D";

const INTERPRETATION_PIXEL: &str = "2D_pixel";
const INTERPRETATION_COMICS: &str = "2D_comics";
const INTERPRETATION_ANIME: &str = "2D_anime";

const MOCK_HASH: &str = "ipfs://hash";

fn create_types() {
	let name = bounded(INTERPRETATION_TYPE_2D);
	let metadata = bounded(MOCK_HASH);
	assert_ok!(AsylumCore::create_interpretation_type(
		Origin::signed(ALICE),
		name.clone(),
		metadata.clone()
	));
	let name = bounded(INTERPRETATION_TYPE_3D);
	assert_ok!(AsylumCore::create_interpretation_type(
		Origin::signed(ALICE),
		name.clone(),
		metadata.clone()
	));
}

fn create_interpretations() {
	create_types();
	let metadata = bounded(MOCK_HASH);
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_2D),
		bounded(INTERPRETATION_PIXEL),
		metadata.clone(),
		metadata.clone()
	));
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_2D),
		bounded(INTERPRETATION_COMICS),
		metadata.clone(),
		metadata.clone()
	));
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_2D),
		bounded(INTERPRETATION_ANIME),
		metadata.clone(),
		metadata.clone()
	));
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_3D),
		bounded(INTERPRETATION_PIXEL),
		metadata.clone(),
		metadata.clone()
	));
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_3D),
		bounded(INTERPRETATION_COMICS),
		metadata.clone(),
		metadata.clone()
	));
	assert_ok!(AsylumCore::create_interpretation(
		Origin::signed(ALICE),
		bounded(INTERPRETATION_TYPE_3D),
		bounded(INTERPRETATION_ANIME),
		metadata.clone(),
		metadata.clone()
	));
}

#[test]
fn should_create_interpretation_type() {
	ExtBuilder::default().build().execute_with(|| {
		let name = bounded(INTERPRETATION_TYPE_2D);
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
		let name = bounded(INTERPRETATION_TYPE_2D);
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
fn should_create_interpretation() {
	ExtBuilder::default().build().execute_with(|| {
		create_types();
		let name = bounded(INTERPRETATION_PIXEL);
		let metadata = bounded(MOCK_HASH);
		assert_ok!(AsylumCore::create_interpretation(
			Origin::signed(ALICE),
			bounded(INTERPRETATION_TYPE_2D),
			name.clone(),
			metadata.clone(),
			metadata.clone()
		));
		assert_eq!(
			AsylumCore::interpretation_info(0, 0).unwrap(),
			IntepretationInfo { name, src: metadata.clone(), metadata }
		);
	});
}

#[test]
fn should_fail_create_interpretation() {
	ExtBuilder::default().build().execute_with(|| {
		create_types();
		let name = bounded(INTERPRETATION_PIXEL);
		let metadata = bounded(MOCK_HASH);
		assert_noop!(
			AsylumCore::create_interpretation(
				Origin::signed(ALICE),
				bounded("SomeNonexistentType"),
				name.clone(),
				metadata.clone(),
				metadata.clone()
			),
			Error::<Test>::InterpretationTypeNotExist
		);
	});
}

#[test]
fn should_create_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_interpretations();
		assert_ok!(AsylumCore::create_template(
			Origin::signed(ALICE),
			bounded("MyTemplate"),
			bounded(MOCK_HASH),
			2,
			vec![Interpretation {
				type_name: bounded(INTERPRETATION_TYPE_2D),
				interpretation_ids: vec![0, 1]
			}]
		));
		assert_eq!(AsylumCore::template_interpretations(0, 0), Some(vec![0, 1]));
	});
}

#[test]
fn should_fail_create_template_1() {
	ExtBuilder::default().build().execute_with(|| {
		create_interpretations();
		assert_noop!(
			AsylumCore::create_template(
				Origin::signed(ALICE),
				bounded("MyTemplate"),
				bounded(MOCK_HASH),
				2,
				vec![Interpretation {
					type_name: bounded("SomeNonexistentType"),
					interpretation_ids: vec![0, 1]
				}]
			),
			Error::<Test>::InterpretationTypeNotExist
		);
	});
}

#[test]
fn should_fail_create_template_2() {
	ExtBuilder::default().build().execute_with(|| {
		create_interpretations();
		assert_noop!(
			AsylumCore::create_template(
				Origin::signed(ALICE),
				bounded("MyTemplate"),
				bounded(MOCK_HASH),
				2,
				vec![Interpretation {
					type_name: bounded(INTERPRETATION_TYPE_2D),
					interpretation_ids: vec![0, 1, 999]
				}]
			),
			Error::<Test>::InterpretationNotExist
		); // <---- TODO: add this check
	});
}

#[test]
fn should_mint_item_from_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_interpretations();
		let type_name = bounded(INTERPRETATION_TYPE_2D);
		assert_ok!(AsylumCore::create_template(
			Origin::signed(ALICE),
			bounded("MyTemplate"),
			bounded(MOCK_HASH),
			2,
			vec![Interpretation { type_name, interpretation_ids: vec![0, 1] }]
		));
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		assert_eq!(AsylumCore::item_interpretations((0, 0, 0)), Some(vec![0, 1]));
	});
}

#[test]
fn should_update_template_and_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_interpretations();
		let type_name = bounded(INTERPRETATION_TYPE_2D);
		assert_ok!(AsylumCore::create_template(
			Origin::signed(ALICE),
			bounded("MyTemplate"),
			bounded(MOCK_HASH),
			2,
			vec![Interpretation { type_name, interpretation_ids: vec![0, 1] }]
		));

		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		let add_3D_type = Change::Add { interpretation_type: 1, interpretation_ids: vec![1, 2] };
		let modify_2D_type = Change::Update { interpretation_type: 0, interpretation_ids: vec![0] };
		let change_set = vec![add_3D_type, modify_2D_type];
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			change_set
		));

		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 0));
		assert_eq!(AsylumCore::template_interpretations(0, 0), Some(vec![0]));
		assert_eq!(AsylumCore::template_interpretations(0, 1), Some(vec![1, 2]));

		assert_ok!(AsylumCore::update_item(Origin::signed(ALICE), 0, 0));
		assert_eq!(AsylumCore::item_interpretations((0, 0, 0)), Some(vec![0]));
		assert_eq!(AsylumCore::item_interpretations((0, 0, 1)), Some(vec![1, 2]));
	});
}
