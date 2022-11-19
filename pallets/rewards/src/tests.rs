use crate::{mock::*, BASE_REWARD_IN_CTC, CREDO_PER_CTC, REWARD_HALF_LIFE, SAWTOOTH_PORT_HEIGHT};

#[test]
fn reward_amount_genesis() {
	//! 0 After Sawtooth
	assert_eq!(Rewards::reward_amount(0), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_block_one() {
	//! 1 After Sawtooth
	assert_eq!(Rewards::reward_amount(1), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_just_before_reaching_first_halflife() {
	assert_eq!(
		Rewards::reward_amount(REWARD_HALF_LIFE - SAWTOOTH_PORT_HEIGHT - 1),
		CREDO_PER_CTC as u128 * BASE_REWARD_IN_CTC as u128
	);
}

#[test]
fn reward_amount_after_one_halflife() {
	assert_eq!(
		Rewards::reward_amount(REWARD_HALF_LIFE - SAWTOOTH_PORT_HEIGHT + 1),
		950_000_000_000_000_000 * BASE_REWARD_IN_CTC as u128
	);
}

#[test]
fn reward_amount_after_two_halflives() {
	assert_eq!(
		Rewards::reward_amount(2 * REWARD_HALF_LIFE - SAWTOOTH_PORT_HEIGHT + 1),
		902_500_000_000_000_000 * BASE_REWARD_IN_CTC as u128
	);
}

#[test]
fn reward_amount_limit() {
	assert_eq!(Rewards::reward_amount(u64::MAX), 0);
}

#[test]
fn issue_reward_handling() {
	new_test_ext().execute_with(|| {
		Rewards::issue_reward(1, 55);

		let event = <frame_system::Pallet<Test>>::events()
			.pop()
			.expect("Expected at least one EventRecord to be found")
			.event;

		assert_eq!(
			event,
			crate::mock::RuntimeEvent::Rewards(crate::Event::<Test>::RewardIssued(1, 55)),
		);
	});
}

#[test]
fn rewards_were_issued_after_mining_blocks() {
	new_test_ext().execute_with(|| {
		let initial_balance = Balances::free_balance(1);

		assert!(initial_balance > 0);
		roll_to(10, 1);

		let new_balance = Balances::free_balance(1);
		assert!(new_balance > initial_balance);
	});
}

#[test]
fn exercise_getter() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		roll_to(10, 2);

		let author = crate::Pallet::<Test>::block_author();
		assert_eq!(author, Some(2));
	});
}
