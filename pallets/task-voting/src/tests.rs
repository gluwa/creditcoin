use core::{borrow::Borrow, sync::atomic::AtomicU64};
use sp_core::bounded_btree_map;

use crate::{mock::*, VoterPower, VotingPower};

fn new_voter() -> u64 {
	static VOTER_COUNT: AtomicU64 = AtomicU64::new(0);
	VOTER_COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}

fn voting_power_of(task: impl Borrow<TaskId>, voter: impl Borrow<AccountId>) -> VotingPower {
	<Voting as VoterPower<Test>>::voting_power_of(task.borrow(), voter.borrow()).unwrap()
}

#[test]
pub fn start_task_voting_works() {
	new_test_ext().execute_with(|| {
		let task = 0;
		let voter = new_voter();
		let datum = 1;

		Pallet::start_task_voting(task, Some((voter, datum))).unwrap();

		let power = voting_power_of(task, voter);

		let votes = Votes {
			vote_total: power,
			votes: bounded_btree_map!(datum => [(voter, power)].into_iter().collect()),
		};

		assert_eq!(ActiveVotes::get(task).unwrap(), votes);
	});
}
