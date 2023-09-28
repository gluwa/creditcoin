pub use crate::AccountId;
use crate::OnChainSeqPhragmen;
use crate::{
	opaque, AccountIdLookup, Address, AuthorityId, Balance, BlakeTwo256, BlockHashCount,
	BlockLength, BlockNumber, BlockWeights, ExistentialDeposit, Hash, Index, MaxLocks,
	MinimumPeriod, Moment, ParityDbWeight, SS58Prefix, Signature, Version, VoterList,
};
use frame_election_provider_support::onchain::OnChainExecution;
use frame_support::pallet_prelude::*;
use frame_support::traits::U128CurrencyToVote;
use frame_support::{construct_runtime, parameter_types};
use frame_system::EnsureRoot;
use pallet_session::PeriodicSessions;
use pallet_staking_substrate::TestBenchmarkingConfig;
use pallet_staking_substrate::UseValidatorsMap;
use sp_runtime::generic;
use sp_runtime::traits::OpaqueKeys;
use sp_runtime::traits::Verify;
use sp_runtime::MultiAddress;
use sp_runtime::Perbill;
use sp_runtime::SaturatedConversion;
use sp_std::prelude::*;

pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
pub type Signer = <Signature as Verify>::Signer;
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
);

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
		Staking: pallet_staking_substrate,
		Session: pallet_session,
		TaskScheduler: pallet_offchain_task_scheduler,
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
	type DbWeight = ParityDbWeight;
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

impl pallet_staking_substrate::Config for Runtime {
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
	type UnixTime = Timestamp;
	type WeightInfo = pallet_staking_substrate::weights::SubstrateWeight<Runtime>;
	type BenchmarkingConfig = TestBenchmarkingConfig;
	type MaxNominations = MaxNominations;
	type CurrencyToVote = U128CurrencyToVote;
	type RewardRemainder = ();
	type SessionsPerEra = BlocksPerEra;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type ElectionProvider = OnChainExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = VoterList;
	type TargetList = UseValidatorsMap<Self>;
	type SessionInterface = ();
	type NextNewSession = Session;
	type AdminOrigin = EnsureRoot<AccountId>;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = Self::AccountId;
	type ValidatorIdOf = ();
	type ShouldEndSession = PeriodicSessions<ConstU32<1>, ConstU32<0>>;
	type NextSessionRotation = ();
	type SessionManager = Staking;
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_offchain_task_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnverifiedTaskTimeout = ConstU32<60>;
	type AuthorityId = AuthorityId;
	type TaskCall = RuntimeCall;
	type WeightInfo = pallet_offchain_task_scheduler::weights::WeightInfo<Runtime>;
	type Task = pallet_offchain_task_scheduler::mocked_task::MockTask<u32>;
	type Authorship = TaskScheduler;
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = UncheckedExtrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: Self::Public,
		account: AccountId,
		nonce: Index,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		let period = BlockHashCount::get() as u64;
		let current_block = System::block_number().saturated_into::<u64>().saturating_sub(1);

		let extra: SignedExtra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
		);

		#[cfg_attr(not(feature = "std"), allow(unused_variables))]
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				frame_support::log::warn!("SignedPayload error: {:?}", e);
			})
			.ok()?;

		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;

		let address = MultiAddress::Id(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}
