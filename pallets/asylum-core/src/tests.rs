use crate::{mock::*, Error};
use asylum_traits::{Change, IntepretationInfo, Interpretation, TagInfo};
use frame_support::{
	assert_noop, assert_ok,
	traits::{tokens::nonfungibles::Inspect, Get},
	BoundedVec,
};
use rmrk_traits::{AccountIdOrCollectionNftTuple, ResourceInfo};
use sp_std::collections::btree_set::BTreeSet;

const PREFIX_2D: &str = "2D";
const PREFIX_3D: &str = "3D";
const TAG_WEAPON: &str = "weapon";
const TAG_DARK: &str = "dark";

const PIXEL: &str = "pixel";
const COMICS: &str = "comics";
const ANIME: &str = "anime";

const TAGS: &[&str] = &[TAG_WEAPON, TAG_DARK];
const PREFIX: &[&str] = &[PREFIX_2D, PREFIX_3D];
const INTERPRETATIONS: &[&str] = &[PIXEL, COMICS, ANIME];

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

fn interpretation<T, V>(
	tag: &str,
	id: &str,
	metadata: &str,
) -> IntepretationInfo<BoundedVec<u8, T>, BoundedVec<u8, V>>
where
	T: Get<u32>,
	V: Get<u32>,
{
	let id = concat(tag, id);
	let metadata = bounded(metadata);
	IntepretationInfo { id, src: Some(metadata.clone()), metadata: Some(metadata) }
}

fn create_tags() {
	for tag in TAGS {
		let name = bounded(tag);
		let metadata = bounded(MOCK_HASH);
		assert_ok!(AsylumCore::create_interpretation_tag(Origin::signed(ALICE), name, metadata));
	}
}

fn tags_set<T>(tags: &[&str]) -> BTreeSet<BoundedVec<u8, T>>
where
	T: Get<u32>,
{
	let mut set = BTreeSet::new();
	for tag in tags {
		set.insert(bounded(tag));
	}
	set
}

fn create_template() {
	create_tags();
	let tags = tags_set(&[TAG_WEAPON, TAG_DARK]);

	assert_ok!(AsylumCore::create_template(
		Origin::signed(ALICE),
		bounded("MyTemplate"),
		bounded(MOCK_HASH),
		None,
		vec![
			Interpretation {
				tags: tags.clone(),
				interpretation: interpretation(PREFIX_2D, PIXEL, MOCK_HASH),
			},
			Interpretation {
				tags: tags.clone(),
				interpretation: interpretation(PREFIX_2D, COMICS, MOCK_HASH),
			},
			Interpretation {
				tags: tags.clone(),
				interpretation: interpretation(PREFIX_2D, ANIME, MOCK_HASH),
			},
			Interpretation {
				tags: tags.clone(),
				interpretation: interpretation(PREFIX_3D, PIXEL, MOCK_HASH),
			},
			Interpretation {
				tags: tags.clone(),
				interpretation: interpretation(PREFIX_3D, COMICS, MOCK_HASH),
			},
			Interpretation { tags, interpretation: interpretation(PREFIX_3D, ANIME, MOCK_HASH) },
		],
	));
}

fn mint_item_from_template() {
	assert_ok!(AsylumCore::mint_item_from_template(
		Origin::signed(ALICE),
		ALICE,
		0,
		bounded(MOCK_HASH)
	));
}

fn to_resource<BoundedInterpretationId, BoundedString>(
	interpretation: IntepretationInfo<BoundedInterpretationId, BoundedString>,
) -> ResourceInfo<BoundedInterpretationId, BoundedString> {
	ResourceInfo {
		id: interpretation.id,
		pending: false,
		pending_removal: false,
		parts: None,
		base: None,
		src: interpretation.src,
		metadata: interpretation.metadata,
		slot: None,
		license: None,
		thumb: None,
	}
}

#[test]
fn should_create_interpretation_tag() {
	ExtBuilder::default().build().execute_with(|| {
		let name = bounded(PREFIX_2D);
		let metadata = bounded(MOCK_HASH);
		assert_ok!(AsylumCore::create_interpretation_tag(
			Origin::signed(ALICE),
			name.clone(),
			metadata.clone()
		));
		assert_eq!(AsylumCore::tags(name.clone()).unwrap(), TagInfo { metadata: metadata.clone() });
		assert_noop!(
			AsylumCore::create_interpretation_tag(Origin::signed(ALICE), name, metadata),
			Error::<Test>::TagAlreadyExists
		);
	});
}

