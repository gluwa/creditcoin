use crate::{mock::*, Disagreement, Vote, Votes};
use codec::Encode;
use frame_support::sp_runtime::traits::Hash;
use frame_support::traits::Get;
use frame_support::traits::Hooks;
use maplit::btreemap;
use sp_core::H256;

type Proposal = Box<Call>;
type ProposalHash = H256;
type TestVotes = Votes<AccountId, BlockNumber, (), ()>;
type TestVote = Vote<(), ()>;
const AYE: TestVote = TestVote::Aye(());
const NAY: TestVote = TestVote::Nay(());

fn make_proposal(value: u64) -> Box<BaseProposal> {
	Box::new(
		Call::System(frame_system::Call::remark_with_event {
			remark: value.to_be_bytes().to_vec(),
		})
		.into(),
	)
}

fn hash_of<S: Encode>(s: &S) -> H256 {
	<Test as frame_system::Config>::Hashing::hash_of(s)
}

impl TestVotes {
	fn with_ayes_nays(num_ayes: u64, num_nays: u64) -> Self {
		let time_limit: BlockNumber = <Test as crate::Config>::TimeLimit::get();
		Self {
			ayes: (0..num_ayes).collect(),
			nays: (0..num_nays)
				.map(|acct| Disagreement { who: acct + num_ayes, reason: () })
				.collect(),
			end: System::block_number() + time_limit,
			extra_data: btreemap! {},
		}
	}
}

struct TestProposal {
	hash: H256,
	proposal: Box<BaseProposal>,
}

impl TestProposal {
	fn new(open: bool) -> Self {
		let proposal = make_proposal(1);
		let hash = hash_of(&proposal);
		if open {
			VotingOracle::open_proposal(proposal.clone(), hash, 0, AYE).unwrap();
		}
		Self { proposal, hash }
	}

	fn with(value: u64, open: bool) -> Self {
		let proposal = make_proposal(value);
		let hash = hash_of(&proposal);
		if open {
			VotingOracle::open_proposal(proposal.clone(), hash, 0, AYE).unwrap();
		}
		Self { proposal, hash }
	}
}

type TestError = crate::Error<Test>;
type Proposals = crate::Proposals<Test>;

mod extrinsics {}

mod helpers {
	use core::convert::TryInto;

	use frame_support::{assert_noop, assert_ok, dispatch::GetDispatchInfo};
	use sp_runtime::DispatchError;
	use sp_std::collections::btree_map::BTreeMap;

	use crate::{Config, Disagreement, MakeProposal, ProposalInfo, Vote, Votes};

	use super::*;

	#[test]
	fn open_proposal_works() {
		new_test_ext().execute_with(|| {
			let assert_votes = |prop: Box<BaseProposal>, hash, initial_vote: TestVote, votes| {
				assert_eq!(
					VotingOracle::open_proposal(prop.clone(), hash, 0, initial_vote),
					Ok(votes)
				);
			};
			let assert_proposal_info = |hash, end| {
				assert!(
					crate::Proposals::<Test>::get().contains(&crate::ProposalInfo { hash, end })
				);
			};
			let assert_proposal_value = |hash: H256, proposal: Box<BaseProposal>| {
				assert_eq!(crate::ProposalOf::<Test>::get(hash).unwrap(), *proposal);
			};

			let time_limit: u64 = <Test as crate::Config>::TimeLimit::get();
			let end = System::block_number() + time_limit;

			let proposal = TestProposal::new(false);
			assert_votes(
				proposal.proposal.clone(),
				proposal.hash,
				AYE,
				Votes { ayes: vec![0], nays: vec![], end, extra_data: btreemap! {} },
			);
			assert_proposal_info(proposal.hash, end);
			assert_proposal_value(proposal.hash, proposal.proposal);

			let second_proposal = TestProposal::with(2, false);
			assert_votes(
				second_proposal.proposal.clone(),
				second_proposal.hash,
				NAY,
				Votes {
					ayes: vec![],
					nays: vec![Disagreement { reason: (), who: 0 }],
					end,
					extra_data: btreemap! {},
				},
			);
			assert_proposal_info(second_proposal.hash, end);
			assert_proposal_value(second_proposal.hash, second_proposal.proposal);
		});
	}

	#[test]
	fn open_proposal_full() {
		new_test_ext().execute_with(|| {
			let mut proposals = vec![];
			let cap = <Test as Config>::MaxProposals::get();
			for i in 0..cap {
				let proposal = make_proposal(i as u64);
				proposals.push(ProposalInfo { hash: hash_of(&proposal), end: 100 });
			}
			Proposals::set(proposals.try_into().unwrap());

			let proposal = TestProposal::with(cap as u64 + 1, false);

			assert_noop!(
				VotingOracle::open_proposal(proposal.proposal, proposal.hash, 1, AYE),
				TestError::TooManyProposals
			);
		});
	}

