/// Money matters.
pub mod currency {
	use node_primitives::Balance;

    /// The existential deposit.
    pub const EXISTENTIAL_DEPOSIT: Balance = 1 * MILLIUNIT;

	pub const UNIT: Balance = 1_000_000_000_000;
	pub const MILLIUNIT: Balance = UNIT / 1_000;
	pub const MICROUNIT: Balance = MILLIUNIT / 1_000;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 50 * MILLIUNIT + (bytes as Balance) * 5 * MICROUNIT
	}
}

/// Time.
pub mod time {
	use node_primitives::{BlockNumber};
    /// This determines the average expected block time that we are targeting.
    /// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
    /// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
    /// up by `pallet_aura` to implement `fn slot_duration()`.
    ///
    /// Change this to adjust the block time.
    pub const MILLISECS_PER_BLOCK: u64 = 12000;

    // NOTE: Currently it is not possible to change the slot duration after the chain has started.
    //       Attempting to do so will brick block production.
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

    // Time is measured by number of blocks.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
}

/// Fee-related.
pub mod fee {
	use frame_support::weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	};
	use node_primitives::Balance;
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updated based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

    /// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
    /// node's balance type.
    ///
    /// This should typically create a mapping between the following ranges:
    ///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
    ///   - `[Balance::min, Balance::max]`
    ///
    /// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
    ///   - Setting it to `0` will essentially disable the weight fee.
    ///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
    pub struct WeightToFee;
    impl WeightToFeePolynomial for WeightToFee {
        type Balance = Balance;
        fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
            // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
            // in our template, we map to 1/10 of that, or 1/10 MILLIUNIT
            let p = super::currency::MILLIUNIT / 10;
            let q = 100 * Balance::from(ExtrinsicBaseWeight::get());
            smallvec![WeightToFeeCoefficient {
                degree: 1,
                negative: false,
                coeff_frac: Perbill::from_rational(p % q, q),
                coeff_integer: p / q,
            }]
        }
    }
}