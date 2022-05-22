use crate::{mock::*, Vote};
use codec::Encode;
use frame_support::sp_runtime::traits::Hash;
use frame_support::traits::Get;
use sp_core::H256;

type Proposal = Box<Call>;
type ProposalHash = H256;

fn make_proposal(value: u64) -> Box<Call> {
	Box::new(Call::System(frame_system::Call::remark_with_event {
		remark: value.to_be_bytes().to_vec(),
	}))
}

fn hash_of<S: Encode>(s: &S) -> H256 {
	<Test as frame_system::Config>::Hashing::hash_of(s)
}

struct TestProposal {
	hash: H256,
	proposal: Proposal,
}

impl TestProposal {
	fn new(open: bool) -> Self {
		let proposal = make_proposal(1);
		let hash = hash_of(&proposal);
		if open {
			VotingOracle::open_proposal(proposal.clone(), hash, 0, Vote::Aye).unwrap();
		}
		Self { proposal, hash }
	}

	fn with(value: u64, open: bool) -> Self {
		let proposal = make_proposal(value);
		let hash = hash_of(&proposal);
		if open {
			VotingOracle::open_proposal(proposal.clone(), hash, 0, Vote::Aye).unwrap();
		}
		Self { proposal, hash }
	}
}

type TestError = crate::Error<Test>;
type Proposals = crate::Proposals<Test>;

mod extrinsics {}

mod helpers {
	use core::convert::TryInto;

	use frame_support::{assert_noop, assert_ok};

	use crate::{Config, Disagreement, ProposalInfo, Vote, Votes};

	use super::*;

	#[test]
	fn open_proposal_works() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(false);
			let time_limit: u64 = <Test as crate::Config>::TimeLimit::get();
			let end = System::block_number() + time_limit;
			assert_eq!(
				VotingOracle::open_proposal(proposal.proposal.clone(), proposal.hash, 0, Vote::Aye),
				Ok(Votes { ayes: vec![0], nays: vec![], end })
			);

			assert!(crate::Proposals::<Test>::get()
				.contains(&crate::ProposalInfo { hash: proposal.hash, end }));

			assert_eq!(crate::ProposalOf::<Test>::get(proposal.hash).unwrap(), *proposal.proposal);

			assert_eq!(
				crate::Voting::<Test>::get(proposal.hash).unwrap(),
				Votes { ayes: vec![0], nays: vec![], end }
			);
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
				VotingOracle::open_proposal(proposal.proposal, proposal.hash, 1, Vote::Aye),
				TestError::TooManyProposals
			);
		});
	}

	#[test]
	fn add_vote_works() {
		new_test_ext().execute_with(|| {
			let proposal = TestProposal::new(true);

			let vote = Vote::Aye;
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

			assert_eq!(VotingOracle::add_vote(&proposal.hash, voter, Vote::Aye), Ok(2));

			assert_noop!(
				VotingOracle::add_vote(&proposal.hash, voter, Vote::Aye),
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
}
