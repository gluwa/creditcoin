use criterion::{black_box, criterion_group, criterion_main, Criterion};
use frame_system::Config;
use frame_system::Pallet as System;
use pallet_creditcoin::ocw::task::guard::LocalTaskStatus;
use runtime::Runtime;
use sp_core::offchain::{testing, OffchainDbExt, OffchainWorkerExt};
use sp_io::TestExternalities;
use sp_runtime::offchain::storage_lock::{BlockAndTime, Lockable, StorageLock};

type Y = System<Runtime>;
type X = BlockAndTime<Y>;

const KEY: &[u8; 7] = b"storage";

fn lock(key: &[u8]) {
	let mut lock = StorageLock::<X>::new(key);
	let _ = lock.try_lock();
}

fn lock_skip<L>(lock: &mut StorageLock<L>) where L: Lockable{
	let _ = lock.try_lock();
}

fn custom_lock_skip(status: &mut LocalTaskStatus) {
	let _a = status.try_fast();
	status.storage_ref.clear();
}

fn custom_lock_free(key: &[u8]) {
	let status = LocalTaskStatus::new(key);
	let _a = status.try_fast();
	let _a = status.try_slow();
}

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("lock unlock", |b| {
		let (offchain, _state) = testing::TestOffchainExt::new();
		let mut t = TestExternalities::default();
		t.register_extension(OffchainDbExt::new(offchain.clone()));
		t.register_extension(OffchainWorkerExt::new(offchain));
		t.execute_with(|| b.iter(|| lock(black_box(KEY))));
	});

	c.bench_function("custom_lock_free", |b| {
		let (offchain, _state) = testing::TestOffchainExt::new();
		let mut t = TestExternalities::default();
		t.register_extension(OffchainDbExt::new(offchain.clone()));
		t.register_extension(OffchainWorkerExt::new(offchain));
		t.execute_with(|| b.iter(|| custom_lock_free(black_box(KEY))));
	});

	c.bench_function("lock skip", |b| {
		let (offchain, _state) = testing::TestOffchainExt::new();
		let mut t = TestExternalities::default();
		t.register_extension(OffchainDbExt::new(offchain.clone()));
		t.register_extension(OffchainWorkerExt::new(offchain));
		let mut lock_ = StorageLock::<X>::new(KEY);
		t.execute_with(|| b.iter(|| lock_skip(black_box(&mut lock_))));
	});

	c.bench_function("custom lock skip", |b| {
		let (offchain, _state) = testing::TestOffchainExt::new();
		let mut t = TestExternalities::default();
		t.register_extension(OffchainDbExt::new(offchain.clone()));
		t.register_extension(OffchainWorkerExt::new(offchain));

		t.execute_with(|| {
			let mut status = LocalTaskStatus::new(KEY);
			b.iter(|| custom_lock_skip(black_box(&mut status)))
		});
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
