use crate::{Config, Pallet};
use alloc::vec::Vec;
use sp_runtime::codec::Encode;
use sp_runtime::offchain::storage_lock::{StorageLock, Time};
use sp_runtime::offchain::Duration;

const SYNCED_NONCE: &[u8] = b"creditcoin/OCW/nonce/nonce/";
const SYNCED_NONCE_LOCK: &[u8] = b"creditcoin/OCW/nonce/lock/";
const LOCK_DEADLINE: u64 = 50_000;

pub(super) fn lock_key<Id: Encode>(id: &Id) -> Vec<u8> {
	id.using_encoded(|encoded_id| SYNCED_NONCE_LOCK.iter().chain(encoded_id).copied().collect())
}

pub fn nonce_key<Id: Encode>(id: &Id) -> Vec<u8> {
	id.using_encoded(|encoded_id| SYNCED_NONCE.iter().chain(encoded_id).copied().collect())
}

impl<T: Config> Pallet<T> {
	pub(super) fn nonce_lock_new(key: &[u8]) -> StorageLock<'_, Time> {
		StorageLock::<Time>::with_deadline(key, Duration::from_millis(LOCK_DEADLINE))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::runtime::Runtime;
	use crate::mock::runtime::RuntimeCall;
	use crate::mock::{roll_to, task::MockTask, ExtBuilder, Trivial, WithWorkerHook};
	use crate::tasks::TaskScheduler;
	use crate::tasks::TaskV2;
	use core::sync::atomic::AtomicU64;
	use frame_support::assert_ok;
	use frame_system::Config as SystemConfig;
	use frame_system::Pallet as System;
	use sp_runtime::offchain::storage::StorageValueRef;
	use sp_runtime::offchain::testing::TestOffchainExt;
	use sp_runtime::traits::IdentifyAccount;
	use std::sync::atomic::Ordering;
	use std::sync::Arc;

