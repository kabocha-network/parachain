//
// this is some complex documentation
//
// yeah
//

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
pub mod mock_weights;
pub use mock_weights::*;

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
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>;
		type MaxBlockWeight: Get<Weight>;
		type WeightInfo: WeightInfo;
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
	pub type Calls<T: Config> = StorageValue<_, Vec<<T as Config>::RuntimeCall>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedCall(<T as Config>::RuntimeCall),
		AtBlockNumberUpdated(u32),
		Dispatched(<T as Config>::RuntimeCall, DispatchResult),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Execute the scheduled calls
		fn on_initialize(_now: T::BlockNumber) -> Weight {
			let at = AtBlockNumber::<T>::get();
			let now = CurrentBlock::<T>::get();
			let mut weight: Weight = Weight::zero();
			let maximum_weight =
				T::MaxBlockWeight::get().checked_div(100).unwrap_or(Weight::zero());

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
							if weight.all_gt(maximum_weight) {
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
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::schedule())]
		pub fn schedule(
			origin: OriginFor<T>,
			call: Box<<T as Config>::RuntimeCall>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let mut new_calls: Vec<<T as Config>::RuntimeCall> = Vec::new();
			let calls = Calls::<T>::take();

			if let Some(calls) = calls {
				new_calls = calls;
			}
			new_calls.push(*call.clone());

			Calls::<T>::put(new_calls);

			Self::deposit_event(Event::<T>::AddedCall(*call));
			Ok(())
		}
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::set_at_block_number())]
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
