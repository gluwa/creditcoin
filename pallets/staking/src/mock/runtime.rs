use crate::Config;
use frame_support::traits::ConstU32;
use frame_support::{construct_runtime, parameter_types};
use frame_system::EnsureRoot;
use sp_runtime::traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify};
use sp_runtime::{generic, MultiSignature, Perbill};

mod opaque {
	use super::*;
	use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub(super) type Block = generic::Block<Header, UncheckedExtrinsic>;
}

pub type Signature = MultiSignature;
pub type Signer = <Signature as Verify>::Signer;
pub type AccountId = <Signer as IdentifyAccount>::AccountId;
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
);
pub type Hash = sp_runtime::testing::H256;
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

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
		Staking: crate,
	}
);

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
parameter_types! {
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;

}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<{ u32::MAX }>;
	type BlockHashCount = ();
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 1;
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
pub const BlocksPerEra: u32 = 60;
pub const BondingDuration: u32 = 12;
pub const SlashDeferDuration: u32 = 10;
pub const MaxUnlockingChunks: u32 = 100;
}

impl Config for Runtime {
	type BondingDuration = BondingDuration;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type HistoryDepth = HistoryDepth;
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type OnStakerSlash = ();
	type Reward = ();
	type RuntimeEvent = RuntimeEvent;
	type Slash = ();
	type SlashDeferDuration = SlashDeferDuration;
	type SlashCancelOrigin = EnsureRoot<AccountId>;
	type UnixTime = pallet_timestamp::Pallet<Runtime>;
	type WeightInfo = ();
}
