#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;
pub use pallet::*;
use parity_scale_codec::MaxEncodedLen;
use voting::traits::{OnVoteConclusion, QuorumMet, VoterPower};
use voting::{Power as VotingPower, Votes};
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod voting;

pub struct UnknownVoterError;

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
	use frame_support::defensive;
	use frame_support::{dispatch::Weight, pallet_prelude::*, traits::Get, Identity, Parameter};
	use frame_system::pallet_prelude::*;
	use pallet_staking::era::EraInterface;
	use pallet_staking::EraIndex;
	use sp_arithmetic::traits::One;
	use sp_arithmetic::traits::Saturating;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type TaskId: Parameter + Ord + MaxEncodedLen;

		type OutputId: Parameter + Ord + MaxEncodedLen;

		type OnVoteConclusion: OnVoteConclusion<Self>;

		type Era: EraInterface<Self::BlockNumber>;

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

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		EraIndex: From<T::BlockNumber>,
	{
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let next_era = T::Era::next_era_start();
			if next_era == block_number {
				T::Era::start_era(block_number);
			} else if next_era < block_number {
				defensive!("Next era behind current block height");
				T::Era::start_era(block_number);
			}
			//TODO benchmark
			Weight::from_ref_time(1_000_000u64)
		}

		//TODO benchmark
		fn on_finalize(block_number: T::BlockNumber) {
			let next_era = T::Era::next_era_start();
			if next_era == block_number {
				T::Era::set_era_start_time()
			}

			if next_era == block_number.saturating_add(One::one()) {
				T::Era::end_active_era();
			}
		}
	}
}
