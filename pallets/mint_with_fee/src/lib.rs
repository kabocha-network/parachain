#![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
pub use frame_support::{
        dispatch::DispatchResult,
        traits::Currency,
        sp_runtime::traits::{CheckedConversion, CheckedDiv},
    };
pub use frame_system::ensure_root;
pub use sp_std::prelude::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;
	}

    // type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    // type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// #[pallet::storage]
	// #[pallet::getter(fn something)]
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// the value have been minted on the target account,
        /// and the fee have been minted on the fee_target_account
		/// [target_account, value, fee_target_account, fee, metadata]
		ValueAndFeeMinted(T::AccountId, u128, T::AccountId, u128, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
        /// the amount have a wrong format, causing an error when converting
		BadAmount,
	}

    #[pallet::storage]
    #[pallet::getter(fn fee_percent)]
    pub type FeePercent<T: Config> = StorageValue<_, u8, ValueQuery>;

    #[pallet::genesis_config]
	pub struct GenesisConfig {
		pub fee_percent: u8
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
            GenesisConfig {
                fee_percent: 10,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
            if self.fee_percent > 100 {
                panic!("Percentage must be less than 100");
            }
            FeePercent::<T>::put(self.fee_percent);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        ///mint value to the target account and fee to the network_service_provider account
		#[pallet::weight(10_000)]
		pub fn mint_with_fee(
            origin: OriginFor<T>,
			target_account: T::AccountId,
			fee_target_account: T::AccountId,
            amount: u128,
            mut metadata: Vec<u8>
		) -> DispatchResult {
            ensure_root(origin)?;

            if metadata.len() > 100 {
                metadata.resize(100, ' ' as u8);
            }

            let fee_amount: u128 = if amount == 0 {
                0
            } else {
                amount * 100 / FeePercent::<T>::get() as u128
            };

            // mint value to the target account
            Self::mint_to_account(&target_account, amount)?;
            Self::mint_to_account(&fee_target_account, fee_amount)?;

            Self::deposit_event(
                Event::<T>::ValueAndFeeMinted(target_account, amount, fee_target_account, fee_amount, metadata)
            );

            Ok(())
		}

		#[pallet::weight(10_000)]
        /// change the value of the fee percent in storage
		pub fn change_fee_percent(
            origin: OriginFor<T>,
            percentage: u8
            ) -> DispatchResult {
            ensure_root(origin)?;

            FeePercent::<T>::put(percentage);
            Ok(())
        }
	}

    impl<T: Config> Pallet<T> {
        fn mint_to_account(target_account: &T::AccountId, amount: u128) -> Result<(), Error<T>> {
            let negative_amount_imbalance = T::Currency::issue(amount.checked_into().ok_or(Error::<T>::BadAmount)?);
            T::Currency::resolve_creating(&target_account, negative_amount_imbalance);

            Ok(())
        }
    }
}
