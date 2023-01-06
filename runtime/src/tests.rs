use super::*;
use crate::{Runtime, RuntimeCall as Call, RuntimeEvent as Event, System};
use assert_matches::assert_matches;
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::RawOrigin;
use pallet_scheduler::Event as SchedulerEvent;
use runtime_utils::{roll_to, ExtBuilder, Trivial};
use sp_core::Pair;
use sp_runtime::{
	traits::{BadOrigin, IdentifyAccount},
	AccountId32, MultiSigner,
};
use std::default::Default;

fn generate_account(seed: &str) -> AccountId32 {
	let seed = seed.bytes().cycle().take(32).collect::<Vec<_>>();
	let key_pair = sp_core::ecdsa::Pair::from_seed_slice(seed.as_slice()).unwrap();
	let pkey = key_pair.public();
	let signer: MultiSigner = pkey.into();
	signer.into_account()
}

#[test]
fn pallet_scheduler_works() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		roll_to::<Trivial, Runtime, Scheduler>(1);
		let call =
			Call::System(frame_system::Call::remark_with_event { remark: b"dummy".to_vec() });
		{
			let boxed = Box::new(call);
			assert_ok!(Scheduler::schedule(RawOrigin::Root.into(), 4, None, 0, boxed));
		}
		roll_to::<Trivial, Runtime, Scheduler>(3);
		assert_matches!(
			System::events().pop().expect("Scheduled Event"),
			EventRecord {
				event: Event::Scheduler(SchedulerEvent::Scheduled { when: 4, index: 0 }),
				..
			}
		);
		roll_to::<Trivial, Runtime, Scheduler>(4);
		assert_matches!(
			System::events().pop().expect("Dispatched Event"),
			EventRecord {
				event: Event::Scheduler(SchedulerEvent::Dispatched { task: (4, 0), .. }),
				..
			}
		);
		roll_to::<Trivial, Runtime, Scheduler>(5);
		assert_eq!(System::events().len(), 2);
	});
}

#[test]
fn must_be_root_to_schedule() {
	ExtBuilder::<()>::default().build_sans_config().execute_with(|| {
		roll_to::<Trivial, Runtime, Scheduler>(1);
		let call =
			Call::System(frame_system::Call::remark_with_event { remark: b"dummy".to_vec() });
		let boxed = Box::new(call.clone());
		let acc = generate_account("Somebody");
		assert_noop!(
			Scheduler::schedule(RawOrigin::Signed(acc).into(), 4, None, 0, boxed),
			BadOrigin
		);
		let boxed = Box::new(call);
		assert_noop!(Scheduler::schedule(RawOrigin::None.into(), 4, None, 0, boxed), BadOrigin);
	});
}
