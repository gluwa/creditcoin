use frame_support::pallet_prelude::StorageDoubleMap;
use frame_support::storage::generator::StorageDoubleMap as DoubleMapGen;
use frame_support::traits::Hooks;
use frame_support::weights::Weight;
use frame_support::IterableStorageDoubleMap;
use parity_scale_codec::{EncodeLike, FullCodec};
use sp_core::ConstU32;
use sp_io::{MultiRemovalResults, TestExternalities};
use sp_runtime::WeakBoundedVec;

use crate::mock::{self, Creditcoin, ExtBuilder};
use crate::mock::{BlockNumber, Test as TestRuntime};
use crate::test_utils::{insert_fake_ask, insert_fake_bid, insert_fake_offer};
use crate::types::{StorageCleanupState, StorageItemCleanupState};
use sp_runtime::traits::Get;

#[derive(Clone, Copy)]
enum StorageKind {
	Ask,
	Bid,
	Offer,
}

struct StorageRequest {
	kind: StorageKind,
	count: usize,
	expiration_block: u64,
}

#[derive(Default)]
struct StorageBuilder {
	requests: Vec<StorageRequest>,
	cleanup_state: Option<StorageCleanupState<BlockNumber>>,
}

impl StorageBuilder {
	fn new() -> Self {
		Self::default()
	}

	fn ask_orders(&mut self, count: usize, expiration_block: u64) -> &mut Self {
		self.requests
			.push(StorageRequest { kind: StorageKind::Ask, count, expiration_block });
		self
	}

	fn bid_orders(&mut self, count: usize, expiration_block: u64) -> &mut Self {
		self.requests
			.push(StorageRequest { kind: StorageKind::Bid, count, expiration_block });
		self
	}

	fn offers(&mut self, count: usize, expiration_block: u64) -> &mut Self {
		self.requests
			.push(StorageRequest { kind: StorageKind::Offer, count, expiration_block });
		self
	}

	#[allow(dead_code)]
	fn cleanup_state(&mut self, state: StorageCleanupState<BlockNumber>) -> &mut Self {
		self.cleanup_state = Some(state);
		self
	}

	fn finish(&mut self) -> TestExternalities {
		let mut t = ExtBuilder::default().build();
		t.execute_with(|| {
			let alice = mock::AccountId::from([1u8; 32]);

			for request in &self.requests {
				match request.kind {
					StorageKind::Ask => {
						for i in 0..request.count {
							insert_fake_ask::<TestRuntime>(
								&alice,
								request.expiration_block,
								i as u32,
							);
						}
					},
					StorageKind::Bid => {
						for i in 0..request.count {
							insert_fake_bid::<TestRuntime>(
								&alice,
								request.expiration_block,
								i as u32,
							);
						}
					},
					StorageKind::Offer => {
						for i in 0..request.count {
							insert_fake_offer::<TestRuntime>(
								&alice,
								request.expiration_block,
								i as u32,
							);
						}
					},
				}
			}

			if let Some(state) = self.cleanup_state.take() {
				crate::CleanupState::<TestRuntime>::put(state);
			}
		});
		t.commit_all().unwrap();
		t
	}
}

#[extend::ext]
impl TestExternalities {
	fn run<F: FnOnce() -> R, R>(&mut self, f: F) -> R {
		let res = self.execute_with(f);
		self.commit_all().unwrap();
		res
	}
	fn then_run(&mut self, f: impl FnOnce()) -> &mut TestExternalities {
		self.run(f);
		self
	}
}

#[extend::ext]
impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value>
	StorageDoubleMap<Prefix, Hasher1, Key1, Hasher2, Key2, Value>
where
	Key1: FullCodec,
	Key2: FullCodec,
	Value: FullCodec,
	Self: IterableStorageDoubleMap<Key1, Key2, Value>,
{
	fn count_at(prefix: impl EncodeLike<Key1>) -> usize {
		Self::iter_prefix(prefix).count()
	}
	fn count() -> usize {
		Self::iter().count()
	}
}

type AskOrders = crate::AskOrders<TestRuntime>;
type BidOrders = crate::BidOrders<TestRuntime>;
type Offers = crate::Offers<TestRuntime>;

