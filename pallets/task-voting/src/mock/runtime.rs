//pub use crate::AccountId;
//use crate::{
//opaque, AccountIdLookup, Address, AuthorityId, Balance, BlakeTwo256, BlockHashCount,
//BlockLength, BlockNumber, BlockWeights, ExistentialDeposit, Hash, Index, MaxLocks,
//MinimumPeriod, Moment, RocksDbWeight, SS58Prefix, Signature, Version,
//};
use frame_support as support;
use frame_support::pallet_prelude::*;
use frame_support::{construct_runtime, parameter_types};
use frame_system as system;
use sp_runtime::generic;
use sp_runtime::MultiAddress;
use sp_runtime::{
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};

pub(super) type BlockNumber = u64;
type Hash = sp_runtime::testing::H256;
pub type Signature = MultiSignature;
pub(crate) type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type Address = MultiAddress<AccountId, ()>;
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

pub mod opaque {
	use super::*;

	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

construct_runtime!(
	pub enum Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		Taskvoting: crate::pallet,
	}
);

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
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
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

impl crate::Config for Runtime {
	type ItemId = Self::Hash;
	type Item = RuntimeCall;
	type Who = Self::AccountId;
	type PowerUnit = u128;
	type PowerProvider = ();
	type MaxVoters = ConstU32<10>;
}