#[test]
fn should_create_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		let tags = tags_set(&[TAG_WEAPON, TAG_DARK]);
		for tag in PREFIX {
			for interpretation_id in INTERPRETATIONS {
				assert_eq!(
					AsylumCore::template_interpretations(0, concat(tag, interpretation_id)),
					Some((interpretation(tag, interpretation_id, MOCK_HASH), tags.clone()))
				);
			}
		}
		assert_eq!(Uniques::class_owner(&0), Some(ALICE));

		assert_noop!(
			AsylumCore::create_template(
				Origin::signed(ALICE),
				bounded("MyTemplate"),
				bounded(MOCK_HASH),
				None,
				vec![Interpretation {
					tags: BTreeSet::new(),
					interpretation: interpretation(PREFIX_2D, PIXEL, MOCK_HASH),
				}]
			),
			Error::<Test>::EmptyTags
		);
	});
}

#[test]
fn should_destroy_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(BOB), 0),
			pallet_uniques::Error::<Test>::NoPermission
		);
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(ALICE), 0),
			pallet_rmrk_core::Error::<Test>::CollectionNotEmpty
		);
		assert_ok!(AsylumCore::burn_item(Origin::signed(ALICE), 0, 0));
		assert_ok!(AsylumCore::destroy_template(Origin::signed(ALICE), 0));
		assert_noop!(
			AsylumCore::destroy_template(Origin::signed(ALICE), 0),
			pallet_rmrk_core::Error::<Test>::CollectionUnknown
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
			0,
			bounded(MOCK_HASH)
		));
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			BOB,
			0,
			bounded(MOCK_HASH)
		));
		assert_noop!(
			AsylumCore::accept_item_update(Origin::signed(ALICE), 0, 1),
			Error::<Test>::NoPermission
		);
		assert_ok!(AsylumCore::accept_item_update(Origin::signed(BOB), 0, 1));

		let tags = tags_set(&[TAG_WEAPON, TAG_DARK]);

		for tag in PREFIX {
			for interpretation_id in INTERPRETATIONS {
				let res_id = concat(tag, interpretation_id);
				assert_eq!(
					AsylumCore::item_interpretation_tags((0, 0, &res_id)),
					Some(tags.clone())
				);
				assert_eq!(
					RmrkCore::resources((0, 0, &res_id)),
					Some(to_resource(interpretation(tag, interpretation_id, MOCK_HASH)))
				);
				assert_eq!(
					AsylumCore::item_interpretation_tags((0, 1, &res_id)),
					Some(tags.clone())
				);
				assert_eq!(
					RmrkCore::resources((0, 1, &res_id)),
					Some(to_resource(interpretation(tag, interpretation_id, MOCK_HASH)))
				);
			}
		}
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
		assert_noop!(
			AsylumCore::burn_item(Origin::signed(ALICE), 0, 0),
			pallet_rmrk_core::Error::<Test>::CollectionUnknown
		);
		create_template();
		mint_item_from_template();
		assert_ok!(AsylumCore::burn_item(Origin::signed(ALICE), 0, 0));
		for tag in PREFIX {
			for interpretation_id in INTERPRETATIONS {
				assert_eq!(
					AsylumCore::item_interpretation_tags((0, 0, concat(tag, interpretation_id))),
					None
				);
			}
		}
		assert_eq!(RmrkCore::nfts(0, 1), None);

		assert_noop!(
			AsylumCore::burn_item(Origin::signed(ALICE), 0, 0),
			pallet_uniques::Error::<Test>::Unknown
		);
	});
}

#[test]
fn should_update_template_and_item() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::create_interpretation_tag(
			Origin::signed(ALICE),
			bounded("NEW"),
			bounded(MOCK_HASH),
		));
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			ALICE,
			0,
			bounded(MOCK_HASH)
		));
		let default_tags = tags_set(&[TAG_WEAPON, TAG_DARK]);
		let modify_interpretation = Change::Modify {
			interpretations: vec![interpretation(PREFIX_3D, PIXEL, "updated_metadata")],
		};
		let new_tags = tags_set(&[PREFIX_3D, TAG_DARK]);
		let add_interpretation = Change::Add {
			interpretations: vec![(interpretation("NEW", PIXEL, MOCK_HASH), new_tags.clone())],
		};
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![modify_interpretation, add_interpretation],
		));
		assert_noop!(
			AsylumCore::update_template(Origin::signed(BOB), 0, 0),
			Error::<Test>::NoPermission
		);
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 0));
		assert_eq!(
			AsylumCore::template_interpretations(0, concat(PREFIX_3D, PIXEL)),
			Some((interpretation(PREFIX_3D, PIXEL, "updated_metadata"), default_tags.clone()))
		);
		assert_eq!(
			AsylumCore::template_interpretations(0, concat("NEW", PIXEL)),
			Some((interpretation("NEW", PIXEL, MOCK_HASH), new_tags.clone()))
		);

		assert_eq!(
			AsylumCore::item_interpretation_tags((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(default_tags)
		);
		assert_eq!(
			RmrkCore::resources((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(to_resource(interpretation(PREFIX_3D, PIXEL, "updated_metadata")))
		);
		assert_eq!(
			RmrkCore::resources((0, 0, concat("NEW", PIXEL))),
			Some(to_resource(interpretation("NEW", PIXEL, MOCK_HASH)))
		);

		let modify_tags = Change::ModifyTags {
			interpretation_id: concat(PREFIX_3D, PIXEL),
			tags: new_tags.clone(),
		};
		let remove_interpretation =
			Change::RemoveInterpretation { interpretation_id: concat(PREFIX_2D, PIXEL) };
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![remove_interpretation, modify_tags],
		));
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 1));
		assert_eq!(AsylumCore::template_interpretations(0, concat(PREFIX_2D, PIXEL)), None);
		assert_eq!(
			AsylumCore::item_interpretation_tags((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(new_tags)
		);
	});
}

