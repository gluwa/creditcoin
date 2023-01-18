
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-18, STEPS: `50`, REPEAT: 30, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `github-runner-3950872627`, CPU: `AMD EPYC 7452 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// --chain
// dev
// --steps=50
// --repeat=30
// --pallet
// crate
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

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	// Storage: Difficulty TargetBlockTime (r:0 w:1)
	fn set_target_block_time() -> Weight {
		// Minimum execution time: 8_200 nanoseconds.
		Weight::from_ref_time(8_500_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Difficulty DifficultyAdjustmentPeriod (r:0 w:1)
	fn set_adjustment_period() -> Weight {
		// Minimum execution time: 8_100 nanoseconds.
		Weight::from_ref_time(8_300_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
