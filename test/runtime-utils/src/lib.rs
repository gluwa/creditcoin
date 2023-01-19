pub mod pool;

extern crate alloc;
use frame_support::{
	sp_runtime::Storage,
	traits::{GenesisBuild, OffchainWorker, OnFinalize, OnInitialize},
};
use frame_system as system;
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
pub(crate) use parking_lot::RwLock;
use pool::{PoolState, TestTransactionPoolExt};
use sp_arithmetic::traits::One;
pub(crate) use sp_core::offchain::{
	testing::{OffchainState, TestOffchainExt},
	OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
};
use sp_core::Pair;
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt};
use sp_runtime::{traits::IdentifyAccount, AccountId32, MultiSigner};
use sp_state_machine::BasicExternalities;
use std::marker::PhantomData;
pub(crate) use std::sync::Arc;

#[derive(Default)]
pub struct ExtBuilder<G = ()> {
	pub keystore: Option<KeyStore>,
	pool: Option<TestTransactionPoolExt>,
	pub offchain: Option<TestOffchainExt>,
	pub genesis_config: G,
}

impl<G> ExtBuilder<G> {
	pub fn with_keystore(mut self) -> Self {
		self.keystore = Some(KeyStore::new());
		self
	}

	pub fn with_pool(&mut self) -> Arc<RwLock<PoolState>> {
		let (pool, state) = TestTransactionPoolExt::new();
		self.pool = Some(pool);
		state
	}

	pub fn with_offchain(&mut self) -> Arc<RwLock<OffchainState>> {
		let (offchain, state) = TestOffchainExt::new();
		self.offchain = Some(offchain);
		state
	}

	fn system_storage<Config: SystemConfig>() -> Storage {
		system::GenesisConfig::default().build_storage::<Config>().unwrap()
	}

	fn add_capabilities(self, ext: &mut TestExternalities) {
		if let Some(keystore) = self.keystore {
			ext.register_extension(KeystoreExt(Arc::new(keystore)));
		}
		if let Some(pool) = self.pool {
			ext.register_extension(TransactionPoolExt::new(pool));
		}
		if let Some(offchain) = self.offchain {
			ext.register_extension(OffchainDbExt::new(offchain.clone()));
			ext.register_extension(OffchainWorkerExt::new(offchain));
		}
	}

	fn assimilate_pallet_storage<Config, Pallet>(&self, mut storage: Storage) -> Storage
	where
		G: GenesisBuild<Config, Pallet>,
	{
		self.genesis_config.assimilate_storage(&mut storage).unwrap();
		storage
	}

	pub fn build<Config, Pallet>(self) -> TestExternalities
	where
		G: GenesisBuild<Config, Pallet>,
		Config: SystemConfig,
	{
		let storage = Self::system_storage::<Config>();
		let storage = self.assimilate_pallet_storage(storage);
		let mut ext: TestExternalities = storage.into();
		self.add_capabilities(&mut ext);
		ext
	}

	pub fn build_sans_config(self) -> TestExternalities {
		let ext = BasicExternalities::default();
		let mut ext: TestExternalities = ext.into_storages().into();
		self.add_capabilities(&mut ext);
		ext
	}
}

pub trait RollTo<R: SystemConfig> {
	type Pallet;

	fn with(_: R::BlockNumber);

	fn roll_to(n: R::BlockNumber) {
		let mut now = <System<R>>::block_number();
		while now < n {
			now += One::one();
			Self::with(now);
		}
	}
}

pub struct Trivial<P, R>(PhantomData<(P, R)>);

impl<Pallet, R: SystemConfig> RollTo<R> for Trivial<Pallet, R>
where
	Pallet: OnInitialize<R::BlockNumber> + OnFinalize<R::BlockNumber>,
{
	type Pallet = Pallet;

	fn with(now: R::BlockNumber) {
		System::<R>::set_block_number(now);
		Pallet::on_initialize(now);
		Pallet::on_finalize(now);
	}
}

pub struct WithWorkerHook<P, R>(PhantomData<(P, R)>);

impl<Pallet, R: SystemConfig> RollTo<R> for WithWorkerHook<Pallet, R>
where
	Pallet:
		OnInitialize<R::BlockNumber> + OnFinalize<R::BlockNumber> + OffchainWorker<R::BlockNumber>,
{
	type Pallet = Pallet;
	fn with(now: R::BlockNumber) {
		System::<R>::set_block_number(now);
		Pallet::on_initialize(now);
		Pallet::offchain_worker(now);
		Pallet::on_finalize(now);
	}
}

pub fn generate_account(seed: &str) -> AccountId32 {
	let seed = seed.bytes().cycle().take(32).collect::<Vec<_>>();
	let key_pair = sp_core::ecdsa::Pair::from_seed_slice(seed.as_slice()).unwrap();
	let pkey = key_pair.public();
	let signer: MultiSigner = pkey.into();
	signer.into_account()
}
