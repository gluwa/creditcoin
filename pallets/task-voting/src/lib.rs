#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;
pub use pallet::*;
use parity_scale_codec::MaxEncodedLen;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;
mod voting;
use voting::traits::{OnVoteConclusion, QuorumMet, VoterPower};
use voting::{Power as VotingPower, Votes};

mod sampling;

pub mod staking;

#[allow(clippy::unnecessary_cast)]
pub mod weights;

pub type RoundOf<T> = Votes<
	<T as frame_system::Config>::AccountId,
	<T as Config>::OutputId,
	<T as Config>::MaxVoters,
>;

pub struct VotingProviderStrategy<P, Q>(PhantomData<(P, Q)>);

impl<T: Config, P: VoterPower<T>, Q> VoterPower<T> for VotingProviderStrategy<P, Q> {
	fn voting_power_of(task: &T::TaskId, voter: &T::AccountId) -> Result<VotingPower, Error<T>> {
		P::voting_power_of(task, voter)
	}
}

impl<T: Config, P, Q: QuorumMet<T>> QuorumMet<T> for VotingProviderStrategy<P, Q> {
	fn meets_quorum(task: &T::TaskId, votes: &RoundOf<T>) -> bool {
		Q::meets_quorum(task, votes)
	}
}

pub struct AtLeastOneVote;

impl<T: Config> QuorumMet<T> for AtLeastOneVote {
	fn meets_quorum(_task: &T::TaskId, votes: &RoundOf<T>) -> bool {
		votes.iter().any(|(_, vote)| vote.total_voting_power > 0)
	}
}

pub struct UniformVoterPower;

impl<T: Config> VoterPower<T> for UniformVoterPower {
	fn voting_power_of(_task: &T::TaskId, _voter: &T::AccountId) -> Result<VotingPower, Error<T>> {
		Ok(1)
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Get, Identity, Parameter};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type TaskId: Parameter + Ord + MaxEncodedLen;

		type OutputId: Parameter + Ord + MaxEncodedLen;

		type OnVoteConclusion: OnVoteConclusion<Self>;

		type VotingProvider: VoterPower<Self> + QuorumMet<Self>;

		#[pallet::constant]
		type MaxVoters: Get<u32>;

		// type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
		fn on_finalize() -> Weight;
		fn on_initialize() -> Weight;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	//#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		VoteInProgress,
		TooManyVoters,
		UnknownTask,
		UnknownVoter,
		DuplicateVoter,
		NoWinner,
	}

	#[pallet::storage]
	pub type Rounds<T: Config> = StorageMap<_, Identity, T::TaskId, RoundOf<T>>;
}
