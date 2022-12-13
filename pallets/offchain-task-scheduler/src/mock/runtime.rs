use frame_support::{
	self as support, parameter_types,
	traits::{ConstU32, ConstU64},
};
use frame_system::{
	self as system,
	mocking::{MockBlock, MockUncheckedExtrinsic},
};
use sp_runtime::{
	traits::{Extrinsic as ExtrinsicT, IdentifyAccount, Verify},
	MultiSignature,
};

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

type Block = MockBlock<Runtime>;
type UncheckedExtrinsic = MockUncheckedExtrinsic<Runtime>;
pub(super) type BlockNumber = u64;
type Hash = sp_core::H256;
pub(crate) type Balance = u128;
pub type Signature = MultiSignature;
pub(crate) type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub(crate) type Extrinsic = TestXt<RuntimeCall, ()>;

impl system::Config for Runtime {
	type BaseCallFilter = support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::testing::Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<{ u32::MAX }>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

pub type Moment = u64;

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

use system::offchain::{CreateSignedTransaction, SendTransactionTypes, SigningTypes};

impl SigningTypes for Runtime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

use sp_runtime::testing::TestXt;

impl<LocalCall> SendTransactionTypes<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(RuntimeCall, <Self::Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl crate::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UnverifiedTaskTimeout = ConstU64<5>;
	type AuthorityId = crate::crypto::AuthorityId;
	type TaskCall = RuntimeCall;
	type WeightInfo = crate::weights::WeightInfo<Self>;
	type Task = super::task::MockTask<u32>;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Runtime where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		TaskScheduler: crate::{Pallet, Storage, Event<T>},
	}
);