// has to be called with externalities in scope
fn cleanup(block: u64) -> Weight {
	Creditcoin::on_initialize(block)
}

fn cleanup_state() -> Option<StorageCleanupState<BlockNumber>> {
	crate::CleanupState::<TestRuntime>::get()
}

const LIMIT: usize = mock::CLEANUP_LIMIT as usize;

#[test]
fn cleans_up_asks() {
	let count = LIMIT / 2;
	let exp = 5;

	StorageBuilder::new().ask_orders(count, exp).finish().execute_with(|| {
		assert_eq!(crate::AskOrders::<TestRuntime>::iter().count(), count);

		cleanup(exp);

		assert_eq!(crate::AskOrders::<TestRuntime>::iter().count(), 0);
		assert_eq!(cleanup_state(), Some(StorageCleanupState::new(exp + 1)));
	});
}

fn weak_bounded<S: Get<u32>>(v: Vec<u8>) -> WeakBoundedVec<u8, S> {
	WeakBoundedVec::force_from(v, None)
}

macro_rules! impl_get_partial_key {
	($($storage: ty),*) => {
		$(
			impl GetPartialKey for $storage {
				fn partial_key(first: u64) -> Vec<u8> {
					<$storage>::storage_double_map_final_key1(first)
				}
			}
		)*
	};
}

trait GetPartialKey {
	fn partial_key(exp: u64) -> Vec<u8>;
}

impl_get_partial_key!(AskOrders, BidOrders, Offers);

fn cursor_for<Storage: GetPartialKey>(
	first: u64,
) -> Option<WeakBoundedVec<u8, ConstU32<{ StorageItemCleanupState::<BlockNumber>::MAX_CURSOR_LEN }>>>
{
	Some(weak_bounded(Storage::partial_key(first)))
}

#[test]
fn cleans_up_asks_to_limit() {
	let count = LIMIT * 2;
	let exp = 5;

	StorageBuilder::new()
		.ask_orders(count, exp)
		.finish()
		.then_run(|| {
			assert_eq!(AskOrders::iter().count(), count);

			cleanup(exp);

			assert_eq!(AskOrders::iter().count(), count - LIMIT);
			assert_eq!(
				cleanup_state(),
				Some(StorageCleanupState {
					ask_orders: StorageItemCleanupState {
						on_block: exp,
						cursor: cursor_for::<AskOrders>(exp)
					},
					..StorageCleanupState::new(exp)
				})
			);
		})
		.then_run(|| {
			assert_eq!(AskOrders::iter().count(), count - LIMIT);

			cleanup(exp + 1);

			assert_eq!(AskOrders::iter().count(), 0);
			assert_eq!(
				cleanup_state(),
				Some(StorageCleanupState {
					ask_orders: StorageItemCleanupState::new(exp + 1),
					..StorageCleanupState::new(exp)
				})
			);
		})
		.then_run(|| {
			assert_eq!(AskOrders::iter().count(), 0);

			cleanup(exp + 2);

			assert_eq!(AskOrders::iter().count(), 0);
			assert_eq!(cleanup_state(), Some(StorageCleanupState::new(exp + 3)));
		});
}

#[test]
fn cleans_up_bids() {
	let count = LIMIT / 2;
	let exp = 5;

	StorageBuilder::new().bid_orders(count, exp).finish().then_run(|| {
		assert_eq!(BidOrders::iter().count(), count);

		cleanup(exp);

		assert_eq!(BidOrders::iter().count(), 0);
		assert_eq!(cleanup_state(), Some(StorageCleanupState::new(exp + 1)));
	});
}

#[test]
fn cleans_up_offers() {
	let count = LIMIT / 2;
	let exp = 5;

	StorageBuilder::new().offers(count, exp).finish().then_run(|| {
		assert_eq!(Offers::iter().count(), count);

		cleanup(exp);

		assert_eq!(Offers::iter().count(), 0);
		assert_eq!(cleanup_state(), Some(StorageCleanupState::new(exp + 1)));
	});
}

