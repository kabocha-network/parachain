//! Benchmarking setup for pallet-mint-with-fee
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_support::{assert_ok, sp_runtime::traits::Saturating, BoundedVec};
use frame_system::RawOrigin;

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
		let metadata: BoundedVec<_, _> = vec![0; 99].try_into().unwrap();
		assert_ok!(Pallet::<T>::change_fee_percent(RawOrigin::Root.into(), 10u32.into()));
	}: _(RawOrigin::Root, bob.clone(), Some(charlie.clone()), i, metadata)
	verify {
		assert_eq!(T::Currency::free_balance(&bob), T::Currency::minimum_balance().saturating_mul(1000u32.into()));
		assert_eq!(T::Currency::free_balance(&charlie), T::Currency::minimum_balance().saturating_mul(100u32.into()));
	}

	change_fee_percent {
	}: _(RawOrigin::Root, 10u32.into())
	verify {

	}
}
