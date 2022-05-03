//! Benchmarking setup for pallet-mint-with-fee

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as MintWithFee;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::vec;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

benchmarks! {
	mint {
        let bob: T::AccountId = get_account::<T>("BOB");
        let charlie: T::AccountId = get_account::<T>("CHARLIE");
		let i in 100_000 .. 1_000_000;
	}: _(RawOrigin::Root, bob.clone(), Some(charlie.clone()), i.into(), vec!(0))
	verify {

	}
}

// impl_benchmark_test_suite!(Capsule, crate::tests::mock::new_test_ext(), crate::tests::mock::Test);
// impl_benchmark_test_suite!(MintWithFee, mock::ExtBuilder::default().balances(vec![]).build(), Test);
