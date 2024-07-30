use super::*;
use crate::{Runtime, RuntimeCall as Call, RuntimeEvent as Event, System};
use assert_matches::assert_matches;
use frame_support::traits::PalletInfoAccess;
use frame_support::StoragePrefixedMap;
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::RawOrigin;
use pallet_scheduler::Event as SchedulerEvent;
use runtime_utils::{generate_account, ExtBuilder, RollTo, Trivial};
use sp_runtime::traits::BadOrigin;
use std::default::Default;

mod staking;
mod task_scheduling;

#[test]
fn pallet_scheduler_works() {
	ExtBuilder::default().build_sans_config().execute_with(|| {
		Trivial::<Scheduler, Runtime>::roll_to(1);
		let call =
			Call::System(frame_system::Call::remark_with_event { remark: b"dummy".to_vec() });
		{
			let boxed = Box::new(call);
			assert_ok!(Scheduler::schedule(RawOrigin::Root.into(), 4, None, 0, boxed));
		}
		Trivial::<Scheduler, Runtime>::roll_to(3);
		assert_matches!(
			System::events().pop().expect("Scheduled Event"),
			EventRecord {
				event: Event::Scheduler(SchedulerEvent::Scheduled { when: 4, index: 0 }),
				..
			}
		);
		Trivial::<Scheduler, Runtime>::roll_to(4);
		assert_matches!(
			System::events().pop().expect("Dispatched Event"),
			EventRecord {
				event: Event::Scheduler(SchedulerEvent::Dispatched { task: (4, 0), .. }),
				..
			}
		);
		Trivial::<Scheduler, Runtime>::roll_to(5);
		assert_eq!(System::events().len(), 2);
	});
}

#[test]
fn must_be_root_to_schedule() {
	ExtBuilder::default().build_sans_config().execute_with(|| {
		Trivial::<Scheduler, Runtime>::roll_to(1);
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

// #[test]
// fn authority_migration_parity_checks() {
// 	use pallet_creditcoin::migrations::v7::{Authorities as AC, SCHEDULER_PREFIX};
// 	use pallet_offchain_task_scheduler::Authorities as AT;
//
// 	//Pallet prefix
// 	let scheduler_prefix = TaskScheduler::name();
// 	assert_eq!(SCHEDULER_PREFIX, scheduler_prefix);
//
// 	//Storage Prefix
// 	assert_eq!(AT::<Runtime>::storage_prefix(), AC::<Runtime>::storage_prefix());
// }
