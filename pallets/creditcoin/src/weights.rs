
//! Autogenerated weights for `crate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-25, STEPS: `8`, REPEAT: `8`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `github-runner-4798384633-attempt-1`, CPU: `AMD EPYC 7452 32-Core Processor`
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
	/// Storage: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1 w:0)
	/// Proof Skipped: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1 w:0)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v6(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `3566`
		// Minimum execution time: 15_601_000 picoseconds.
		Weight::from_parts(17_929_303, 0)
			.saturating_add(Weight::from_parts(0, 3566))
			// Standard Error: 561
			.saturating_add(Weight::from_parts(3_302, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Proof Skipped: unknown `0xd766358cca00233e6155d7c14e2c085f5e0621c4869aa60c02be9adcc98a0d1d` (r:1025 w:1024)
	/// Storage: TaskScheduler Authorities (r:0 w:1024)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `t` is `[0, 1024]`.
	fn migration_v7(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153 + t * (53 ±0)`
		//  Estimated: `3600 + t * (2529 ±0)`
		// Minimum execution time: 15_800_000 picoseconds.
		Weight::from_parts(16_100_000, 0)
			.saturating_add(Weight::from_parts(0, 3600))
			// Standard Error: 100_768
			.saturating_add(Weight::from_parts(12_734_351, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2529).saturating_mul(t.into()))
	}
	/// Storage: Creditcoin AskOrders (r:255 w:255)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:255 w:255)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:255 w:255)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// Storage: Creditcoin DealOrders (r:510 w:509)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 255]`.
	/// The range of component `b` is `[0, 255]`.
	/// The range of component `o` is `[0, 255]`.
	/// The range of component `d` is `[0, 255]`.
	/// The range of component `f` is `[0, 255]`.
	fn on_initialize(a: u32, b: u32, o: u32, d: u32, f: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `138 + a * (69 ±0) + b * (69 ±0) + o * (69 ±0) + d * (261 ±0) + f * (293 ±0)`
		//  Estimated: `7059 + d * (3099 ±0) + f * (3099 ±0) + a * (2923 ±0) + b * (2923 ±0) + o * (2890 ±0)`
		// Minimum execution time: 2_890_183_000 picoseconds.
		Weight::from_parts(2_919_684_000, 0)
			.saturating_add(Weight::from_parts(0, 7059))
			// Standard Error: 228_849
			.saturating_add(Weight::from_parts(5_121_907, 0).saturating_mul(d.into()))
			// Standard Error: 228_849
			.saturating_add(Weight::from_parts(10_201_005, 0).saturating_mul(f.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(f.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(f.into())))
			.saturating_add(Weight::from_parts(0, 3099).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 3099).saturating_mul(f.into()))
			.saturating_add(Weight::from_parts(0, 2923).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 2923).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 2890).saturating_mul(o.into()))
	}
	/// Storage: Creditcoin Addresses (r:1 w:1)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	fn register_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `4062`
		// Minimum execution time: 96_503_000 picoseconds.
		Weight::from_parts(100_103_000, 0)
			.saturating_add(Weight::from_parts(0, 4062))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin LegacyWallets (r:1 w:1)
	/// Proof: Creditcoin LegacyWallets (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Creditcoin LegacyBalanceKeeper (r:1 w:0)
	/// Proof: Creditcoin LegacyBalanceKeeper (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_legacy_wallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `288`
		//  Estimated: `8607`
		// Minimum execution time: 88_402_000 picoseconds.
		Weight::from_parts(91_003_000, 0)
			.saturating_add(Weight::from_parts(0, 8607))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_ask_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `11714`
		// Minimum execution time: 46_902_000 picoseconds.
		Weight::from_parts(47_401_000, 0)
			.saturating_add(Weight::from_parts(0, 11714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin UsedGuids (r:1 w:1)
	/// Proof: Creditcoin UsedGuids (max_values: None, max_size: Some(274), added: 2749, mode: MaxEncodedLen)
	fn add_bid_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206`
		//  Estimated: `11714`
		// Minimum execution time: 46_301_000 picoseconds.
		Weight::from_parts(47_901_000, 0)
			.saturating_add(Weight::from_parts(0, 11714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	fn add_offer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `580`
		//  Estimated: `11706`
		// Minimum execution time: 45_701_000 picoseconds.
		Weight::from_parts(47_002_000, 0)
			.saturating_add(Weight::from_parts(0, 11706))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:0)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:0)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:0)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn add_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `817`
		//  Estimated: `17288`
		// Minimum execution time: 58_902_000 picoseconds.
		Weight::from_parts(60_102_000, 0)
			.saturating_add(Weight::from_parts(0, 17288))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn add_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3513`
		// Minimum execution time: 14_501_000 picoseconds.
		Weight::from_parts(14_801_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn persist_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7965`
		// Minimum execution time: 46_301_000 picoseconds.
		Weight::from_parts(46_602_000, 0)
			.saturating_add(Weight::from_parts(0, 7965))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn fail_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `290`
		//  Estimated: `7965`
		// Minimum execution time: 33_301_000 picoseconds.
		Weight::from_parts(34_001_000, 0)
			.saturating_add(Weight::from_parts(0, 7965))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	fn fund_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1002`
		//  Estimated: `14096`
		// Minimum execution time: 63_502_000 picoseconds.
		Weight::from_parts(64_402_000, 0)
			.saturating_add(Weight::from_parts(0, 14096))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	fn lock_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `4089`
		// Minimum execution time: 31_001_000 picoseconds.
		Weight::from_parts(31_400_000, 0)
			.saturating_add(Weight::from_parts(0, 4089))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn register_funding_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `20652`
		// Minimum execution time: 64_002_000 picoseconds.
		Weight::from_parts(64_902_000, 0)
			.saturating_add(Weight::from_parts(0, 20652))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:0)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:0)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn register_repayment_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `684`
		//  Estimated: `20652`
		// Minimum execution time: 63_602_000 picoseconds.
		Weight::from_parts(64_902_000, 0)
			.saturating_add(Weight::from_parts(0, 20652))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Creditcoin Transfers (r:1 w:1)
	/// Proof: Creditcoin Transfers (max_values: None, max_size: Some(987), added: 3462, mode: MaxEncodedLen)
	fn close_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1104`
		//  Estimated: `14096`
		// Minimum execution time: 64_902_000 picoseconds.
		Weight::from_parts(66_202_000, 0)
			.saturating_add(Weight::from_parts(0, 14096))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn exempt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `662`
		//  Estimated: `9644`
		// Minimum execution time: 46_002_000 picoseconds.
		Weight::from_parts(46_501_000, 0)
			.saturating_add(Weight::from_parts(0, 9644))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin Addresses (r:2 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: Creditcoin AskOrders (r:1 w:1)
	/// Proof: Creditcoin AskOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin BidOrders (r:1 w:1)
	/// Proof: Creditcoin BidOrders (max_values: None, max_size: Some(448), added: 2923, mode: MaxEncodedLen)
	/// Storage: Creditcoin Offers (r:1 w:1)
	/// Proof: Creditcoin Offers (max_values: None, max_size: Some(415), added: 2890, mode: MaxEncodedLen)
	/// Storage: Creditcoin DealOrders (r:1 w:1)
	/// Proof: Creditcoin DealOrders (max_values: None, max_size: Some(624), added: 3099, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn register_deal_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `24422`
		// Minimum execution time: 164_605_000 picoseconds.
		Weight::from_parts(167_905_000, 0)
			.saturating_add(Weight::from_parts(0, 24422))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Creditcoin CollectCoinsContract (r:1 w:0)
	/// Proof: Creditcoin CollectCoinsContract (max_values: Some(1), max_size: Some(279), added: 774, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:0)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:1 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	fn request_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `209`
		//  Estimated: `14606`
		// Minimum execution time: 45_901_000 picoseconds.
		Weight::from_parts(47_501_000, 0)
			.saturating_add(Weight::from_parts(0, 14606))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:0)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn fail_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `153`
		//  Estimated: `7316`
		// Minimum execution time: 29_500_000 picoseconds.
		Weight::from_parts(29_801_000, 0)
			.saturating_add(Weight::from_parts(0, 7316))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:0)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Creditcoin CollectedCoins (r:1 w:1)
	/// Proof: Creditcoin CollectedCoins (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	/// Storage: Creditcoin Addresses (r:1 w:0)
	/// Proof: Creditcoin Addresses (max_values: None, max_size: Some(597), added: 3072, mode: MaxEncodedLen)
	/// Storage: TaskScheduler PendingTasks (r:0 w:1)
	/// Proof: TaskScheduler PendingTasks (max_values: None, max_size: Some(1512), added: 3987, mode: MaxEncodedLen)
	fn persist_collect_coins() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `283`
		//  Estimated: `11378`
		// Minimum execution time: 87_403_000 picoseconds.
		Weight::from_parts(89_303_000, 0)
			.saturating_add(Weight::from_parts(0, 11378))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: TaskScheduler Authorities (r:1 w:1)
	/// Proof: TaskScheduler Authorities (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn remove_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `3513`
		// Minimum execution time: 19_101_000 picoseconds.
		Weight::from_parts(19_500_000, 0)
			.saturating_add(Weight::from_parts(0, 3513))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Creditcoin CollectCoinsContract (r:0 w:1)
	/// Proof: Creditcoin CollectCoinsContract (max_values: Some(1), max_size: Some(279), added: 774, mode: MaxEncodedLen)
	fn set_collect_coins_contract() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_600_000 picoseconds.
		Weight::from_parts(9_900_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
