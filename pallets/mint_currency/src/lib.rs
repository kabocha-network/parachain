#![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
        dispatch::DispatchResult, pallet_prelude::*,
        traits::Currency,
        sp_runtime::traits::CheckedConversion
    };
	use frame_system::{
        pallet_prelude::*,
        ensure_root,
    };

	/// Configure the pallet by specifying the parameters and types on which it depends.
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
	// pub type Something<T> = StorageValue<_, u32>;
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// the proposal value have been minted on the target account
		/// [proposal_id, target_account, value]
		ProposalValueMinted(u32, T::AccountId, u128),
		/// the proposal value have been added on the target account from the treasury
		/// [proposal_id, target_account, value]
		ProposalValueAddedFromTreasury(u32, T::AccountId, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proposal have not been yet accepted
		ProposalNotAccepted,
        /// There is not enough funds in the treasury to pay the proposal
        NotEnoughFunds,
		Overflow,
        NotImplementedYet, // TO DELETE ON PRODUCTION
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
            // should the transaction be signed ?
            // if yes, by root ?
            ensure_root(origin)?;

            // ensure the proposal_hash have been accepted
            // else, return Error::ProposalNotAccepted

            let fee_amount: u128 = amount / 10;

            let neg_imbalance = T::Currency::issue(amount.checked_into().ok_or(Error::<T>::Overflow)?);
            let neg_fee_imbalance = T::Currency::issue(fee_amount.checked_into().ok_or(Error::<T>::Overflow)?);

            if let Err(_) = T::Currency::resolve_into_existing(&target_account, neg_imbalance) {
                //something to do with neg_imbalance to resolve the imbalance: either remove funds,
                //or add them to the treasury
                return Err(Error::<T>::Overflow)?;
            };
            if let Err(_) = T::Currency::resolve_into_existing(&network_service_provider, neg_fee_imbalance) {
                //something to do with neg_fee_imbalance to resolve the imbalance: either remove funds,
                //or add them to the treasury
                return Err(Error::<T>::Overflow)?;
            };

            Self::deposit_event(
                Event::<T>::ProposalValueMinted(proposal_hash, target_account, amount)
            );
            Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn proposal_value_from_treasury(
            _origin: OriginFor<T>,
			_proposal_hash: u32,
			_target_account: T::AccountId,
			_network_service_provider: T::AccountId,
            _amount: u128,
		) -> DispatchResult {
            Err(Error::<T>::NotImplementedYet)?
            // should the transaction be signed ?
            // if yes, by root ?
            // ensure_root(origin)?;

            // ensure the proposal_hash have been accepted
            // else, return Error::ProposalNotAccepted

            // let fee_amount: u128 = amount / 10;
            // let total_amount = amount + fee_amount;

            // check if there is enough funds in treasury

            // transfer
            // Self::deposit_event(
            //     Event::<T>::ProposalValueAddedFromTreasury(proposal_hash, target_account, amount)
            // );
            // Ok(())
		}
	}
}