	#[test]
	fn add_vote_works() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);

			let vote = AYE;
			let voter = 2;

			assert_ok!(VotingOracle::add_vote(&proposal.hash, voter, vote));

			let votes = VotingOracle::voting(&proposal.hash).unwrap();
			assert_eq!(votes.ayes.len(), 2);
			assert_eq!(*votes.ayes.last().unwrap(), voter);
		});
	}

	#[test]
	fn add_vote_nay() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);

			let voter = 1;
			let vote = Vote::Nay(());

			assert_eq!(VotingOracle::add_vote(&proposal.hash, voter, vote), Ok(2));

			let votes = VotingOracle::voting(&proposal.hash).unwrap();
			assert_eq!(votes.nays.len(), 1);
			assert_eq!(*votes.nays.last().unwrap(), Disagreement { who: voter, reason: () });
		});
	}

	#[test]
	fn add_vote_already_voted() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);

			let voter = 1;

			assert_eq!(VotingOracle::add_vote(&proposal.hash, voter, AYE), Ok(2));

			assert_noop!(
				VotingOracle::add_vote(&proposal.hash, voter, AYE),
				TestError::AlreadyVoted
			);

			assert_noop!(
				VotingOracle::add_vote(&proposal.hash, voter, Vote::Nay(())),
				TestError::AlreadyVoted
			);
		});
	}

	#[test]
	fn meets_quorum_works() {
		let quorum_percent: u32 = <Test as Config>::QuorumPercentage::get();
		let member_count = 100;
		let voted = quorum_percent * member_count / 100;

		assert!(VotingOracle::meets_quorum(voted, member_count));
		assert!(!VotingOracle::meets_quorum(voted - 1, member_count));
	}

	#[test]
	fn meets_quorum_no_members() {
		assert!(VotingOracle::meets_quorum(0, 0));
	}

	#[test]
	fn do_accept_proposal_works() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);
			VotingOracle::add_vote(&proposal.hash, 1, AYE).unwrap();

			let prop = proposal.proposal.clone().make_proposal(()).unwrap();
			let prop_weight = prop.get_dispatch_info().weight;

			assert_eq!(
				VotingOracle::do_accept_proposal(
					2,
					2,
					&proposal.hash,
					proposal.proposal.clone(),
					&BTreeMap::new()
				),
				Ok(prop_weight)
			);

			assert_eq!(on_proposal_accepted_calls(), 1);

			assert_eq!(
				System::events().pop().unwrap().event,
				Event::VotingOracle(crate::Event::<Test>::Executed {
					proposal_hash: proposal.hash,
					result: Err(DispatchError::BadOrigin),
				})
			);

			assert!(crate::ProposalOf::<Test>::get(&proposal.hash).is_none());
			assert!(crate::Voting::<Test>::get(&proposal.hash).is_none());
			assert!(Proposals::get().into_iter().find(|p| p.hash == proposal.hash).is_none());
		});
	}

	#[test]
	fn remove_proposal_works() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);

			assert!(crate::ProposalOf::<Test>::get(&proposal.hash).is_some());
			assert!(crate::Voting::<Test>::get(&proposal.hash).is_some());
			assert!(Proposals::get().into_iter().find(|p| p.hash == proposal.hash).is_some());

			VotingOracle::remove_proposal(&proposal.hash);

			assert!(crate::ProposalOf::<Test>::get(&proposal.hash).is_none());
			assert!(crate::Voting::<Test>::get(&proposal.hash).is_none());
			assert!(Proposals::get().into_iter().find(|p| p.hash == proposal.hash).is_none());
		});
	}

	#[test]
	fn on_initialize_removes_expired_proposals() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);
			let hash = proposal.hash;
			let time_limit: u64 = <Test as crate::Config>::TimeLimit::get();

			assert!(crate::ProposalOf::<Test>::get(&hash).is_some());
			assert!(crate::Voting::<Test>::get(&hash).is_some());
			assert!(Proposals::get().into_iter().find(|p| p.hash == hash).is_some());

			System::set_block_number(System::block_number() + time_limit);

			VotingOracle::on_initialize(System::block_number());

			assert!(crate::ProposalOf::<Test>::get(&hash).is_none());
			assert!(crate::Voting::<Test>::get(&hash).is_none());
			assert!(Proposals::get().into_iter().find(|p| p.hash == hash).is_none());
		});
	}
}
