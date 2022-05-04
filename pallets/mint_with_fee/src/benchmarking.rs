//! Benchmarking setup for pallet-mint-with-fee
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_system::RawOrigin;
use frame_support::{
    sp_runtime::traits::Saturating,
    assert_ok,
};
use sp_std::vec;

use crate::Pallet;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

benchmarks! {
	mint {
        let bob: T::AccountId = get_account::<T>("BOB");
        let charlie: T::AccountId = get_account::<T>("CHARLIE");
        let i = T::Currency::minimum_balance().saturating_mul(1000u32.into());
        let metadata = vec![0; 100];
		assert_ok!(Pallet::<T>::change_fee_percent(RawOrigin::Root.into(), 10u32.into()));
	}: _(RawOrigin::Root, bob.clone(), Some(charlie.clone()), i.into(), metadata)
	verify {
        assert_eq!(T::Currency::free_balance(&bob), T::Currency::minimum_balance().saturating_mul(1000u32.into()));
        assert_eq!(T::Currency::free_balance(&charlie), T::Currency::minimum_balance().saturating_mul(100u32.into()));
	}

    change_fee_percent {
    }: _(RawOrigin::Root, 10u32.into())
    verify {

    }
}
