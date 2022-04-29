use super::mock::*;
// use crate::Error;
use frame_support::assert_ok;

#[test]
fn functional_mint_call() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::mint(Origin::root(), BOB, None, 100_000, vec!(1, 2, 3)));

		assert_eq!(Balances::free_balance(BOB), 200_000);
	});
}

#[test]
fn functional_mint_with_fee_call() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::change_fee_percent(Origin::root(), 10));
		assert_ok!(MintWithFee::mint(Origin::root(), BOB, Some(CHARLIE), 100_000, vec!(1, 2, 3)));


		assert_eq!(Balances::free_balance(BOB), 200_000);
		assert_eq!(Balances::free_balance(CHARLIE), 110_000);
	});
}

#[test]
fn mint_0_token() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::mint(Origin::root(), BOB, Some(CHARLIE), 0, vec!(1, 2, 3)));

		assert_eq!(Balances::free_balance(BOB), 100_000);
		assert_eq!(Balances::free_balance(CHARLIE), 100_000);
	});
}

#[test]
fn change_fee_percent() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(MintWithFee::change_fee_percent(Origin::root(), 30));

		assert_eq!(MintWithFee::fee_percent(), 30);
	});
}

// #[test]
// fn change_fee_percent_over_100() {
// 	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
// 		assert_noop!(
// 			MintWithFee::change_fee_percent(Origin::root(), 110),
// 			Error::<Test>::InvalidPercentage
// 		);
// 	});
// }

// #[test]
// fn oversized_metadata() {
// 	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
//         assert_ok!(MintWithFee::mint(Origin::root(), BOB, Some(CHARLIE), 0, vec!(1,2,3)));
//
//         assert_eq!(Balances::free_balance(BOB), 100_000_000);
//         assert_eq!(Balances::free_balance(CHARLIE), 100_000_000);
// 	});
// }

//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
