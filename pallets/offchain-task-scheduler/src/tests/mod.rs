#![cfg(test)]

use crate::mock::generate_authority;
use crate::mock::runtime::{
	Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, System, TaskScheduler,
};
use crate::mocked_task::MockTask;
use crate::tasks::{TaskScheduler as TaskSchedulerT, TaskV2};
use crate::Error;
use crate::Event;
use assert_matches::assert_matches;
use frame_support::{assert_noop, assert_ok};
use runtime_utils::{generate_account, ExtBuilder, RollTo, Trivial};

#[test]
fn submit_output_yields_an_event() {
	let mut ext = ExtBuilder::default().with_keystore();
	let auth = generate_authority(&mut ext, 0);

	ext.build_sans_config().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let task = MockTask::Remark(0);
		let task_id = TaskV2::<Runtime>::to_id(&task);
		let deadline = TaskScheduler::deadline();
		let call: RuntimeCall = task.persistence_call(&task_id).unwrap().into();

		assert_ok!(TaskScheduler::submit_output(
			RuntimeOrigin::signed(auth.into()),
			deadline,
			task_id,
			Box::new(call),
			()
		));
		let mut events = System::events();

		assert_matches!(events.pop(), Some(event) => {
			assert_eq!(event.event,
				RuntimeEvent::TaskScheduler(Event::<Runtime>::TaskCompleted { task_id, result: Ok(()) })
			);
		});
	});
}

#[test]
fn submit_output_removes_the_completed_task() {
	let mut ext = ExtBuilder::default().with_keystore();
	let auth = generate_authority(&mut ext, 0);

	ext.build_sans_config().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let task = MockTask::Remark(0);
		let task_id = TaskV2::<Runtime>::to_id(&task);
		let deadline = TaskScheduler::deadline();
		let call: RuntimeCall = task.forward_task().unwrap().into();

		assert_ok!(TaskScheduler::submit_output(
			RuntimeOrigin::signed(auth.into()),
			deadline,
			task_id,
			Box::new(call),
			()
		));

		assert!(!TaskScheduler::is_scheduled(&deadline, &task_id));
	});
}

#[test]
fn submit_output_is_authorized() {
	let ext = ExtBuilder::default().with_keystore();
	let auth = generate_account("somebody");

	ext.build_sans_config().execute_with(|| {
		Trivial::<TaskScheduler, Runtime>::roll_to(1);

		let task = MockTask::Remark(0);
		let task_id = TaskV2::<Runtime>::to_id(&task);
		let deadline = TaskScheduler::deadline();
		let call: RuntimeCall = task.forward_task().unwrap().into();

		assert_noop!(
			TaskScheduler::submit_output(
				RuntimeOrigin::signed(auth),
				deadline,
				task_id,
				Box::new(call),
				()
			),
			Error::<Runtime>::UnauthorizedSubmission
		);

		assert!(!TaskScheduler::is_scheduled(&deadline, &task_id));
	});
}

#[test]
fn submit_out_wakes_up_worker_when_sampled() {
	todo!()
}
