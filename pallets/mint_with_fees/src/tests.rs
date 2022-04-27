use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
        // assert_ok!(MintCurrency::proposal_value_mint(Origin::signed(0), 0, ));
		// Dispatch a signed extrinsic.
		// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));
	});
}
//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
