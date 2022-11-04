#![cfg(test)]

use crate::mock::roll_to;
use crate::mock::runtime::AccountId;
use crate::mock::runtime::Call;
use crate::mock::runtime::Extrinsic;
use crate::mock::runtime::Origin;
use crate::mock::runtime::Runtime;
use crate::mock::runtime::System;
use crate::mock::runtime::TaskScheduler;
use crate::mock::task::MockTask;
use crate::mock::ExtBuilder;
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
