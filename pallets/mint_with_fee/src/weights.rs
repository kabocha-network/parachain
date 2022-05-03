#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{
        Weight,
        constants::RocksDbWeight as DbWeight,
    }
};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn mint() -> Weight;
    fn change_fee_percent() -> Weight;
}

/// Weight functions for `pallet_mint_with_fee`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: MintWithfee FeePercent (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn mint() -> Weight {
		(106_721_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: MintWithfee FeePercent (r:0 w:1)
	fn change_fee_percent() -> Weight {
		(31_050_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

impl WeightInfo for () {
	// Storage: MintWithfee FeePercent (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn mint() -> Weight {
		(106_721_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	// Storage: MintWithfee FeePercent (r:0 w:1)
	fn change_fee_percent() -> Weight {
		(31_050_000 as Weight)
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
