
//! Autogenerated weights for `super`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-05, STEPS: `50`, REPEAT: 30, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// --chain
// dev
// --steps=50
// --repeat=30
// --pallet
// super
// --extrinsic=*
// --execution
// wasm
// --wasm-execution=compiled
// --heap-pages=10000
// --output
// ./pallets/difficulty/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `super`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> super::WeightInfo for WeightInfo<T> {
        // Storage: Difficulty TargetBlockTime (r:0 w:1)
        fn set_target_block_time() -> Weight {
                (4_300_000 as Weight)
                        .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
        // Storage: Difficulty DifficultyAdjustmentPeriod (r:0 w:1)
        fn set_adjustment_period() -> Weight {
                (3_900_000 as Weight)
                        .saturating_add(T::DbWeight::get().writes(1 as Weight))
        }
}
