use crate::{mock::*, REWARD_HALF_LIFE};

#[test]
fn reward_amount_genesis() {
	assert_eq!(Rewards::reward_amount(0), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_block_one() {
	assert_eq!(Rewards::reward_amount(1), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_just_before_reaching_first_halflife() {
	assert_eq!(Rewards::reward_amount(REWARD_HALF_LIFE - 1), 28_000_000_000_000_000_000);
}

#[test]
fn reward_amount_after_one_halflife() {
	assert_eq!(Rewards::reward_amount(REWARD_HALF_LIFE + 1), 950_000_000_000_000_000 * 28);
}

#[test]
fn reward_amount_after_two_halflives() {
	assert_eq!(Rewards::reward_amount(2 * REWARD_HALF_LIFE + 1), 902_500_000_000_000_000 * 28);
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

		assert_eq!(event, crate::mock::Event::Rewards(crate::Event::RewardIssued(1, 55)),);
	});
}

#[test]
fn rewards_were_issued_after_mining_blocks() {
	new_test_ext().execute_with(|| {
		let initial_balance = Balances::free_balance(1);

		assert!(initial_balance > 0);
		roll_to(10);

		let new_balance = Balances::free_balance(1);

		log::debug!("******* BALANCES={:?}, {:?}", initial_balance, new_balance);

		assert!(new_balance > initial_balance);
	});
}
