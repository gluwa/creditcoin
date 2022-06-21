#![cfg(test)]
#![allow(unused_imports)]

use std::convert::{TryFrom, TryInto};

use assert_matches::assert_matches;
use codec::Decode;
use ethereum_types::{H160, U64};
use frame_support::{assert_noop, assert_ok, once_cell::sync::Lazy};
use frame_system::Pallet as System;
use sp_runtime::traits::IdentifyAccount;

use crate::mock::{
	roll_by_with_ocw, set_rpc_uri, AccountId, ExtBuilder, MockedRpcRequests, Origin, Test,
};
use crate::ocw::task::guard::LocalTaskStatus;
use crate::ocw::{
	errors::{OffchainError, VerificationFailureCause as Cause},
	rpc::{EthTransaction, EthTransactionReceipt},
	ETH_CONFIRMATIONS,
};
use crate::tests::generate_address_with_proof;
use crate::tests::RefstrExt;
use crate::types::CollectedCoinsId;
use crate::Pallet as Creditcoin;
use crate::{ocw::rpc::JsonRpcResponse, ExternalAddress};

struct NoOp;

use super::Task;
use crate::ocw::EncodeLike;
use crate::pallet::{Config, Store};

impl<T, K2> Task<T, T::BlockNumber, K2> for NoOp
where
	T: Config,
	K2: EncodeLike<T::Hash>,
{
	type VerifiedTask = ();

	fn verify(&self) -> crate::ocw::VerificationResult<Self::VerifiedTask> {
		Ok(())
	}

	fn failure_call(&self, _deadline: T::BlockNumber, _cause: Cause) -> crate::Call<T> {
		todo!()
	}

	fn success_call(
		&self,
		_deadline: T::BlockNumber,
		_verified_task: Self::VerifiedTask,
	) -> crate::Call<T> {
		todo!()
	}

	fn is_complete(_persistent_storage_key: K2) -> bool {
		false
	}
}

use std::sync::{
	atomic::{AtomicU8, Ordering},
	Arc,
};

use sp_runtime::offchain::storage::StorageValueRef;

// spawn multiple threads and count atomically
#[test]
fn concurrent_access() {
	let a = Arc::new(AtomicU8::new(0));
	let x = a.clone();
	let handles: Vec<_> = (0..10)
		.into_iter()
		.map(move |_| {
			let x = x.clone();
			std::thread::spawn(move || {
				let mut wins = 0;
				let a = x.clone();
				while wins < 10 {
					a.fetch_add(1, Ordering::Relaxed);
					wins += 1;
				}
			})
		})
		.collect();
	for h in handles {
		let _ = h.join();
	}

	assert_eq!(a.load(Ordering::Acquire), 100);
}

// spawn multiple threads and count atomically, it should fail as the context is isolated.
#[test]
fn concurrent_storage() {
	let storage_key = b"demo_status";

	let handles: Vec<_> = (0..20)
		.into_iter()
		.map(move |_| {
			std::thread::spawn(move || {
				let ext = ExtBuilder::default();
				ext.build_offchain_and_execute_with_state(|_, _| {
					let mut tries = 0;
					let a = StorageValueRef::persistent(storage_key);
					while tries < 10_000 {
						tries += 1;
						let res = a.mutate::<u32, (), _>(|a: Result<Option<u32>, _>| {
							let v = if let Ok(a) = a { a } else { None };
							match v {
								Some(a) => Ok(a + 1),
								None => Ok(1),
							}
						});
						match res {
							Ok(_) => (),
							Err(e) => panic!("{:?}", e),
						};
					}

					let val =
						StorageValueRef::persistent(storage_key).get::<u32>().unwrap().unwrap();
					assert!(val != 10_000u32);
				});
			})
		})
		.collect();

	for h in handles {
		h.join().expect_err("assertion panics");
	}
}

//single thread /state tests
// OCW mutex per guard
// Guard is released when dropped
// Guard is kept alive
// Guard concurrent failure
// broken status is recoverable
// Clearing Guard is atomic

//fast/slow path always transition to processing
//decode failure trnasitions to processing
//Droppping transitions to none; dropping with keep_alive trnasitions to enqueued;
