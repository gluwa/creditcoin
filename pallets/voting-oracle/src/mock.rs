use sp_std::{cell::RefCell, collections::btree_map::BTreeMap};

use crate::{self as pallet_voting_oracle, AggregateData, OnProposalComplete};
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type BlockNumber = u64;
pub type AccountId = u64;
pub type Reason = ();

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		VotingOracle: pallet_voting_oracle::{Pallet, Call, Origin<T>, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<{ u32::MAX }>;
}

impl pallet_voting_oracle::Config for Test {
	type Origin = Origin;
	type Event = Event;
	type Proposal = Call;
	type MaxProposals = ConstU32<10>;
	type DisagreementReason = Reason;
	type TimeLimit = ConstU64<10>;
	type QuorumPercentage = ConstU32<50>;
	type OnProposalComplete = CountCalls;
	type ProposalExtraData = ();
	type ProposalWithoutData = crate::MakeProposalIdentity<Test, Call>;
	type DataAggregator = TakeFirst;
}

pub struct TakeFirst;
impl AggregateData<Test> for TakeFirst {
	fn aggregate_data(
		extra_data: &BTreeMap<<Test as pallet_voting_oracle::Config>::ProposalExtraData, u32>,
	) -> Result<<Test as pallet_voting_oracle::Config>::ProposalExtraData, ()> {
		Ok(*extra_data.iter().next().unwrap().0)
	}
}

pub struct CountCalls;

pub(crate) type BaseProposal = <Test as crate::Config>::ProposalWithoutData;

impl OnProposalComplete<H256, BaseProposal, Reason> for CountCalls {
	fn on_proposal_accepted(_proposal_hash: &H256, _proposal: &BaseProposal) {
		ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow_mut().accept += 1);
	}

	fn on_proposal_rejected(
		_proposal_hash: &H256,
		_proposal: &BaseProposal,
		_reasons: &sp_std::collections::btree_set::BTreeSet<Reason>,
	) {
		ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow_mut().reject += 1);
	}

	fn on_proposal_expired(_proposal_hash: &H256, _proposal: &BaseProposal) {
		ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow_mut().expire += 1);
	}
}

#[derive(Clone, Default)]
struct OnProposalCompleteCallCounts {
	accept: u32,
	reject: u32,
	expire: u32,
}
impl OnProposalCompleteCallCounts {
	fn reset(&mut self) {
		self.accept = 0;
		self.reject = 0;
		self.expire = 0;
	}
}

thread_local! {
	static ON_PROPOSAL_COMPLETE_COUNTS: RefCell<OnProposalCompleteCallCounts>  = RefCell::new(OnProposalCompleteCallCounts::default());
}

pub fn on_proposal_accepted_calls() -> u32 {
	ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow().accept)
}

pub fn on_proposal_rejected_calls() -> u32 {
	ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow().reject)
}

pub fn on_proposal_expired_calls() -> u32 {
	ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow().expire)
}

pub fn reset_call_counts() {
	ON_PROPOSAL_COMPLETE_COUNTS.with(|c| c.borrow_mut().reset());
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_tracing::try_init_simple();
	reset_call_counts();
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
