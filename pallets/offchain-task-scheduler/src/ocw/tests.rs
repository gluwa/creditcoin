#![cfg(test)]

use crate::{
	mock::{
		generate_authority,
		runtime::{
			AccountId, Extrinsic, Runtime, RuntimeCall, RuntimeOrigin, System, TaskScheduler,
		},
	},
	mocked_task::MockTask,
	ocw::StorageValueRef,
	tasks::{storage_key, task_lock, TaskScheduler as TaskSchedulerT, TaskV2},
	Pallet,
};
use assert_matches::assert_matches;
use frame_support::assert_ok;
use frame_support::dispatch::Dispatchable;
use runtime_utils::{
	pool::with_failing_submit_transaction, ExtBuilder, RollTo, Trivial, WithWorkerHook,
};
use sp_io::offchain::sleep_until;
use sp_runtime::codec::{Decode, Encode};
use sp_runtime::offchain::storage_lock::{BlockAndTime, Lockable};
use sp_runtime::offchain::Duration;
use sp_runtime::traits::IdentifyAccount;

type GuardDeadline = <BlockAndTime<System> as Lockable>::Deadline;

#[test]
fn completed_oversubscribed_tasks_are_skipped() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	let acct_pubkey = generate_authority(&mut ext_builder, 0);
	let pool = ext_builder.with_pool();
	ext_builder.with_offchain();
	let auth = AccountId::from(acct_pubkey.into_account().0);
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		//register twice (oversubscribe) under different expiration (aka deadline).

		let deadline = TaskScheduler::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		TaskScheduler::insert(&deadline, &id, task.clone());

		Trivial::<TaskScheduler, Runtime>::roll_to(2);

		let deadline_2 = TaskScheduler::deadline();
		TaskScheduler::insert(&deadline_2, &id, task);

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(3);

		//We now have 2 enqueued tasks.
		let tx = pool.write().transactions.pop().expect("A single task");
		// No more tasks
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			tx.call,
			RuntimeCall::System(frame_system::pallet::Call::remark_with_event {
				remark: 0.encode()
			})
		);

		assert_ok!(tx.call.dispatch(RuntimeOrigin::signed(auth)));

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(deadline_2);

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
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		//register twice under different expiration aka deadline
		let deadline = TaskScheduler::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		TaskScheduler::insert(&deadline, &id, task.clone());

		Trivial::<TaskScheduler, Runtime>::roll_to(2);

		//register twice under different expiration aka deadline
		let deadline_2 = TaskScheduler::deadline();
		TaskScheduler::insert(&deadline_2, &id, task);

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(3);

		//insertion checks
		assert!(TaskScheduler::is_scheduled(&deadline, &id));
		assert!(TaskScheduler::is_scheduled(&deadline_2, &id));

		assert!(TaskScheduler::pending_tasks(deadline, id).is_some());
		assert!(TaskScheduler::pending_tasks(deadline_2, id).is_some());
	});
}

#[test]
fn evaluation_error_is_retried() {
	let logs = traced_test::trace();
	let mut ext_builder = ExtBuilder::default().with_keystore();
	generate_authority(&mut ext_builder, 0);
	ext_builder.with_offchain();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let deadline = TaskScheduler::deadline();
		let task = MockTask::Evaluation;
		let id = TaskV2::<Runtime>::to_id(&task);
		TaskScheduler::insert(&deadline, &id, task);

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);
		assert!(logs.contain("Failed to verify pending task Evaluation"));
		// It failed Evaluation and remains scheduled.
		assert!(TaskScheduler::is_scheduled(&deadline, &id));

		let key = storage_key(&id);
		assert!(StorageValueRef::persistent(key.as_ref())
			.get::<GuardDeadline>()
			.expect("decoded")
			.is_none());
	});
}

