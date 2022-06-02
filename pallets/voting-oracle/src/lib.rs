#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code))]

use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::{Hash, SaturatedConversion};
use frame_support::traits::{ChangeMembers, InitializeMembers};
pub use pallet::*;
use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};
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
pub struct Votes<AccountId, BlockNumber, ProposalExtraData, Reason> {
	ayes: Vec<AccountId>,
	nays: Vec<Disagreement<AccountId, Reason>>,
	extra_data: BTreeMap<ProposalExtraData, u32>,
	end: BlockNumber,
}

impl<AccountId, BlockNumber, ProposalExtraData, Reason>
	Votes<AccountId, BlockNumber, ProposalExtraData, Reason>
{
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
pub enum ProposalOrHash<ProposalWithoutData, Hash> {
	Proposal(Box<ProposalWithoutData>),
	Hash(Hash),
}

impl<ProposalWithoutData, Hash> ProposalOrHash<ProposalWithoutData, Hash>
where
	ProposalWithoutData: Encode,
	Hash: Clone,
{
	pub fn hash<T: Config>(&self) -> Hash
	where
		<T as frame_system::Config>::Hashing: sp_runtime::traits::Hash<Output = Hash>,
	{
		match self {
			ProposalOrHash::Proposal(base_proposal) => T::Hashing::hash_of(base_proposal),
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
enum Vote<ProposalExtraData, Reason> {
	Aye(ProposalExtraData),
	Nay(Reason),
}

type VoteOf<T> = Vote<<T as Config>::ProposalExtraData, <T as Config>::DisagreementReason>;

impl<D, R> sp_std::fmt::Debug for Vote<D, R>
where
	D: sp_std::fmt::Debug,
	R: sp_std::fmt::Debug,
{
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		match self {
			Vote::Aye(extra_data) => write!(f, "Vote::Aye({:?})", extra_data),
			Vote::Nay(reason) => write!(f, "Vote::Nay({:?})", reason),
		}
	}
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
		for ProposalInfo { hash, .. } in Self::proposals() {
			Voting::<T>::mutate(hash, |v| {
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

pub trait MakeProposal<T: Config> {
	fn make_proposal(self, extra_data: T::ProposalExtraData) -> Result<T::Proposal, ()>;
}

#[derive(TypeInfo, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MakeProposalIdentity<T, P>(P, PhantomData<T>);

impl<T, P> From<P> for MakeProposalIdentity<T, P> {
	fn from(p: P) -> Self {
		MakeProposalIdentity(p, PhantomData)
	}
}

impl<T, P> MakeProposal<T> for MakeProposalIdentity<T, P>
where
	T: Config<ProposalExtraData = (), Proposal = P>,
{
	fn make_proposal(self, _extra_data: ()) -> Result<<T as Config>::Proposal, ()> {
		Ok(self.0)
	}
}

pub trait AggregateData<T: Config> {
	fn aggregate_data(
		extra_data: &BTreeMap<T::ProposalExtraData, u32>,
	) -> Result<T::ProposalExtraData, ()>;
}

impl<T: Config> AggregateData<T> for ()
where
	T::ProposalExtraData: Clone,
{
	fn aggregate_data(
		extra_data: &BTreeMap<T::ProposalExtraData, u32>,
	) -> Result<T::ProposalExtraData, ()> {
		extra_data.keys().next().ok_or(()).map(|k| k.clone())
	}
}

#[allow(type_alias_bounds)]
pub type ProposalOrHashOf<T: Config> = ProposalOrHash<T::ProposalWithoutData, T::Hash>;

#[allow(type_alias_bounds)]
pub type VotesOf<T: Config> =
	Votes<T::AccountId, T::BlockNumber, T::ProposalExtraData, T::DisagreementReason>;

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

		type ProposalWithoutData: Parameter + MakeProposal<Self>;

		type ProposalExtraData: Parameter + Ord;

		type DataAggregator: AggregateData<Self>;

		#[pallet::constant]
		type MaxProposals: Get<ProposalIndex>;

		#[pallet::constant]
		type TimeLimit: Get<Self::BlockNumber>;

		#[pallet::constant]
		type QuorumPercentage: Get<MemberCount>;

		type DisagreementReason: Parameter + Ord;

		type OnProposalComplete: OnProposalComplete<
			<Self as frame_system::Config>::Hash,
			Self::ProposalWithoutData,
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
		StorageMap<_, Identity, T::Hash, <T as Config>::ProposalWithoutData, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn voting)]
	#[pallet::unbounded]
	pub type Voting<T: Config> = StorageMap<_, Identity, T::Hash, VotesOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Executed { proposal_hash: T::Hash, result: DispatchResult },
		Opened { proposal_hash: T::Hash },
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		NotMember,
		NonexistentProposal,
		TooManyProposals,
		AlreadyVoted,
		AggregationFailed,
		MakeProposalFailed,
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
			proposal: ProposalOrHashOf<T>,
			extra_data: T::ProposalExtraData,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::vote_on_proposal(who, proposal, Vote::Aye(extra_data))?;

			Ok(())
		}

		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3))]
		#[transactional]
		pub fn reject(
			origin: OriginFor<T>,
			proposal: ProposalOrHashOf<T>,
			reason: T::DisagreementReason,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::vote_on_proposal(who, proposal, Vote::Nay(reason))?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn open_proposal(
			proposal: Box<T::ProposalWithoutData>,
			proposal_hash: T::Hash,
			who: T::AccountId,
			initial_vote: VoteOf<T>,
		) -> Result<VotesOf<T>, Error<T>> {
			let end = frame_system::Pallet::<T>::block_number() + T::TimeLimit::get();
			let proposal_info = ProposalInfo { hash: proposal_hash, end };
			Proposals::<T>::try_mutate(|proposals| -> Result<usize, Error<T>> {
				proposals.try_push(proposal_info).map_err(|_| Error::<T>::TooManyProposals)?;
				Ok(proposals.len())
			})?;
			ProposalOf::<T>::insert(&proposal_hash, proposal);
			let votes: VotesOf<T> = match initial_vote {
				Vote::Aye(data) => {
					let mut extra_data = BTreeMap::new();
					extra_data.insert(data, 1);
					Votes { end, ayes: vec![who.clone()], nays: vec![], extra_data }
				},
				Vote::Nay(reason) => Votes {
					end,
					ayes: vec![],
					nays: vec![Disagreement { who, reason }],
					extra_data: BTreeMap::new(),
				},
			};
			Voting::<T>::insert(&proposal_hash, &votes);
			Self::deposit_event(Event::<T>::Opened { proposal_hash });
			Ok(votes)
		}

		pub(crate) fn add_vote(
			proposal_hash: &T::Hash,
			who: T::AccountId,
			vote: VoteOf<T>,
		) -> Result<MemberCount, Error<T>> {
			let mut voting = Self::voting(&proposal_hash).ok_or(Error::<T>::NonexistentProposal)?;

			ensure!(!voting.ayes.contains(&who), Error::<T>::AlreadyVoted);
			ensure!(
				voting.nays.iter().find(|vote| vote.who == who).is_none(),
				Error::<T>::AlreadyVoted
			);

			match vote {
				Vote::Aye(data) => {
					voting.ayes.push(who);
					*voting.extra_data.entry(data).or_insert(0) += 1;
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
			log::info!(
				"meets_quorum: voted = {voted}, member_count = {member_count}, cutoff = {cutoff}"
			);
			voted >= cutoff
		}

		pub(crate) fn close_proposal(
			proposal_hash: &T::Hash,
			proposal: Box<T::ProposalWithoutData>,
			votes: &VotesOf<T>,
		) -> Result<(), Error<T>> {
			log::info!("Closing proposal {:?}", proposal_hash);
			let ayes_count = votes.ayes.len() as MemberCount;
			let nays_count = votes.nays.len() as MemberCount;
			let vote_count = votes.count() as MemberCount;

			log::info!("Ayes: {ayes_count}, Nays: {nays_count}, Votes: {vote_count}");

			if ayes_count >= nays_count {
				// enact proposal
				let _proposal_weight = Self::do_accept_proposal(
					vote_count,
					ayes_count,
					proposal_hash,
					proposal,
					&votes.extra_data,
				)?;
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
			base_proposal: Box<T::ProposalWithoutData>,
			extra_data: &BTreeMap<T::ProposalExtraData, u32>,
		) -> Result<Weight, Error<T>> {
			T::OnProposalComplete::on_proposal_accepted(proposal_hash, &base_proposal);
			let aggregate = T::DataAggregator::aggregate_data(extra_data)
				.map_err(|_| Error::<T>::AggregationFailed)?;

			let proposal: T::Proposal = <T::ProposalWithoutData as MakeProposal<T>>::make_proposal(
				*base_proposal,
				aggregate,
			)
			.map_err(|_| Error::<T>::MakeProposalFailed)?;

			let dispatch_weight = proposal.get_dispatch_info().weight;

			let origin = RawOrigin::Members(approvals, members).into();
			let result = proposal.dispatch(origin);
			Self::deposit_event(Event::<T>::Executed {
				proposal_hash: proposal_hash.clone(),
				result: result.map(|_| ()).map_err(|e| e.error),
			});
			let proposal_weight = get_result_weight(result).unwrap_or(dispatch_weight);

			Self::remove_proposal(proposal_hash);

			Ok(proposal_weight)
		}

		pub(crate) fn do_reject_proposal(
			proposal_hash: &T::Hash,
			proposal: &T::ProposalWithoutData,
			votes: &VotesOf<T>,
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
			proposal: ProposalOrHashOf<T>,
			vote: VoteOf<T>,
		) -> Result<(), Error<T>> {
			log::info!("vote_on_proposal: {who:?} {:?} {vote:?}", proposal.hash::<T>());
			let members = Self::members();
			ensure!(members.contains(&who), Error::<T>::NotMember);

			let proposal_hash = proposal.hash::<T>();
			let proposal_open = ProposalOf::<T>::get(&proposal_hash);
			let (vote_count, proposal) = if let Some(proposal) = proposal_open {
				let count = Self::add_vote(&proposal_hash, who.clone(), vote)?;
				(count, Box::new(proposal))
			} else {
				match proposal {
					ProposalOrHash::Proposal(base_proposal) => {
						Self::open_proposal(base_proposal.clone(), proposal_hash, who, vote)?;
						(1, base_proposal)
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
