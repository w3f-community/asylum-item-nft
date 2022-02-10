use crate::{mock::*, Error};
use asylum_traits::IntepretationTypeInfo;
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};

macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

fn bounded<T>(string: &str) -> BoundedVec<u8, T>
where
	T: Get<u32>,
{
	TryInto::<BoundedVec<u8, T>>::try_into(string.as_bytes().to_vec()).unwrap()
}

const INTERPRETATION_TYPE_NAME_1: &str = "2D";
const TEMPLATE_NAME_1: &str = "Sword";
const TEMPLATE_NAME_2: &str = "Staff";
const TEMPLATE_NAME_3: &str = "Rags";
const MOCK_METADATA: &str = "ipfs://hash";

#[test]
fn should_create_interpretation_type() {
	ExtBuilder::default().build().execute_with(|| {
		let name = bounded(INTERPRETATION_TYPE_NAME_1);
		let metadata = bounded(MOCK_METADATA);
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
		let name = bounded(INTERPRETATION_TYPE_NAME_1);
		let metadata = bounded(MOCK_METADATA);
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