#[test]
fn forget_task_guard_when_task_has_been_persisted() {
	let logs = traced_test::trace();
	let mut ext_builder = ExtBuilder::default().with_keystore();
	generate_authority(&mut ext_builder, 0);
	ext_builder.with_offchain();
	ext_builder.with_pool();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let deadline = TaskScheduler::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		TaskScheduler::insert(&deadline, &id, task.clone());

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);
		let key = crate::tasks::storage_key(&id);
		let mut lock = crate::tasks::task_lock::<Runtime>(&key);
		let lock_deadline = lock.try_lock().map(|_| ()).expect_err("deadline");
		sleep_until(lock_deadline.timestamp.add(Duration::from_millis(1)));

		let deadline = TaskScheduler::deadline();
		TaskScheduler::insert(&deadline, &id, task);

		//fake a task being in storage.
		crate::mocked_task::is_persisted_replace(true);
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(lock_deadline.block_number + 1);

		assert!(logs.contain("Already handled Task"));

		let key = storage_key(&id);
		let mut lock = task_lock::<Runtime>(&key);

		let guard = lock.try_lock();
		assert!(guard.is_err());

		//revert thread_local
		crate::mocked_task::is_persisted_replace(false);
	});
}

#[test]
fn offchain_worker_logs_error_when_transfer_validation_errors() {
	let logs = traced_test::trace();
	let mut ext_builder = ExtBuilder::default().with_keystore();
	generate_authority(&mut ext_builder, 0);
	ext_builder.with_offchain();
	ext_builder.with_pool();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let deadline = TaskScheduler::deadline();
		let task = MockTask::Scheduler;
		let id = TaskV2::<Runtime>::to_id(&task);

		TaskScheduler::insert(&deadline, &id, task);

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);

		assert!(logs.contain("Task verification encountered a processing error"));
	});
}

#[test]
fn effective_guard_lifetime_until_task_expiration() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	generate_authority(&mut ext_builder, 0);
	ext_builder.with_offchain();
	let pool = ext_builder.with_pool();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let deadline = TaskScheduler::deadline();
		let task = MockTask::Remark(0);
		let id = TaskV2::<Runtime>::to_id(&task);
		TaskScheduler::insert(&deadline, &id, task);

		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(2);

		let tx = pool.write().transactions.pop().expect("Remark");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();

		assert_eq!(
			tx.call,
			RuntimeCall::System(frame_system::pallet::Call::remark_with_event {
				remark: 0.encode()
			})
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
	let acct_pubkey = generate_authority(&mut ext_builder, 0);
	let pool = ext_builder.with_pool();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let call = RuntimeCall::System(frame_system::pallet::Call::remark_with_event {
			remark: 0.encode(),
		});

		assert_ok!(Pallet::<Runtime>::offchain_signed_tx(acct_pubkey.into(), |_| call.clone()));
		Trivial::<TaskScheduler, Runtime>::roll_to(2);

		assert_matches!(pool.write().transactions.pop(), Some(tx) => {
			let tx = Extrinsic::decode(&mut &*tx).unwrap();
			assert_eq!(tx.call, call);
		});
	});
}

#[test]
fn offchain_signed_tx_send_fails() {
	let mut ext_builder = ExtBuilder::default().with_keystore();
	let acct_pubkey = generate_authority(&mut ext_builder, 0);
	ext_builder.with_pool();
	ext_builder.build::<Runtime>().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let call = RuntimeCall::System(frame_system::pallet::Call::remark_with_event {
			remark: 0.encode(),
		});

		use frame_support::assert_err;
		with_failing_submit_transaction(|| {
			assert_err!(
				Pallet::<Runtime>::offchain_signed_tx(acct_pubkey.into(), |_| call.clone()),
				crate::Error::OffchainSignedTxFailed
			);
		})
	});
}

#[test]
fn offchain_worker_should_log_when_authority_is_missing() {
	let logs = traced_test::trace();
	ExtBuilder::default().with_keystore().build_sans_config().execute_with(|| {
		WithWorkerHook::<TaskScheduler, Runtime>::roll_to(1);

		assert!(logs.contain("Not an authority, skipping offchain work"));
	});
}
