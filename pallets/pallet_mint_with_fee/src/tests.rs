use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn functional_mint_call() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::mint(
			Origin::root(),
			BOB,
			None,
			100_000,
			vec!(1, 2, 3)
		));

		assert_eq!(Balances::free_balance(BOB), 200_000);
	});
}

#[test]
fn functional_mint_with_fee_call() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::change_fee_percent(Origin::root(), 10));
		assert_ok!(MintWithFee::mint(
			Origin::root(),
			BOB,
			Some(CHARLIE),
			100_000,
			vec!(1, 2, 3)
		));

		assert_eq!(Balances::free_balance(BOB), 200_000);
		assert_eq!(Balances::free_balance(CHARLIE), 110_000);
	});
}

#[test]
fn mint_0_token() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::mint(
			Origin::root(),
			BOB,
			Some(CHARLIE),
			0,
			vec!(1, 2, 3)
		));

		assert_eq!(Balances::free_balance(BOB), 100_000);
		assert_eq!(Balances::free_balance(CHARLIE), 100_000);
	});
}

#[test]
fn mint_bad_origin() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_noop!(
			MintWithFee::mint(Origin::signed(1), BOB, None, 30, vec!(0)),
			BadOrigin
		);
	});
}

#[test]
fn mint_too_long_metadata() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_noop!(
			MintWithFee::mint(Origin::root(), BOB, None, 30, vec![0; 200]),
			Error::<Test>::TooLongMetadata
		);
	});
}

#[test]
fn change_fee_percent() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::change_fee_percent(Origin::root(), 30));

		assert_eq!(MintWithFee::fee_percent(), 30);
	});
}

#[test]
fn change_fee_bad_origin() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_noop!(
			MintWithFee::change_fee_percent(Origin::signed(1), 30),
			BadOrigin
		);
	});
}
