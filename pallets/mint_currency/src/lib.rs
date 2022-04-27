#![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
pub use frame_support::{
        dispatch::DispatchResult, pallet_prelude::*,
        traits::Currency,
        sp_runtime::traits::CheckedConversion
    };
pub use frame_system::{
        pallet_prelude::*,
        ensure_root,
    };

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

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
		/// the proposal value have been minted on the target account
		/// [proposal_id, target_account, value]
		ValueMinted(u32, T::AccountId, u128),
		/// the proposal value have been minted on the target account
		/// [proposal_id, target_account, value]
		FeesMinted(u32, T::AccountId, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proposal have not been yet accepted
		ProposalNotAccepted,
        /// There is not enough funds in the treasury to pay the proposal
        NotEnoughFunds,
        /// Overflow when adding the value to the account
		Overflow,
        /// TO DELETE ON PROD
        NotImplementedYet,
	}

    #[pallet::storage]
    #[pallet::getter(fn percent_for_nsp)]
    pub type PercentForNSM<T: Config> = StorageValue<_, u8>;

    #[pallet::genesis_config]
	pub struct GenesisConfig {
		pub percent_for_nsp: u8
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				percent_for_nsp: 10,
			}
		}
	}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
            PercentForNSM::<T>::put(self.percent_for_nsp);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn proposal_value_mint(
            origin: OriginFor<T>,
			proposal_hash: u32,
			target_account: T::AccountId,
			network_service_provider: T::AccountId,
            amount: u128,
		) -> DispatchResult {
            ensure_root(origin)?;

            let fee_amount: u128 = amount / Self::percent_for_nsp().unwrap() as u128;

            let neg_imbalance = T::Currency::issue(amount.checked_into().ok_or(Error::<T>::Overflow)?);
            let neg_fee_imbalance = T::Currency::issue(fee_amount.checked_into().ok_or(Error::<T>::Overflow)?);

            T::Currency::resolve_creating(&target_account, neg_imbalance);
            T::Currency::resolve_creating(&network_service_provider, neg_fee_imbalance);

            Self::deposit_event(
                Event::<T>::ValueMinted(proposal_hash, target_account, amount)
            );
            Ok(())
		}
	}
}