#[test]
fn cleans_up_to_limit() {
	let count = LIMIT / 2;
	let exp = 5;

	StorageBuilder::new()
		.ask_orders(count, exp)
		.bid_orders(count, exp)
		.offers(count, exp)
		.finish()
		.then_run(|| {
			assert_eq!(AskOrders::iter().count(), count);
			assert_eq!(BidOrders::iter().count(), count);
			assert_eq!(Offers::iter().count(), count);

			cleanup(exp);

			assert_eq!(AskOrders::iter().count(), 0);
			assert_eq!(BidOrders::iter().count(), 0);
			assert_eq!(Offers::iter().count(), count);

			assert_eq!(
				cleanup_state(),
				Some(StorageCleanupState {
					ask_orders: StorageItemCleanupState::new(exp + 1),
					bid_orders: StorageItemCleanupState::new(exp + 1),
					offers: StorageItemCleanupState { on_block: exp, cursor: None },
				})
			);
		});
}

#[test]
fn catches_up_on_blocks() {
	let count = LIMIT * 2;
	let exp = 5;

	StorageBuilder::new()
		.ask_orders(count, exp)
		.ask_orders(1, exp + 1)
		.bid_orders(1, exp + 1)
		.offers(1, exp + 1)
		.ask_orders(1, exp + 2)
		.bid_orders(1, exp + 2)
		.offers(1, exp + 2)
		.finish()
		.then_run(|| {
			assert_eq!(AskOrders::count_at(exp), count);
			assert_eq!(AskOrders::count_at(exp + 1), 1);
			assert_eq!(AskOrders::count_at(exp + 2), 1);
			assert_eq!(BidOrders::count_at(exp + 1), 1);
			assert_eq!(BidOrders::count_at(exp + 2), 1);
			assert_eq!(Offers::count_at(exp + 1), 1);
			assert_eq!(Offers::count_at(exp + 2), 1);

			cleanup(exp);

			assert_eq!(AskOrders::count_at(exp), count / 2);
			assert_eq!(BidOrders::count(), 2);
			assert_eq!(Offers::count(), 2);

			assert_eq!(
				cleanup_state(),
				Some(StorageCleanupState {
					ask_orders: StorageItemCleanupState {
						on_block: exp,
						cursor: cursor_for::<AskOrders>(exp)
					},
					..StorageCleanupState::new(exp)
				})
			)
		})
		.then_run(|| {
			cleanup(exp + 1);

			assert_eq!(AskOrders::count_at(exp), 0);
			assert_eq!(AskOrders::count_at(exp + 1), 1);
			assert_eq!(AskOrders::count_at(exp + 2), 1);
			assert_eq!(BidOrders::count(), 2);
			assert_eq!(Offers::count(), 2);

			assert_eq!(
				cleanup_state(),
				Some(StorageCleanupState {
					ask_orders: StorageItemCleanupState::new(exp + 1),
					..StorageCleanupState::new(exp)
				})
			)
		})
		.then_run(|| {
			cleanup(exp + 2);

			assert_eq!(AskOrders::count(), 0);
			assert_eq!(BidOrders::count(), 0);
			assert_eq!(Offers::count(), 0);

			assert_eq!(cleanup_state(), Some(StorageCleanupState::new(exp + 3)))
		});
}

#[test]
fn cleanup_state_transitions() {
	let block = 5;
	let initial_state = StorageItemCleanupState::new(block);

	// No results => no change
	assert_eq!(initial_state.clone().updated_with(None), initial_state);

	// did some work, but no work left to do (cursor is None) => go on to next block
	let results = MultiRemovalResults { backend: 1, loops: 1, unique: 1, maybe_cursor: None };
	assert_eq!(
		initial_state.clone().updated_with(Some(results)),
		StorageItemCleanupState::new(block + 1)
	);

	// did some work, but there's more to do (cursor is Some) => stay on same block, but update cursor
	let resume_cursor = vec![1, 2, 3];
	let results = MultiRemovalResults {
		backend: 1,
		loops: 1,
		unique: 1,
		maybe_cursor: Some(resume_cursor.clone()),
	};
	assert_eq!(
		initial_state.updated_with(Some(results)),
		StorageItemCleanupState { on_block: block, cursor: Some(weak_bounded(resume_cursor)) }
	);
}
