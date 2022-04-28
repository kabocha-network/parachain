#![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use frame_support::{
	dispatch::DispatchResult,
	sp_runtime::traits::{CheckedConversion, CheckedDiv, CheckedMul},
	traits::Currency,
};
pub use frame_system::ensure_root;
pub use pallet::*;
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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// the value have been minted on the target accout
		/// [target_account, value, metadata]
		ValueMinted(T::AccountId, u128, Vec<u8>),
		/// the fees have been minted on the nsm account
		/// [nsp_account, value, metadata]
		FeeMinted(T::AccountId, u128, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
        /// the amount have a wrong format, causing an error when converting
        /// should only happen if the Currency is not a u128
		BadAmount,
        /// percentage is more than 100
        InvalidPercentage,
        /// Overflow
        Overflow,
        /// Too long metadata
        TooLongMetadata,
	}


	#[pallet::storage]
	#[pallet::getter(fn fee_percent)]
	pub type FeePercent<T: Config> = StorageValue<_, u8, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub fee_percent: u8,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			GenesisConfig { fee_percent: 10 }
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

    // T::x -> x est def dans la config
    // x::<T> -> est générique de T
    // x<T> -> est générique de T (mais dans la déclaration)
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn mint(
			origin: OriginFor<T>,
			target_account: T::AccountId,
			fee_target_account: Option<T::AccountId>,
			amount: u128,
			metadata: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;

            if metadata.len() > 100 {
                return Err(Error::<T>::TooLongMetadata.into());
            }

            if let Some(fee_target_account) = fee_target_account {
			    let fee_amount: u128 = amount.checked_mul(Self::fee_percent().into()).ok_or(Error::<T>::Overflow)? / 100;

			    Self::mint_to_account(&fee_target_account, fee_amount)?;

			    Self::deposit_event(Event::<T>::FeeMinted(
                    fee_target_account,
                    fee_amount,
                    metadata.clone(),
                ));
            };

			Self::mint_to_account(&target_account, amount)?;

			Self::deposit_event(Event::<T>::ValueMinted(
				target_account,
				amount,
				metadata,
			));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn change_fee_percent(origin: OriginFor<T>, percentage: u8) -> DispatchResult {
			ensure_root(origin)?;

            if percentage > 100 {
                return Err(Error::<T>::InvalidPercentage.into());
            }
			FeePercent::<T>::put(percentage);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn mint_to_account(target_account: &T::AccountId, amount: u128) -> Result<(), Error<T>> {
			let negative_amount_imbalance =
				T::Currency::issue(amount.checked_into().ok_or(Error::<T>::BadAmount)?);
			T::Currency::resolve_creating(&target_account, negative_amount_imbalance);

			Ok(())
		}
	}
}
