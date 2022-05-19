#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{Hash, SaturatedConversion};
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;

#[allow(clippy::unnecessary_cast)]
pub mod weights;

pub type ProposalIndex = u32;

pub type MemberCount = u32;

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum RawOrigin<AccountId> {
	Members(MemberCount, MemberCount),
	Member(AccountId),
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Votes<AccountId, BlockNumber, Reason> {
	ayes: Vec<AccountId>,
	nays: Vec<Disagreement<AccountId, Reason>>,
	end: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Disagreement<AccountId, Reason> {
	who: AccountId,
	reason: Reason,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum ProposalOrHash<Proposal, Hash> {
	Proposal(Box<Proposal>),
	Hash(Hash),
}

impl<Proposal, Hash> ProposalOrHash<Proposal, Hash>
where
	Proposal: Encode,
	Hash: Clone,
{
	pub fn hash<T: Config>(&self) -> Hash
	where
		<T as frame_system::Config>::Hashing: sp_runtime::traits::Hash<Output = Hash>,
	{
		match self {
			ProposalOrHash::Proposal(proposal) => T::Hashing::hash_of(proposal),
			ProposalOrHash::Hash(hash) => hash.clone(),
		}
	}
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct ProposalInfo<Hash, BlockNumber> {
	hash: Hash,
	end: BlockNumber,
}

#[derive(Clone)]
enum Vote<Reason> {
	Aye,
	Nay(Reason),
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;

	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Origin: From<RawOrigin<Self::AccountId>>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Proposal: Parameter
			+ Dispatchable<Origin = <Self as Config>::Origin, PostInfo = PostDispatchInfo>
			+ From<frame_system::Call<Self>>
			+ GetDispatchInfo;

		type MaxProposals: Get<ProposalIndex>;

		type TimeLimit: Get<Self::BlockNumber>;

		type QuorumPercentage: Get<MemberCount>;

		type DisagreementReason: Parameter;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::origin]
	pub type Origin<T> = RawOrigin<<T as frame_system::Config>::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn members)]
	#[pallet::unbounded]
	pub type Members<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageValue<
		_,
		BoundedVec<ProposalInfo<T::Hash, T::BlockNumber>, T::MaxProposals>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_of)]
	#[pallet::unbounded]
	pub type ProposalOf<T: Config> =
		StorageMap<_, Identity, T::Hash, <T as Config>::Proposal, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn voting)]
	#[pallet::unbounded]
	pub type Voting<T: Config> = StorageMap<
		_,
		Identity,
		T::Hash,
		Votes<T::AccountId, T::BlockNumber, T::DisagreementReason>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		NotMember,
		NonexistentProposal,
		TooManyProposals,
		AlreadyVoted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3))]
		#[transactional]
		pub fn accept(
			origin: OriginFor<T>,
			proposal: ProposalOrHash<T::Proposal, T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let members = Self::members();
			ensure!(members.contains(&who), Error::<T>::NotMember);

			let proposal_hash = proposal.hash::<T>();
			let proposal_open = ProposalOf::<T>::contains_key(&proposal_hash);
			let vote_count = if proposal_open {
				Self::add_vote(&proposal_hash, who.clone(), Vote::Aye)?
			} else {
				match proposal {
					ProposalOrHash::Proposal(proposal) => {
						Self::open_proposal(proposal, proposal_hash, who, Vote::Aye)?;
						1
					},
					ProposalOrHash::Hash(_) => {
						return Err(Error::<T>::NonexistentProposal.into());
					},
				}
			};

			// close if threshold met

			if Self::meets_quorum(vote_count, members.len().saturated_into()) {
				// Self::close_proposal(&proposal_hash)?;
			}

			Ok(())
		}

		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3))]
		#[transactional]
		#[allow(unused_variables)]
		pub fn reject(
			origin: OriginFor<T>,
			proposal: ProposalOrHash<T::Proposal, T::Hash>,
			reason: T::DisagreementReason,
		) -> DispatchResult {
			todo!();

			#[allow(unreachable_code)]
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn open_proposal(
			proposal: Box<T::Proposal>,
			proposal_hash: T::Hash,
			who: T::AccountId,
			initial_vote: Vote<T::DisagreementReason>,
		) -> Result<Votes<T::AccountId, T::BlockNumber, T::DisagreementReason>, Error<T>> {
			let end = frame_system::Pallet::<T>::block_number() + T::TimeLimit::get();
			let proposal_info = ProposalInfo { hash: proposal_hash, end };
			Proposals::<T>::try_mutate(|proposals| -> Result<usize, Error<T>> {
				proposals.try_push(proposal_info).map_err(|_| Error::<T>::TooManyProposals)?;
				Ok(proposals.len())
			})?;
			ProposalOf::<T>::insert(&proposal_hash, proposal);
			let votes: Votes<_, _, T::DisagreementReason> = match initial_vote {
				Vote::Aye => Votes { end, ayes: vec![who.clone()], nays: vec![] },
				Vote::Nay(reason) => {
					Votes { end, ayes: vec![], nays: vec![Disagreement { who, reason }] }
				},
			};
			Voting::<T>::insert(&proposal_hash, &votes);
			Ok(votes)
		}

		fn add_vote(
			proposal_hash: &T::Hash,
			who: T::AccountId,
			vote: Vote<T::DisagreementReason>,
		) -> Result<MemberCount, Error<T>> {
			let mut voting = Self::voting(&proposal_hash).ok_or(Error::<T>::NonexistentProposal)?;

			ensure!(!voting.ayes.contains(&who), Error::<T>::AlreadyVoted);
			ensure!(
				voting.nays.iter().find(|vote| vote.who == who).is_none(),
				Error::<T>::AlreadyVoted
			);

			match vote {
				Vote::Aye => {
					voting.ayes.push(who);
				},
				Vote::Nay(reason) => {
					voting.nays.push(Disagreement { who, reason });
				},
			}

			Voting::<T>::insert(&proposal_hash, &voting);

			let vote_count = voting.ayes.len().saturating_add(voting.nays.len());
			Ok(vote_count.saturated_into())
		}

		fn meets_quorum(voted: MemberCount, member_count: MemberCount) -> bool {
			let cutoff = member_count * T::QuorumPercentage::get() / 100;
			voted >= cutoff
		}

		#[allow(unused_variables)]
		fn close_proposal(
			proposal_hash: &T::Hash,
			votes: Votes<T::AccountId, T::BlockNumber, T::DisagreementReason>,
		) -> Result<(), Error<T>> {
			todo!()
		}
	}
}