#[test]
fn should_update_template_and_item_pending() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		assert_ok!(AsylumCore::create_interpretation_tag(
			Origin::signed(ALICE),
			bounded("NEW"),
			bounded(MOCK_HASH),
		));
		assert_ok!(AsylumCore::mint_item_from_template(
			Origin::signed(ALICE),
			BOB,
			0,
			bounded(MOCK_HASH)
		));
		let default_tags = tags_set(&[TAG_WEAPON, TAG_DARK]);
		let modify_interpretation = Change::Modify {
			interpretations: vec![interpretation(PREFIX_3D, PIXEL, "updated_metadata")],
		};
		let new_tags = tags_set(&[PREFIX_3D, TAG_DARK]);
		let add_interpretation = Change::Add {
			interpretations: vec![(interpretation("NEW", PIXEL, MOCK_HASH), new_tags.clone())],
		};
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![modify_interpretation, add_interpretation],
		));
		assert_noop!(
			AsylumCore::update_template(Origin::signed(BOB), 0, 0),
			Error::<Test>::NoPermission
		);
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 0));
		assert_eq!(
			AsylumCore::template_interpretations(0, concat(PREFIX_3D, PIXEL)),
			Some((interpretation(PREFIX_3D, PIXEL, "updated_metadata"), default_tags.clone()))
		);
		assert_eq!(
			AsylumCore::template_interpretations(0, concat("NEW", PIXEL)),
			Some((interpretation("NEW", PIXEL, MOCK_HASH), new_tags.clone()))
		);
		assert_noop!(
			AsylumCore::accept_item_update(Origin::signed(ALICE), 0, 0),
			Error::<Test>::NoPermission
		);
		assert_ok!(AsylumCore::accept_item_update(Origin::signed(BOB), 0, 0));
		assert_eq!(
			AsylumCore::item_interpretation_tags((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(default_tags)
		);
		assert_eq!(
			RmrkCore::resources((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(to_resource(interpretation(PREFIX_3D, PIXEL, "updated_metadata")))
		);
		assert_eq!(
			RmrkCore::resources((0, 0, concat("NEW", PIXEL))),
			Some(to_resource(interpretation("NEW", PIXEL, MOCK_HASH)))
		);

		let modify_tags = Change::ModifyTags {
			interpretation_id: concat(PREFIX_3D, PIXEL),
			tags: new_tags.clone(),
		};
		let remove_interpretation =
			Change::RemoveInterpretation { interpretation_id: concat(PREFIX_2D, PIXEL) };
		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![remove_interpretation, modify_tags],
		));
		assert_ok!(AsylumCore::update_template(Origin::signed(ALICE), 0, 1));
		assert_ok!(AsylumCore::accept_item_update(Origin::signed(BOB), 0, 0));
		assert_eq!(AsylumCore::template_interpretations(0, concat(PREFIX_2D, PIXEL)), None);
		assert_eq!(
			AsylumCore::item_interpretation_tags((0, 0, concat(PREFIX_3D, PIXEL))),
			Some(new_tags)
		);
	});
}

#[test]
fn should_fail_update_template() {
	ExtBuilder::default().build().execute_with(|| {
		create_template();
		let remove_interpretation =
			Change::RemoveInterpretation { interpretation_id: concat(PREFIX_3D, PIXEL) };
		let update_removed_interpretation = Change::Modify {
			interpretations: vec![interpretation(PREFIX_3D, PIXEL, "updated_metadata")],
		};

		assert_ok!(AsylumCore::submit_template_change_proposal(
			Origin::signed(ALICE),
			ALICE,
			0,
			vec![remove_interpretation, update_removed_interpretation],
		));
		assert_noop!(
			AsylumCore::update_template(Origin::signed(ALICE), 0, 0),
			Error::<Test>::TemplateDoesntSupportThisInterpretation
		);
	});
}
