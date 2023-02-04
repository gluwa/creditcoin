
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-23, STEPS: `50`, REPEAT: 30, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `nathans-mbp.lan`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// -p
// crate
// -e
// *
// --dev
// --execution
// wasm
// --output
// /Users/nathanw/Documents/Work/creditcoin/pallets/creditcoin/src/weights.rs
// --repeat
// 30
// --steps
// 50

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	// Storage: Creditcoin PendingTasks (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v7(t: u32, ) -> Weight {
		// Minimum execution time: 5_300 nanoseconds.
		Weight::from_ref_time(5_300_000 as u64)
			// Standard Error: 22_048
			.saturating_add(Weight::from_ref_time(7_932_888 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(t as u64)))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: TaskScheduler Authorities (r:0 w:20)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v8(t: u32, ) -> Weight {
		// Minimum execution time: 10_039 nanoseconds.
		Weight::from_ref_time(10_039_000 as u64)
			// Standard Error: 20_773
			.saturating_add(Weight::from_ref_time(9_004_065 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(t as u64)))
	}
	// Storage: Creditcoin DealOrders (r:511 w:510)
	// Storage: Creditcoin BidOrders (r:0 w:255)
	// Storage: Creditcoin Offers (r:0 w:255)
	// Storage: Creditcoin AskOrders (r:0 w:25)
	fn on_initialize(a: u32, b: u32, o: u32, d: u32, f: u32, ) -> Weight {
		Weight::from_ref_time(1_210_357_000 as u64)
			// Standard Error: 2_179_000
			.saturating_add(Weight::from_ref_time(1_253_000 as u64).saturating_mul(a as u64))
			// Standard Error: 2_179_000
			.saturating_add(Weight::from_ref_time(313_000 as u64).saturating_mul(o as u64))
			// Standard Error: 2_179_000
			.saturating_add(Weight::from_ref_time(10_053_000 as u64).saturating_mul(d as u64))
			// Standard Error: 2_179_000
			.saturating_add(Weight::from_ref_time(15_846_000 as u64).saturating_mul(f as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(d as u64)))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(f as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(o as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(d as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(f as u64)))
	}
	// Storage: Creditcoin Addresses (r:1 w:1)
	fn register_address() -> Weight {
		// Minimum execution time: 56_000 nanoseconds.
		Weight::from_ref_time(57_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin LegacyWallets (r:1 w:1)
	// Storage: Creditcoin LegacyBalanceKeeper (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim_legacy_wallet() -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(41_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin AskOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_ask_order() -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin BidOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_bid_order() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(24_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Offers (r:1 w:1)
	fn add_offer() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(28_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Offers (r:1 w:0)
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn add_deal_order() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:1)
	fn add_authority() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(6_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_transfer() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(22_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_transfer() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn fund_deal_order() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	fn lock_deal_order() -> Weight {
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_funding_transfer() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_repayment_transfer() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_funding_transfer_legacy() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn register_repayment_transfer_legacy() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn close_deal_order() -> Weight {
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_ref_time(32_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn exempt() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(23_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin AskOrders (r:1 w:1)
	// Storage: Creditcoin BidOrders (r:1 w:1)
	// Storage: Creditcoin Offers (r:1 w:1)
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	fn register_deal_order() -> Weight {
		// Minimum execution time: 86_000 nanoseconds.
		Weight::from_ref_time(92_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Creditcoin CollectCoinsContract (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	fn request_collect_coins() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(22_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_collect_coins() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(15_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_collect_coins() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(40_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin Authorities (r:1 w:1)
	fn remove_authority() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Currencies (r:1 w:1)
	fn register_currency() -> Weight {
		// Minimum execution time: 13_000 nanoseconds.
		Weight::from_ref_time(14_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin CollectCoinsContract (r:0 w:1)
	fn set_collect_coins_contract() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(5_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
