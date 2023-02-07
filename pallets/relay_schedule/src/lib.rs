#![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo, Weight};

use sp_runtime::sp_std::{boxed::Box, vec::Vec};

pub use frame_system::RawOrigin;
pub use sp_runtime::traits::Dispatchable;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Call: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>;
		type MaxBlockWeight: Get<Weight>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type CurrentBlock<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type AtBlockNumber<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub type Calls<T: Config> = StorageValue<_, Vec<<T as Config>::Call>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedCall(<T as Config>::Call),
		AtBlockNumberUpdated(u32),
		Dispatched(<T as Config>::Call, DispatchResult),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Execute the scheduled calls
		fn on_initialize(_now: T::BlockNumber) -> Weight {
			let at = AtBlockNumber::<T>::get();
			let now = CurrentBlock::<T>::get();
			let mut weight: Weight = 0;

			if now >= at {
				let calls = Calls::<T>::take();
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 0));
				// let calls = temp.as_mut();
				if let Some(mut calls) = calls {
					// let mut error_calls: Vec<<T as Config>::Call> = Vec::new();
					loop {
						let call = calls.pop();
						if let Some(call) = call {
							let call_weight = call.get_dispatch_info().weight;
							// let result = call.dispatch(frame_system::RawOrigin::Root.into());
							let (maybe_actual_call_weight, result) =
								match call.clone().dispatch(frame_system::RawOrigin::Root.into()) {
									Ok(post_info) => (post_info.actual_weight, Ok(())),
									Err(error_and_info) => (
										error_and_info.post_info.actual_weight,
										Err(error_and_info.error),
									),
								};
							let actual_call_weight =
								maybe_actual_call_weight.unwrap_or(call_weight);
							weight = weight.saturating_add(actual_call_weight);
							Self::deposit_event(Event::Dispatched(call, result));
							if weight >= T::MaxBlockWeight::get() / 100 {
								break
							}
						} else {
							break
						}
					}
					// calls = [calls, &error_calls].concat();
					Calls::<T>::put(calls);
					weight = weight.saturating_add(T::DbWeight::get().reads_writes(0, 1));
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
		pub fn set_at_block_number(origin: OriginFor<T>, block: u32) -> DispatchResult {
			ensure_root(origin)?;

			AtBlockNumber::<T>::put(block);
			Self::deposit_event(Event::<T>::AtBlockNumberUpdated(block));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn set_block_number(block: u32) -> DispatchResult {
			CurrentBlock::<T>::put(block);
			Ok(())
		}
	}
}
