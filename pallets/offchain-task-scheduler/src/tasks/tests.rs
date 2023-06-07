#![cfg(test)]

use super::StorageLock;
use crate::mock::runtime::Runtime;
use frame_system::Pallet as System;
use runtime_utils::ExtBuilder;
use sp_core::offchain::Duration;
use sp_io::offchain;
use sp_runtime::offchain::storage_lock::{BlockAndTime, Time};

#[test]
fn lock_released_when_guard_is_dropped() {
	let mut ext = ExtBuilder::default();
	let state = ext.with_offchain();
	ext.build_sans_config().execute_with(|| {
		let key = b"id_1";
		let mut l1 = StorageLock::<'_, Time>::new(key);
		let g = l1.try_lock();
		assert!(g.is_ok());
		drop(g);
		assert!(state.read().persistent_storage.get(key).is_none());
	});
}

#[test]
fn lock_guard_is_kept_alive() {
	let mut ext = ExtBuilder::default();
	ext.with_offchain();
	ext.build_sans_config().execute_with(|| {
		let mut l1 = StorageLock::<'_, Time>::new(b"id_1");
		let g = l1.try_lock();
		g.expect("ok").forget();
		let g = l1.try_lock();
		assert!(g.is_err());
	});
}

#[test]
fn lock_expires() {
	let mut ext = ExtBuilder::default();
	ext.with_offchain();
	ext.build_sans_config().execute_with(|| {
		System::<Runtime>::set_block_number(1);
		let mut l1 = StorageLock::<'_, BlockAndTime<System<Runtime>>>::with_block_and_time_deadline(
			b"id_1",
			1,
			Duration::from_millis(0),
		);
		let g = l1.try_lock().expect("ok");
		g.forget();
		System::<Runtime>::set_block_number(3);
		let sleep_until = offchain::timestamp().add(Duration::from_millis(1));
		offchain::sleep_until(sleep_until);
		let g = l1.try_lock();
		assert!(g.is_ok());
	});
}

#[test]
fn lock_mutual_exclusion() {
	let mut ext = ExtBuilder::default();
	let state = ext.with_offchain();
	ext.build_sans_config().execute_with(|| {
		let mut l1 = StorageLock::<'_, Time>::new(b"id_1");
		let mut l2 = StorageLock::<'_, Time>::new(b"id_2");

		let g1 = l1.try_lock().expect("ok");
		g1.forget();
		let g1 = l1.try_lock();
		assert!(g1.is_err());
		//won't release because it was not a guard.
		drop(g1);
		let g2 = l2.try_lock().expect("ok");
		drop(g2);
		let g2 = l2.try_lock();
		assert!(g2.is_ok());

		let g1 = l1.try_lock();
		assert!(g1.is_err());
		drop(g2);
		let deadline = state.read().persistent_storage.get(b"id_2");
		assert!(deadline.is_none());
		let deadline = state.read().persistent_storage.get(b"id_1");
		assert!(deadline.is_some());
	});
}