	#[test]
	fn increment_after_call() {
		let mut builder = ExtBuilder::default().with_keystore();
		let pkey = builder.generate_authority();
		builder.with_offchain();
		builder.with_pool();
		let mut ext = builder.build();
		let acct = <Runtime as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.execute_with(|| {
			roll_to::<Trivial>(1);

			let task_deadline = Runtime::deadline();
			let task = MockTask::Remark(0);
			let id = TaskV2::<Runtime>::to_id(&task);
			Runtime::insert(&task_deadline, &id, task);

			roll_to::<WithWorkerHook>(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce: u64 = synced_nonce.get().unwrap().unwrap();
			assert_eq!(synced_nonce, 1u64);
			let nonce = System::<Runtime>::account(acct).nonce;
			assert_eq!(nonce, 1u64);
		});
	}

	#[test]
	fn unique_per_account() {
		let mut builder = ExtBuilder::default().with_keystore();
		let pkey = builder.generate_authority();
		let acct_1 = <Runtime as SystemConfig>::AccountId::from(pkey.into_account().0);
		let pkey = builder.generate_authority();
		let acct_2 = <Runtime as SystemConfig>::AccountId::from(pkey.into_account().0);
		assert!(nonce_key(&acct_1) != nonce_key(&acct_2));
		assert!(lock_key(&acct_1) != lock_key(&acct_2));
	}

	#[test]
	fn not_incremented_on_evaluation_error() {
		let mut builder = ExtBuilder::default().with_keystore();
		let pkey = builder.generate_authority();
		builder.with_offchain();
		let mut ext = builder.build();
		let acct = <Runtime as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.execute_with(|| {
			roll_to::<Trivial>(1);

			let task_deadline = Runtime::deadline();
			let task = MockTask::Evaluation;
			let id = TaskV2::<Runtime>::to_id(&task);
			Runtime::insert(&task_deadline, &id, task);

			roll_to::<WithWorkerHook>(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce = synced_nonce.get::<u64>().unwrap();
			assert!(synced_nonce.is_none());
			let nonce = System::<Runtime>::account(acct).nonce;
			assert_eq!(nonce, 0u64);
		});
	}

	#[test]
	fn not_incremented_on_scheduler_error() {
		let mut builder = ExtBuilder::default().with_keystore();
		let pkey = builder.generate_authority();
		builder.with_offchain();
		let mut ext = builder.build();
		let acct = <Runtime as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.execute_with(|| {
			roll_to::<Trivial>(1);

			let task_deadline = Runtime::deadline();
			let task = MockTask::Scheduler;
			let id = TaskV2::<Runtime>::to_id(&task);
			Runtime::insert(&task_deadline, &id, task);

			roll_to::<WithWorkerHook>(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce = synced_nonce.get::<u64>().unwrap();
			assert!(synced_nonce.is_none());
			let nonce = System::<Runtime>::account(acct).nonce;
			assert_eq!(nonce, 0u64);
		});
	}

	#[test]
	fn parallel_increment() {
		let (offchain, _) = TestOffchainExt::new();
		const THREADS: u32 = 3;
		const LOOP: u32 = 10;
		let nonces = Arc::new(AtomicU64::new(0));

		let handles = (0..THREADS).into_iter().map(|_| {
			let offchain = offchain.clone();
			let nonces = nonces.clone();

			std::thread::spawn(move || {
				let mut ext_builder = ExtBuilder::default().with_keystore();
				let acct = {
					let acct_pubkey = ext_builder.generate_authority();
					<Runtime as SystemConfig>::AccountId::from(acct_pubkey.into_account().0)
				};
				ext_builder.offchain = Some(offchain);
				ext_builder.with_pool();
				let mut ext = ext_builder.build();

				let execute = || {
					roll_to::<Trivial>(1);
					let call: RuntimeCall =
						MockTask::Remark(0).forward_task(Runtime::deadline()).expect("call").into();

					for _ in 0..LOOP {
						assert_ok!(crate::Pallet::<Runtime>::submit_txn_with_synced_nonce(
							acct.clone(),
							|_| call.clone(),
						));

						let nonce = System::<Runtime>::account(acct.clone()).nonce;
						nonces.fetch_add(nonce, Ordering::Relaxed);
					}
				};

				ext.execute_with(execute);
			})
		});

		for h in handles {
			h.join().expect("testing context is shared");
		}

		let mut ext_builder = ExtBuilder::default();
		ext_builder.offchain = Some(offchain);
		let mut ext = ext_builder.build();
		ext.execute_with(|| {
			let nonce_post_submition_sum = (THREADS * LOOP) * (THREADS * LOOP + 1) / 2;
			assert_eq!(nonces.load(Ordering::Relaxed), nonce_post_submition_sum as u64);
		});
	}

	#[test]
	fn lock_works() {
		let (offchain, _) = TestOffchainExt::new();
		const THREADS: u32 = 2;

		let handles = (0..THREADS).map(|_| {
			let offchain = offchain.clone();

			std::thread::spawn(move || {
				let mut ext_builder = ExtBuilder::default().with_keystore();
				let acct_pubkey = ext_builder.generate_authority();
				let acct = <Runtime as SystemConfig>::AccountId::from(acct_pubkey.into_account().0);
				ext_builder.offchain = Some(offchain);
				let mut ext = ext_builder.build();

				let execute = || {
					roll_to::<Trivial>(1);

					let key = lock_key(&acct);
					let mut lock = crate::Pallet::<Runtime>::nonce_lock_new(&key);
					let guard = lock.try_lock();
					guard.map(|g| g.forget()).or_else(|deadline| {
						// failed to acq guard; move to active guard's deadline boundaries
						sp_io::offchain::sleep_until(deadline);
						//deadline still effective
						lock.try_lock().map(|_| ())
					})
				};

				ext.execute_with(execute)
			})
		});

		if !handles.into_iter().any(|h| h.join().expect("thread joins").is_err()) {
			panic!("lock should block")
		}
	}

	#[test]
	fn nonce_lock_expires() {
		let mut ext_builder = ExtBuilder::default().with_keystore();
		ext_builder.with_offchain();
		ext_builder.build().execute_with(|| {
			roll_to::<Trivial>(1);

			let key = &b"lock_key"[..];
			let mut lock = Pallet::<Runtime>::nonce_lock_new(key);
			let guard = lock.try_lock().expect("ok");
			guard.forget();
			let guard = lock.try_lock();
			let deadline = guard.map(|_| ()).expect_err("deadline");
			// failed to acq guard; move past active guard's deadline boundary
			sp_io::offchain::sleep_until(deadline.add(Duration::from_millis(LOCK_DEADLINE + 1)));
			let g = lock.try_lock();
			assert!(g.is_ok());
		});
	}
}
