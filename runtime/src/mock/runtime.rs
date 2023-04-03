pub use crate::AccountId;
use crate::{
	opaque, AccountIdLookup, Address, Balance, BlakeTwo256, BlockHashCount, BlockLength,
	BlockNumber, BlockWeights, ExistentialDeposit, Hash, Index, MaxLocks, MinimumPeriod, Moment,
	RocksDbWeight, SS58Prefix, Signature, SignedExtra, Version,
};
use frame_support::pallet_prelude::*;
use frame_support::traits::U128CurrencyToVote;
use frame_support::{construct_runtime, parameter_types};
use frame_system::EnsureRoot;
use pallet_session::PeriodicSessions;
use pallet_staking::{DefaultElection, NoKeys};
use pallet_staking::{EmptyList, TrivialTargetList};
use pallet_staking::{TestBenchmarkingConfig, TrivialSessionHandler};
use sp_runtime::generic;
use sp_runtime::Perbill;
use sp_std::prelude::*;

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

construct_runtime!(
	pub enum Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Staking: pallet_staking::pallet,
		Session: pallet_session,
	}
);

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = AccountIdLookup<Self::AccountId, ()>;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<{ u32::MAX }>;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
pub const HistoryDepth: u32 = 84;
pub const MaxNominations: u32 = 1;
pub const BlocksPerEra: u32 = 60;
pub const BondingDuration: u32 = 12;
pub const SlashDeferDuration: u32 = 10;
pub const MaxUnlockingChunks: u32 = 100;
pub const MaxNominatorRewardedPerValidator: u32 = 0;
pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(0);
}

impl pallet_staking::Config for Runtime {
	type BondingDuration = BondingDuration;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type HistoryDepth = HistoryDepth;
	type EraPayout = ();
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type OnStakerSlash = ();
	type Reward = ();
	type RuntimeEvent = RuntimeEvent;
	type Slash = ();
	type SlashDeferDuration = SlashDeferDuration;
	type SlashCancelOrigin = EnsureRoot<AccountId>;
	type UnixTime = Timestamp;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
	type BenchmarkingConfig = TestBenchmarkingConfig;
	type MaxNominations = MaxNominations;
	type CurrencyToVote = U128CurrencyToVote;
	type RewardRemainder = ();
	type SessionsPerEra = BlocksPerEra;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type ElectionProvider = DefaultElection<(AccountId, Self::BlockNumber, Staking)>;
	type GenesisElectionProvider = DefaultElection<(AccountId, Self::BlockNumber, Staking)>;
	type VoterList = EmptyList<Self>;
	type TargetList = TrivialTargetList<Self>;
	type SessionInterface = ();
	type NextNewSession = Session;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = Self::AccountId;
	type ValidatorIdOf = ();
	type ShouldEndSession = PeriodicSessions<ConstU32<1>, ConstU32<0>>;
	type NextSessionRotation = ();
	type SessionManager = Staking;
	type SessionHandler = TrivialSessionHandler<Self>;
	type Keys = NoKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}
