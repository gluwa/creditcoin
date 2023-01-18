
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-18, STEPS: `8`, REPEAT: 8, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `github-runner-3950872627`, CPU: `AMD EPYC 7452 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/creditcoin-node
// benchmark
// pallet
// --chain
// dev
// --steps=8
// --repeat=8
// --pallet
// crate
// --extrinsic=*
// --execution
// wasm
// --wasm-execution=compiled
// --heap-pages=10000
// --output
// ./pallets/creditcoin/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `crate`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0xd766358cca00233e6155d7c14e2c085f4e7b9012096b41c4eb3aaf947f6ea429] (r:1 w:1)
	// Storage: Creditcoin PendingTasks (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v7(t: u32, ) -> Weight {
		// Minimum execution time: 14_101 nanoseconds.
		Weight::from_ref_time(14_601_000 as u64)
			// Standard Error: 23_949
			.saturating_add(Weight::from_ref_time(15_296_931 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(t as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(t as u64)))
	}
	// Storage: Creditcoin DealOrders (r:511 w:510)
	// Storage: Creditcoin BidOrders (r:0 w:255)
	// Storage: Creditcoin Offers (r:0 w:255)
	// Storage: Creditcoin AskOrders (r:0 w:36)
	/// The range of component `a` is `[0, 255]`.
	/// The range of component `b` is `[0, 255]`.
	/// The range of component `o` is `[0, 255]`.
	/// The range of component `d` is `[0, 255]`.
	/// The range of component `f` is `[0, 255]`.
	fn on_initialize(a: u32, b: u32, o: u32, d: u32, f: u32, ) -> Weight {
		// Minimum execution time: 3_776_024 nanoseconds.
		Weight::from_ref_time(3_802_025_000 as u64)
			// Standard Error: 357_182
			.saturating_add(Weight::from_ref_time(8_310_643 as u64).saturating_mul(d as u64))
			// Standard Error: 357_182
			.saturating_add(Weight::from_ref_time(12_323_854 as u64).saturating_mul(f as u64))
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
		// Minimum execution time: 101_306 nanoseconds.
		Weight::from_ref_time(104_706_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin LegacyWallets (r:1 w:1)
	// Storage: Creditcoin LegacyBalanceKeeper (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn claim_legacy_wallet() -> Weight {
		// Minimum execution time: 87_905 nanoseconds.
		Weight::from_ref_time(90_306_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin AskOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_ask_order() -> Weight {
		// Minimum execution time: 56_204 nanoseconds.
		Weight::from_ref_time(57_104_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin BidOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin UsedGuids (r:1 w:1)
	fn add_bid_order() -> Weight {
		// Minimum execution time: 55_804 nanoseconds.
		Weight::from_ref_time(57_204_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Offers (r:1 w:1)
	fn add_offer() -> Weight {
		// Minimum execution time: 63_104 nanoseconds.
		Weight::from_ref_time(63_604_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Offers (r:1 w:0)
	// Storage: Creditcoin AskOrders (r:1 w:0)
	// Storage: Creditcoin BidOrders (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn add_deal_order() -> Weight {
		// Minimum execution time: 63_604 nanoseconds.
		Weight::from_ref_time(64_004_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:1)
	fn add_authority() -> Weight {
		// Minimum execution time: 14_501 nanoseconds.
		Weight::from_ref_time(14_901_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_transfer() -> Weight {
		// Minimum execution time: 51_103 nanoseconds.
		Weight::from_ref_time(52_804_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_transfer() -> Weight {
		// Minimum execution time: 41_002 nanoseconds.
		Weight::from_ref_time(41_603_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn fund_deal_order() -> Weight {
		// Minimum execution time: 66_804 nanoseconds.
		Weight::from_ref_time(68_204_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	fn lock_deal_order() -> Weight {
		// Minimum execution time: 39_002 nanoseconds.
		Weight::from_ref_time(39_202_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:1 w:1)
	fn register_funding_transfer() -> Weight {
		// Minimum execution time: 70_804 nanoseconds.
		Weight::from_ref_time(71_805_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:0)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Currencies (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:1 w:1)
	fn register_repayment_transfer() -> Weight {
		// Minimum execution time: 71_004 nanoseconds.
		Weight::from_ref_time(72_004_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:1 w:1)
	fn register_funding_transfer_legacy() -> Weight {
		// Minimum execution time: 73_004 nanoseconds.
		Weight::from_ref_time(74_904_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:2 w:0)
	// Storage: Creditcoin Transfers (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:1 w:1)
	fn register_repayment_transfer_legacy() -> Weight {
		// Minimum execution time: 72_704 nanoseconds.
		Weight::from_ref_time(74_405_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Creditcoin Transfers (r:1 w:1)
	fn close_deal_order() -> Weight {
		// Minimum execution time: 68_704 nanoseconds.
		Weight::from_ref_time(69_304_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Creditcoin DealOrders (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	fn exempt() -> Weight {
		// Minimum execution time: 51_303 nanoseconds.
		Weight::from_ref_time(52_503_000 as u64)
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
		// Minimum execution time: 163_910 nanoseconds.
		Weight::from_ref_time(165_710_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Creditcoin CollectCoinsContract (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: TaskScheduler PendingTasks (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	fn request_collect_coins() -> Weight {
		// Minimum execution time: 51_103 nanoseconds.
		Weight::from_ref_time(53_503_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn fail_collect_coins() -> Weight {
		// Minimum execution time: 34_802 nanoseconds.
		Weight::from_ref_time(35_602_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:0)
	// Storage: Creditcoin CollectedCoins (r:1 w:1)
	// Storage: Creditcoin Addresses (r:1 w:0)
	// Storage: Creditcoin PendingTasks (r:0 w:1)
	fn persist_collect_coins() -> Weight {
		// Minimum execution time: 92_905 nanoseconds.
		Weight::from_ref_time(94_205_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TaskScheduler Authorities (r:1 w:1)
	fn remove_authority() -> Weight {
		// Minimum execution time: 18_801 nanoseconds.
		Weight::from_ref_time(19_201_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin Currencies (r:1 w:1)
	fn register_currency() -> Weight {
		// Minimum execution time: 30_102 nanoseconds.
		Weight::from_ref_time(30_401_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Creditcoin CollectCoinsContract (r:0 w:1)
	fn set_collect_coins_contract() -> Weight {
		// Minimum execution time: 11_400 nanoseconds.
		Weight::from_ref_time(11_701_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
