#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{Hash, SaturatedConversion};
use frame_support::traits::{ChangeMembers, InitializeMembers};
pub use pallet::*;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;

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

impl<AccountId, BlockNumber, Reason> Votes<AccountId, BlockNumber, Reason> {
	pub fn count(&self) -> usize {
		self.ayes.len() + self.nays.len()
	}
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

pub trait OnProposalComplete<Hash, Proposal, Reason> {
	fn on_proposal_accepted(proposal_hash: &Hash, proposal: &Proposal);

	fn on_proposal_rejected(proposal_hash: &Hash, proposal: &Proposal, reasons: &BTreeSet<Reason>);

	fn on_proposal_expired(proposal_hash: &Hash, proposal: &Proposal);
}

impl<H, P, R> OnProposalComplete<H, P, R> for () {
	fn on_proposal_accepted(_: &H, _: &P) {}

	fn on_proposal_rejected(_: &H, _: &P, _: &BTreeSet<R>) {}

	fn on_proposal_expired(_: &H, _: &P) {}
}

impl<T: Config> ChangeMembers<T::AccountId> for Pallet<T> {
	fn change_members_sorted(
		_incoming: &[T::AccountId],
		outgoing: &[T::AccountId],
		new: &[T::AccountId],
	) {
		let mut outgoing = outgoing.to_vec();
		outgoing.sort();
		for info in Self::proposals() {
			Voting::<T>::mutate(info.hash, |v| {
				if let Some(mut votes) = v.take() {
					votes.ayes = votes
						.ayes
						.into_iter()
						.filter(|i| outgoing.binary_search(i).is_err())
						.collect();
					votes.nays = votes
						.nays
						.into_iter()
						.filter(|i| outgoing.binary_search(&i.who).is_err())
						.collect();
					*v = Some(votes);
				}
			});
		}

		Members::<T>::put(new);
	}
}

impl<T: Config> InitializeMembers<T::AccountId> for Pallet<T> {
	fn initialize_members(members: &[T::AccountId]) {
		if !members.is_empty() {
			assert!(Members::<T>::get().is_empty(), "Members are already initialized!");

			Members::<T>::put(members);
		}
	}
}

pub struct EnsureProportionAtLeast<AccountId, const N: u32, const D: u32>(PhantomData<AccountId>);
impl<
		O: Into<Result<RawOrigin<AccountId>, O>> + From<RawOrigin<AccountId>>,
		AccountId,
		const N: u32,
		const D: u32,
	> EnsureOrigin<O> for EnsureProportionAtLeast<AccountId, N, D>
{
	type Success = ();
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			RawOrigin::Members(n, m) if n * D >= N * m => Ok(()),
			r => Err(O::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn successful_origin() -> O {
		O::from(RawOrigin::Members(0u32, 0u32))
	}
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

		type DisagreementReason: Parameter + Ord;

		type OnProposalComplete: OnProposalComplete<
			<Self as frame_system::Config>::Hash,
			Self::Proposal,
			Self::DisagreementReason,
		>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub members: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { members: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			use sp_std::collections::btree_set::BTreeSet;
			let members_set: BTreeSet<_> = self.members.iter().collect();
			assert_eq!(
				members_set.len(),
				self.members.len(),
				"Members cannot contain duplicate accounts."
			);

			Pallet::<T>::initialize_members(&self.members)
		}
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
	pub enum Event<T: Config> {
		Executed { proposal_hash: T::Hash, result: DispatchResult },
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		NotMember,
		NonexistentProposal,
		TooManyProposals,
		AlreadyVoted,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_num: BlockNumberFor<T>) -> Weight {
			let proposals = Proposals::<T>::get();
			for proposal in proposals.iter().filter(|p| p.end <= block_num) {
				if let Err(e) = Self::expire_proposal(&proposal.hash) {
					log::error!("Error expiring proposal: {:?}", e);
				}
			}
			// TODO: proper weight calculation
			0
		}
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

			Self::vote_on_proposal(who, proposal, Vote::Aye)?;

			Ok(())
		}

		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3))]
		#[transactional]
		pub fn reject(
			origin: OriginFor<T>,
			proposal: ProposalOrHash<T::Proposal, T::Hash>,
			reason: T::DisagreementReason,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::vote_on_proposal(who, proposal, Vote::Nay(reason))?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn open_proposal(
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

		pub(crate) fn add_vote(
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

		pub(crate) fn meets_quorum(voted: MemberCount, member_count: MemberCount) -> bool {
			let cutoff = member_count * T::QuorumPercentage::get() / 100;
			voted >= cutoff
		}

		pub(crate) fn close_proposal(
			proposal_hash: &T::Hash,
			proposal: Box<T::Proposal>,
			votes: &Votes<T::AccountId, T::BlockNumber, T::DisagreementReason>,
		) -> Result<(), Error<T>> {
			let ayes_count = votes.ayes.len() as MemberCount;
			let nays_count = votes.nays.len() as MemberCount;
			let vote_count = votes.count() as MemberCount;

			if ayes_count >= nays_count {
				// enact proposal
				let _proposal_weight =
					Self::do_accept_proposal(vote_count, ayes_count, proposal_hash, proposal);
			} else {
				// veto proposal
				Self::do_reject_proposal(proposal_hash, &proposal, votes);
			}

			Ok(())
		}

		#[must_use]
		pub(crate) fn do_accept_proposal(
			members: MemberCount,
			approvals: MemberCount,
			proposal_hash: &T::Hash,
			proposal: Box<T::Proposal>,
		) -> Weight {
			T::OnProposalComplete::on_proposal_accepted(proposal_hash, &proposal);
			let dispatch_weight = proposal.get_dispatch_info().weight;

			let origin = RawOrigin::Members(approvals, members).into();
			let result = proposal.dispatch(origin);
			Self::deposit_event(Event::<T>::Executed {
				proposal_hash: proposal_hash.clone(),
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			let proposal_weight = get_result_weight(result).unwrap_or(dispatch_weight);

			Self::remove_proposal(proposal_hash);

			proposal_weight
		}

		pub(crate) fn do_reject_proposal(
			proposal_hash: &T::Hash,
			proposal: &T::Proposal,
			votes: &Votes<T::AccountId, T::BlockNumber, T::DisagreementReason>,
		) {
			let reasons =
				votes.nays.iter().map(|vote| vote.reason.clone()).collect::<BTreeSet<_>>();

			T::OnProposalComplete::on_proposal_rejected(proposal_hash, &proposal, &reasons);

			Self::remove_proposal(proposal_hash);
		}

		pub(crate) fn remove_proposal(proposal_hash: &T::Hash) {
			ProposalOf::<T>::remove(proposal_hash);
			Voting::<T>::remove(proposal_hash);
			Proposals::<T>::mutate(|proposals| {
				proposals.retain(|proposal| proposal.hash != *proposal_hash);
			});
		}

		pub(crate) fn expire_proposal(proposal_hash: &T::Hash) -> Result<(), Error<T>> {
			let proposal = ProposalOf::<T>::get(proposal_hash)
				.ok_or_else(|| Error::<T>::NonexistentProposal)?;
			T::OnProposalComplete::on_proposal_expired(proposal_hash, &proposal);
			Self::remove_proposal(proposal_hash);
			Ok(())
		}

		pub(crate) fn vote_on_proposal(
			who: T::AccountId,
			proposal: ProposalOrHash<T::Proposal, T::Hash>,
			vote: Vote<T::DisagreementReason>,
		) -> Result<(), Error<T>> {
			let members = Self::members();
			ensure!(members.contains(&who), Error::<T>::NotMember);

			let proposal_hash = proposal.hash::<T>();
			let proposal_open = ProposalOf::<T>::get(&proposal_hash);
			let (vote_count, proposal) = if let Some(proposal) = proposal_open {
				let count = Self::add_vote(&proposal_hash, who.clone(), vote)?;
				(count, Box::new(proposal))
			} else {
				match proposal {
					ProposalOrHash::Proposal(proposal) => {
						Self::open_proposal(proposal.clone(), proposal_hash, who, vote)?;
						(1, proposal)
					},
					ProposalOrHash::Hash(_) => {
						return Err(Error::<T>::NonexistentProposal.into());
					},
				}
			};

			// close if threshold met

			if Self::meets_quorum(vote_count, members.len().saturated_into()) {
				let votes = Voting::<T>::get(&proposal_hash)
					.ok_or_else(|| Error::<T>::NonexistentProposal)?;
				Self::close_proposal(&proposal_hash, proposal, &votes)?;
			}

			Ok(())
		}
	}
}

/// Return the weight of a dispatch call result as an `Option`.
///
/// Will return the weight regardless of what the state of the result is.
fn get_result_weight(result: DispatchResultWithPostInfo) -> Option<Weight> {
	match result {
		Ok(post_info) => post_info.actual_weight,
		Err(err) => err.post_info.actual_weight,
	}
}
