#![cfg(test)]

use super::{StorageLock, Task};
use crate::{
	mock::{roll_by_with_ocw, set_rpc_uri, AccountId, ExtBuilder, MockedRpcRequests, Origin, Test},
	ocw::{
		errors::{OffchainError, VerificationFailureCause as Cause},
		rpc::{EthTransaction, EthTransactionReceipt, JsonRpcResponse},
		EncodeLike, ETH_CONFIRMATIONS,
	},
	pallet::{Config, Store},
	tests::{generate_address_with_proof, HexToAddress},
	types::CollectedCoinsId,
	ExternalAddress, Pallet as Creditcoin,
};
use assert_matches::assert_matches;
use codec::Decode;
use ethereum_types::{H160, U64};
use frame_support::{assert_noop, assert_ok, once_cell::sync::Lazy};
use frame_system::Pallet as System;
use pallet_timestamp::Pallet as Timestamp;
use sp_core::offchain::Duration;
use sp_io::offchain;
use sp_runtime::{
	offchain::{
		storage::StorageValueRef,
		storage_lock::{BlockAndTime, Lockable, Time},
	},
	traits::{BlockNumberProvider, IdentifyAccount},
};
use std::convert::{TryFrom, TryInto};
use std::sync::{
	atomic::{AtomicU8, Ordering},
	Arc,
};
use std::thread;

#[test]
fn lock_released_when_guard_is_dropped() {
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|state, _| {
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
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|_, _| {
		let mut l1 = StorageLock::<'_, Time>::new(b"id_1");
		let g = l1.try_lock();
		g.expect("ok").forget();
		let g = l1.try_lock();
		assert!(g.is_err());
	});
}

#[test]
fn lock_expires() {
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|_, _| {
		System::<Test>::set_block_number(1);
		let mut l1 = StorageLock::<'_, BlockAndTime<System<Test>>>::with_block_and_time_deadline(
			b"id_1",
			1,
			Duration::from_millis(0),
		);
		let g = l1.try_lock().expect("ok");
		g.forget();
		System::<Test>::set_block_number(3);
		let sleep_until = offchain::timestamp().add(Duration::from_millis(1));
		offchain::sleep_until(sleep_until);
		let g = l1.try_lock();
		assert!(g.is_ok());
	});
}

#[test]
fn lock_mutual_exclusion() {
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|state, _| {
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
