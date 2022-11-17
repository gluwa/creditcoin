use crate as pallet_rewards;
use frame_support::{
	parameter_types,
	traits::{ConstU32, Hooks},
};
use frame_system as system;
use parity_scale_codec::Encode;
use sp_core::H256;
use sp_runtime::{
	testing::{Digest, DigestItem, Header},
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type BlockNumber = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Rewards: pallet_rewards::{Pallet, Storage, Event<T>},
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
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
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

type Balance = u128;

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

impl pallet_rewards::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type WeightInfo = super::weights::WeightInfo<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	tracing::try_init_simple();
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	// accounts 1 to 5 have initial balances
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 10_000_000_000_000_000_000),
			(2, 20_000_000_000_000_000_000),
			(3, 30_000_000_000_000_000_000),
			(4, 40_000_000_000_000_000_000),
			(5, 50_000_000_000_000_000_000),
			(6, 60_000_000_000_000_000_000),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn roll_to(n: BlockNumber, author_account_id: u64) {
	let mut i = System::block_number();
	while i < n {
		Rewards::on_finalize(i);
		Balances::on_finalize(i);

		i += 1;
		let parent = System::parent_hash();
		let digest_item =
			DigestItem::PreRuntime(sp_consensus_pow::POW_ENGINE_ID, author_account_id.encode());
		System::initialize(&i, &parent, &Digest { logs: vec![digest_item.clone()] });
		System::set_block_number(i);
		Rewards::on_initialize(i);
	}
}
