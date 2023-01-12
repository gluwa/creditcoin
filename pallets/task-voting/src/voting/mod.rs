use super::{pallet::Rounds, Config, Error, Pallet};
use frame_support::{ensure, traits::Get, BoundedBTreeMap, BoundedBTreeSet};
use scale_info::TypeInfo;
use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};

pub mod traits;
use traits::*;

pub type Power = u64;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(Max))]
pub struct Votes<AccountId, Id, Max> {
	pub votes: BoundedBTreeMap<Id, Data<AccountId, Max>, Max>,
	pub vote_total: Power,
}

impl<AccountId: Ord, MaxVoters: Get<u32>> Data<AccountId, MaxVoters> {
	pub fn add_voter<T: Config>(&mut self, voter: AccountId, power: Power) -> Result<(), Error<T>> {
		if !self.voters.try_insert(voter).map_err(|_| Error::TooManyVoters)? {
			return Err(Error::DuplicateVoter);
		}

		self.total_voting_power += power;

		Ok(())
	}

	pub fn new() -> Self {
		Self { total_voting_power: 0, voters: BoundedBTreeSet::new() }
	}
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(Max))]
pub struct Data<AccountId, Max> {
	pub total_voting_power: Power,
	pub voters: BoundedBTreeSet<AccountId, Max>,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct Summary<OutputId> {
	vote_total: Power,
	winning_vote_total: Power,
	winning_data: OutputId,
	runner_up_total: Power,
}

impl<AccountId, OutputId: Ord, MaxVoters> Votes<AccountId, OutputId, MaxVoters> {
	pub fn iter(
		&self,
	) -> sp_std::collections::btree_map::Iter<'_, OutputId, Data<AccountId, MaxVoters>> {
		self.votes.iter()
	}
}

impl<AccountId, OutputId: Ord + Clone, MaxVoters> Votes<AccountId, OutputId, MaxVoters> {
	pub fn tally_votes<T: Config>(&self) -> Result<Summary<OutputId>, Error<T>> {
		let mut best_data = None;
		let mut best_power = 0;
		let mut second_best_power = 0;

		for (id, vote) in self.iter() {
			if vote.total_voting_power > best_power {
				second_best_power = best_power;
				best_power = vote.total_voting_power;
				best_data = Some(id);
			} else if vote.total_voting_power > second_best_power {
				second_best_power = vote.total_voting_power;
			}
		}
		let best_data = best_data.ok_or(Error::NoWinner)?;

		let summary = Summary {
			vote_total: self.vote_total,
			winning_vote_total: best_power,
			winning_data: best_data.clone(),
			runner_up_total: second_best_power,
		};

		Ok(summary)
	}
}

impl<T: Config> Pallet<T> {
	pub fn submit_vote(
		task: T::TaskId,
		output: T::OutputId,
		voter: T::AccountId,
	) -> Result<(), Error<T>> {
		let mut round = Rounds::<T>::get(&task).ok_or(Error::UnknownTask)?;

		let power = T::VotingProvider::voting_power_of(&task, &voter)?;

		if let Some(vote) = round.votes.get_mut(&output) {
			vote.add_voter(voter, power)?;
		} else {
			let mut vote = Data::new();
			vote.add_voter(voter, power)?;
			round.votes.try_insert(output, vote).map_err(|_| Error::TooManyVoters)?;
		}

		Rounds::<T>::insert(task, round);
		Ok(())
	}

	pub fn try_conclude_voting(task_id: T::TaskId) -> Result<(), Error<T>> {
		let round = Rounds::<T>::get(&task_id).ok_or(Error::UnknownTask)?;

		if T::VotingProvider::meets_quorum(&task_id, &round) {
			let summary = round.tally_votes()?;
			T::OnVoteConclusion::voting_concluded(&task_id, summary, &round);
			Rounds::<T>::remove(&task_id);
		}

		Ok(())
	}

	pub fn start_task_voting(
		task_id: T::TaskId,
		initial_vote: Option<(T::AccountId, T::OutputId)>,
	) -> Result<(), Error<T>> {
		ensure!(!Rounds::<T>::contains_key(&task_id), Error::VoteInProgress);
		let mut votes = BoundedBTreeMap::new();
		let vote_total = if let Some((voter, data)) = initial_vote {
			let power = T::VotingProvider::voting_power_of(&task_id, &voter)?;
			let mut vote = Data::new();
			vote.add_voter(voter, power)?;
			votes.try_insert(data, vote).map_err(|_| Error::TooManyVoters)?;
			power
		} else {
			0
		};

		let votes = Votes { votes, vote_total };
		Rounds::<T>::insert(task_id, votes);
		Ok(())
	}
}
