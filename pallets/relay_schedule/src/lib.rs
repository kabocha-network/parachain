#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::weights::{GetDispatchInfo, PostDispatchInfo};

use sp_runtime::sp_std::{boxed::Box, vec::Vec};

pub use sp_runtime::traits::Dispatchable;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Call: Parameter
			+ Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>;
		type AtBlockNumber: Get<u32>;
		type MaxBlockWeight: Get<Weight>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type CurrentBlock<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub type Calls<T: Config> = StorageValue<_, Vec<<T as Config>::Call>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedCall(<T as Config>::Call),
		BlockNumberUp(u32),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Execute the scheduled calls
		fn on_initialize(_now: T::BlockNumber) -> Weight {
			let at = T::AtBlockNumber::get();
			let now = CurrentBlock::<T>::get();
			let mut weight = 0;

			if now >= at {
				let calls = Calls::<T>::take();
				// let calls = temp.as_mut();
				if let Some(mut calls) = calls {
					// let mut error_calls: Vec<<T as Config>::Call> = Vec::new();
					loop {
						let call = calls.pop();
						if let Some(call) = call {
							let result = call
								.dispatch(frame_system::RawOrigin::Root.into())
								.map(|res| res.actual_weight.unwrap_or(0))
								.map_err(|e| e.error);
							if result.is_err() {
								// error_calls.push(call);
							} else {
								weight += result.unwrap();
								if weight >= T::MaxBlockWeight::get() {
									break;
								}
							}
						} else {
							break
						}
					}
					// calls = [calls, &error_calls].concat();
					Calls::<T>::put(calls)
				}
			}
			weight
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn schedule(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> DispatchResult {
			ensure_root(origin)?;

			let mut new_calls: Vec<<T as Config>::Call> = Vec::new();
			let calls = Calls::<T>::take();

			if let Some(calls) = calls {
				new_calls = calls;
			}
			new_calls.push(*call.clone());

			Calls::<T>::put(new_calls);

			Self::deposit_event(Event::<T>::AddedCall(*call));
			Ok(())
		}

		#[pallet::weight(T::DbWeight::get().writes(1))]
		pub fn set_block_number(origin: OriginFor<T>, block: u32) -> DispatchResult {
			// ensure_none(origin)?;

			CurrentBlock::<T>::put(block);
			Self::deposit_event(Event::<T>::BlockNumberUp(block));
			Ok(())
		}
	}
}
