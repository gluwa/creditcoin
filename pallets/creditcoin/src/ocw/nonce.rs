use crate::{Config, Pallet};
use alloc::vec::Vec;
use parity_scale_codec::Encode;
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
	use crate::helpers::HexToAddress;
	use crate::mock::{roll_to, roll_to_with_ocw, ExtBuilder, RuntimeOrigin, Test};
	use crate::ocw::errors::VerificationFailureCause as Cause;
	use crate::ocw::tasks::collect_coins::tests::mock_rpc_for_collect_coins;
	use crate::ocw::tasks::collect_coins::{testing_constants::CHAIN, tests::TX_HASH};
	use crate::tests::generate_address_with_proof;
	use crate::types::{Address, AddressId};
	use crate::Pallet as Creditcoin;
	use assert_matches::assert_matches;
	use core::sync::atomic::AtomicU64;
	use frame_support::assert_ok;
	use frame_system::Config as SystemConfig;
	use frame_system::Pallet as System;
	use parity_scale_codec::Decode;
	use sp_runtime::offchain::storage::StorageValueRef;
	use sp_runtime::offchain::testing::TestOffchainExt;
	use sp_runtime::traits::IdentifyAccount;
	use std::sync::atomic::Ordering;
	use std::sync::Arc;

	#[test]
	fn incremented_for_persisted_task() {
		let mut ext = ExtBuilder::default();
		let pkey = ext.generate_authority();
		let acct = <Test as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|state, _| {
			mock_rpc_for_collect_coins(&state);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");
			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			roll_to(1);
			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc),
				addr,
				TX_HASH.hex_to_address()
			));
			roll_to_with_ocw(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce: u64 = synced_nonce.get().unwrap().unwrap();
			assert_eq!(synced_nonce, 1u64);
			let nonce = System::<Test>::account(acct).nonce;
			assert_eq!(nonce, 1u64);
		});
	}

	#[test]
	fn incremented_for_failed_task() {
		let mut ext = ExtBuilder::default();
		let pkey = ext.generate_authority();
		let acct = <Test as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|state, pool| {
			mock_rpc_for_collect_coins(&state);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");
			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			let mut fake = addr;
			fake[0] = 0xff;
			let address_id = AddressId::new::<Test>(&CHAIN, &fake);
			let entry = Address { blockchain: CHAIN, value: fake.clone(), owner: acc.clone() };
			crate::Addresses::<Test>::insert(address_id, entry);

			roll_to(1);
			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc),
				fake.clone(),
				TX_HASH.hex_to_address()
			));

			let deadline = Test::unverified_transfer_deadline();
			roll_to_with_ocw(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce: u64 = synced_nonce.get().unwrap().unwrap();
			assert_eq!(synced_nonce, 1u64);
			let nonce = System::<Test>::account(acct).nonce;
			assert_eq!(nonce, 1u64);

			let expected_collected_coins_id = crate::CollectedCoinsId::new::<crate::mock::Test>(
				&CHAIN,
				&TX_HASH.hex_to_address(),
			);

			let call = crate::Call::<crate::mock::Test>::fail_task {
				task_id: expected_collected_coins_id.into(),
				cause: Cause::IncorrectSender,
				deadline,
			};

			assert_matches!(pool.write().transactions.pop(),
				Some(tx) => {
					let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
					assert_eq!(tx.call, crate::mock::RuntimeCall::Creditcoin(call));
				}
			);
		});
	}

	#[test]
	fn unique_per_account() {
		let mut ext = ExtBuilder::default();
		let pkey = ext.generate_authority();
		let acct_1 = <Test as SystemConfig>::AccountId::from(pkey.into_account().0);
		let pkey = ext.generate_authority();
		let acct_2 = <Test as SystemConfig>::AccountId::from(pkey.into_account().0);
		assert!(nonce_key(&acct_1) != nonce_key(&acct_2));
		assert!(lock_key(&acct_1) != lock_key(&acct_2));
	}

	#[test]
	fn not_incremented_on_task_error() {
		let mut ext = ExtBuilder::default();
		let pkey = ext.generate_authority();
		let acct = <Test as SystemConfig>::AccountId::from(pkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, pool| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");
			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			roll_to(1);
			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc),
				addr,
				TX_HASH.hex_to_address()
			));
			roll_to_with_ocw(2);

			let key = &nonce_key(&acct);
			let synced_nonce = StorageValueRef::persistent(key);
			let synced_nonce = synced_nonce.get::<u64>().unwrap();
			assert!(synced_nonce.is_none());
			let nonce = System::<Test>::account(acct).nonce;
			assert_eq!(nonce, 0u64);

			assert!(pool.write().transactions.is_empty());
		});
	}

	#[test]
	fn parallel_increment() {
		let (offchain, _) = TestOffchainExt::new();
		const THREADS: u32 = 3;
		let nonces = Arc::new(AtomicU64::new(0));

		let handles = (0..THREADS).into_iter().map(|_| {
			let offchain = offchain.clone();
			let nonces = nonces.clone();

			std::thread::spawn(move || {
				let mut ext_builder = ExtBuilder::default();
				let acct_pubkey = ext_builder.generate_authority();
				let acct = <Test as SystemConfig>::AccountId::from(acct_pubkey.into_account().0);
				let expected_collected_coins_id =
					crate::CollectedCoinsId::new::<Test>(&CHAIN, &[0]);
				let (mut ext, pool) = ext_builder.build_with(offchain);
				let execute = || {
					crate::mock::roll_to(1);
					let call = crate::Call::<Test>::fail_task {
						task_id: expected_collected_coins_id.into(),
						cause: Cause::AbiMismatch,
						deadline: Test::unverified_transfer_deadline(),
					};
					assert_ok!(crate::Pallet::<Test>::submit_txn_with_synced_nonce(
						acct.clone(),
						|_| call.clone(),
					));

					assert_matches!(pool.write().transactions.pop(),
						Some(tx) => {
							let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
							assert_eq!(tx.call, crate::mock::RuntimeCall::Creditcoin(call));
						}
					);

					let nonce = System::<Test>::account(acct).nonce;
					nonces.fetch_add(nonce, Ordering::Relaxed);
				};

				ext.execute_with(execute);
			})
		});

		for h in handles {
			h.join().expect("testing context is shared");
		}

		let ext_builder = ExtBuilder::default();
		let (mut ext, _) = ext_builder.build_with(offchain);
		ext.execute_with(|| {
			let nonce_post_submition_sum = (THREADS) * (THREADS + 1) / 2;
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
				let mut ext_builder = ExtBuilder::default();
				let acct_pubkey = ext_builder.generate_authority();
				let acct = <Test as SystemConfig>::AccountId::from(acct_pubkey.into_account().0);
				let (mut ext, _) = ext_builder.build_with(offchain);

				let execute = || {
					crate::mock::roll_to(1);

					let key = lock_key(&acct);
					let mut lock = Pallet::<Test>::nonce_lock_new(&key);
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
		let ext = ExtBuilder::default();
		ext.build_offchain_and_execute_with_state(|_, _| {
			System::<Test>::set_block_number(1);

			let key = &b"lock_key"[..];
			let mut lock = Pallet::<Test>::nonce_lock_new(key);
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
