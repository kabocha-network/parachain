//! # MintWithFee Pallet
//!
//! The MintWithFee pallet provide functionality to mint tokens with an optional fee,
//! that is a percentage of the total amount of tokens minted.
//!
//! ## Overview
//!
//! The MintWithFee pallet provide function for:
//!
//! - Minting tokens with an optional fee.
//! - Changing the percentage of the fee.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `set_fee` - Set the fee percentage.
//! - `mint` - Mint tokens with an optional fee.
//!
//! ## GenesisConfig
//!
//! The MintWithFee pallet requires a `GenesisConfig` to be set containing:
//!
//! - `fee_percent` - The percentage of the total amount of tokens minted that will be used as a
//!   fee.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::*;

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

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type WeightInfo: WeightInfo;
		type MaxMetadataSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// the value have been minted on the target accout
		/// [target_account, value, metadata]
		ValueMinted(T::AccountId, BalanceOf<T>, BoundedVec<u8, T::MaxMetadataSize>),
		/// the fees have been minted on the nsm account
		/// [nsp_account, value, metadata]
		FeeMinted(T::AccountId, BalanceOf<T>, BoundedVec<u8, T::MaxMetadataSize>),
		/// the percentage have been changed
		/// [new_percentage]
		FeeChanged(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Overflow
		Overflow,
		/// Too long metadata
		TooLongMetadata,
	}

	#[pallet::storage]
	/// Holds the percentage of the amount that will be minted on the fee account (if provided)
	#[pallet::getter(fn fee_percent)]
	pub type FeePercent<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub fee_percent: BalanceOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		/// Direct implementation of `GenesisBuild::assimilate_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
			<Self as GenesisBuild<T>>::assimilate_storage(self, storage)
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				fee_percent: 10u32.into(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			FeePercent::<T>::put(self.fee_percent);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mints the given amount of value on the target account,
		/// and mint a percent of the amount on the fee account, if provided
		///
		/// `mint` will increase the total issuance, and increase the amounts of the targets
		/// accounts.
		///
		/// The dispatch origin for this call must be `Signed` by the root.
		///
		/// # <weight>
		///
		/// Related functions:
		/// - `mint_to_account` can be one or two times, depending on if the fee account is provided
		///   or not.
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn mint(
			origin: OriginFor<T>,
			target_account: T::AccountId,
			fee_target_account: Option<T::AccountId>,
			#[pallet::compact] amount: BalanceOf<T>,
			metadata: BoundedVec<u8, T::MaxMetadataSize>,
		) -> DispatchResult {
			ensure_root(origin)?;

			if metadata.len() > 100 {
				return Err(Error::<T>::TooLongMetadata.into())
			}

			if let Some(fee_target_account) = fee_target_account {
				let fee_amount = (amount / 100u32.into())
					.checked_mul(&Self::fee_percent())
					.ok_or(Error::<T>::Overflow)?;

				Self::mint_to_account(&fee_target_account, fee_amount);

				Self::deposit_event(Event::<T>::FeeMinted(
					fee_target_account,
					fee_amount,
					metadata.clone(),
				));
			};

			Self::mint_to_account(&target_account, amount);

			Self::deposit_event(Event::<T>::ValueMinted(target_account, amount, metadata));

			Ok(())
		}

		/// Change the value of the fee percentage in storage
		///
		/// `set_fee` will change the value of the fee percentage in storage,
		/// affecting the next calls to `mint`
		///
		/// The dispatch origin for this call must be `Signed` by the root.
		#[pallet::weight(T::WeightInfo::change_fee_percent())]
		pub fn change_fee_percent(
			origin: OriginFor<T>,
			percentage: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			FeePercent::<T>::put(percentage);

			Self::deposit_event(Event::<T>::FeeChanged(percentage));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Mints the given amount on the target account
		///
		/// cannot fail
		fn mint_to_account(target_account: &T::AccountId, amount: BalanceOf<T>) {
			let negative_amount_imbalance = T::Currency::issue(amount);
			T::Currency::resolve_creating(target_account, negative_amount_imbalance);
		}
	}
}
