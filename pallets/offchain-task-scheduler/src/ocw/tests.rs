#![cfg(test)]

use crate::mock::roll_to;
use crate::mock::runtime::AccountId;
use crate::mock::runtime::Call;
use crate::mock::runtime::Extrinsic;
use crate::mock::runtime::Origin;
use crate::mock::runtime::Runtime;
use crate::mock::runtime::System;
use crate::mock::runtime::TaskScheduler;
use crate::mock::ExtBuilder;
use crate::mock::task::MockTask;
use crate::mock::Trivial;
use crate::mock::WithWorkerHook;
use crate::ocw::StorageValueRef;
use crate::tasks::storage_key;
use crate::tasks::TaskScheduler as TaskSchedulerT;
use crate::tasks::{task_lock, TaskV2};
use crate::Pallet;
use assert_matches::assert_matches;
use codec::{Decode, Encode};
use frame_support::assert_ok;
use frame_support::dispatch::Dispatchable;
use sp_io::offchain::sleep_until;
use sp_runtime::offchain::storage_lock::{BlockAndTime, Lockable};
use sp_runtime::offchain::Duration;
use sp_runtime::traits::IdentifyAccount;

type GuardDeadline = <BlockAndTime<System> as Lockable>::Deadline;

#[test]
//#[tracing_test::traced_test]
fn completed_oversubscribed_tasks_are_skipped() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	let acct_pubkey = ext_builder.generate_authority();
	let pool = ext_builder.with_pool();
	ext_builder.with_offchain();
	let auth = AccountId::from(acct_pubkey.into_account().0);
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		//register twice (oversubscribe) under different expiration (aka deadline).

		let deadline = Runtime::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		Runtime::insert(&deadline, &id, task.clone());

		roll_to::<Trivial>(2);

		let deadline_2 = Runtime::deadline();
		Runtime::insert(&deadline_2, &id, task);

		roll_to::<WithWorkerHook>(3);

		//We now have 2 enqueued tasks.
		let tx = pool.write().transactions.pop().expect("A single task");
		// No more tasks
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			tx.call,
			Call::System(frame_system::pallet::Call::remark_with_event { remark: 0.encode() })
		);

		assert_ok!(tx.call.dispatch(Origin::signed(auth)));

		roll_to::<WithWorkerHook>(deadline_2);

		//task expires without yielding txns.
		assert!(pool.read().transactions.is_empty());

		let key = storage_key(&id);
		//lock set
		assert!(StorageValueRef::persistent(key.as_ref())
			.get::<GuardDeadline>()
			.expect("decoded")
			.is_some());
	});
}

//tasks can be oversubscribed with different deadlines
#[test]
fn task_deadline_oversubscription() {
	let ext_builder = ExtBuilder::default().with_keystore();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		//register twice under different expiration aka deadline
		let deadline = Runtime::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		Runtime::insert(&deadline, &id, task.clone());

		roll_to::<Trivial>(2);

		//register twice under different expiration aka deadline
		let deadline_2 = Runtime::deadline();
		Runtime::insert(&deadline_2, &id, task);

		roll_to::<WithWorkerHook>(3);

		//insertion checks
		assert!(Runtime::is_scheduled(&deadline, &id));
		assert!(Runtime::is_scheduled(&deadline_2, &id));

		assert!(TaskScheduler::pending_tasks(deadline, id).is_some());
		assert!(TaskScheduler::pending_tasks(deadline_2, id).is_some());
	});
}

#[test]
#[tracing_test::traced_test]
fn evaluation_error_is_retried() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	ext_builder.generate_authority();
	ext_builder.with_offchain();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		let deadline = Runtime::deadline();
		let task = MockTask::Evaluation;
		let id = TaskV2::<Runtime>::to_id(&task);
		Runtime::insert(&deadline, &id, task);

		roll_to::<WithWorkerHook>(2);
		assert!(logs_contain("Failed to verify pending task Evaluation"));
		// It failed Evaluation and remains scheduled.
		assert!(Runtime::is_scheduled(&deadline, &id));

		let key = storage_key(&id);
		assert!(StorageValueRef::persistent(key.as_ref())
			.get::<GuardDeadline>()
			.expect("decoded")
			.is_none());
	});
}

#[test]
#[tracing_test::traced_test]
fn forget_task_guard_when_task_has_benn_persisted() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	ext_builder.generate_authority();
	ext_builder.with_offchain();
	ext_builder.with_pool();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		let deadline = Runtime::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		Runtime::insert(&deadline, &id, task.clone());

		roll_to::<WithWorkerHook>(2);
		let key = crate::tasks::storage_key(&id);
		let mut lock = crate::tasks::task_lock::<Runtime>(&key);
		let lock_deadline = lock.try_lock().map(|_| ()).expect_err("deadline");
		sleep_until(lock_deadline.timestamp.add(Duration::from_millis(1)));

		let deadline = Runtime::deadline();
		Runtime::insert(&deadline, &id, task);

		roll_to::<WithWorkerHook>(lock_deadline.block_number + 1);

		assert!(logs_contain("Already handled Task"));

		let key = storage_key(&id);
		let mut lock = task_lock::<Runtime>(&key);

		let guard = lock.try_lock();
		assert!(guard.is_err());
	});
}

#[test]
#[tracing_test::traced_test]
fn offchain_worker_logs_error_when_transfer_validation_errors() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	ext_builder.generate_authority();
	ext_builder.with_offchain();
	ext_builder.with_pool();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		let deadline = Runtime::deadline();
		let task = MockTask::Scheduler;
		let id = TaskV2::<Runtime>::to_id(&task);

		Runtime::insert(&deadline, &id, task);

		roll_to::<WithWorkerHook>(2);

		assert!(logs_contain("Task verification encountered a processing error"));
	});
}

#[test]
fn effective_guard_lifetime_until_task_expiration() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	ext_builder.generate_authority();
	ext_builder.with_offchain();
	let pool = ext_builder.with_pool();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		let deadline = Runtime::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		Runtime::insert(&deadline, &id, task);

		roll_to::<WithWorkerHook>(2);

		let tx = pool.write().transactions.pop().expect("Remark");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();

		assert_eq!(
			tx.call,
			Call::System(frame_system::pallet::Call::remark_with_event { remark: 0.encode() })
		);

		let key = storage_key(&id);

		let mut lock = crate::tasks::task_lock::<Runtime>(&key);
		let lock_deadline = lock.try_lock().map(|_| ()).expect_err("deadline");

		// The task is cleaned up at the deadline even though the lock is acquirable.
		assert!(lock_deadline.block_number >= deadline - 1);
	});
}

#[test]
fn offchain_signed_tx_works() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	let acct_pubkey = ext_builder.generate_authority();
	let auth = AccountId::from(acct_pubkey.into_account().0);
	ext_builder.with_offchain();
	let pool = ext_builder.with_pool();
	ext_builder.build().execute_with(|| {
		roll_to::<Trivial>(1);

		let call =
			Call::System(frame_system::pallet::Call::remark_with_event { remark: 0.encode() });

		assert_ok!(Pallet::<Runtime>::offchain_signed_tx(auth.clone(), |_| call.clone().into(),));
		roll_to::<Trivial>(2);

		assert_matches!(pool.write().transactions.pop(), Some(tx) => {
			let tx = Extrinsic::decode(&mut &*tx).unwrap();
			assert_eq!(tx.call, call);
		});
	});
}
