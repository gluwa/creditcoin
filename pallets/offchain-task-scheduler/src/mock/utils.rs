use super::runtime::{AccountId, Runtime};
use super::runtime::{BlockNumber, System, TaskScheduler};
use frame_support::traits::{GenesisBuild, OffchainWorker, OnFinalize, OnInitialize};
use frame_system as system;
pub(crate) use parking_lot::RwLock;
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
pub(crate) use sp_runtime::offchain::{
	testing::{OffchainState, PoolState, TestOffchainExt, TestTransactionPoolExt},
	OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
};
use sp_runtime::{traits::IdentifyAccount, RuntimeAppPublic};
pub(crate) use std::sync::Arc;

#[derive(Default)]
pub struct ExtBuilder {
	keystore: Option<KeyStore>,
	pool: Option<TestTransactionPoolExt>,
	pub offchain: Option<TestOffchainExt>,
	authorities: Vec<AccountId>,
}

impl ExtBuilder {
	pub(crate) fn generate_authority(&mut self) -> sp_core::sr25519::Public {
		const PHRASE: &str =
			"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
		let pubkey = self
			.keystore
			.as_ref()
			.expect("A keystore")
			.sr25519_generate_new(
				crate::crypto::Public::ID,
				Some(&format!("{}/auth{}", PHRASE, self.authorities.len() + 1)),
			)
			.unwrap();
		self.authorities.push(AccountId::new(pubkey.into_account().0));
		pubkey
	}

	pub(crate) fn with_keystore(mut self) -> Self {
		self.keystore = Some(KeyStore::new());
		self
	}

	pub(crate) fn with_pool(&mut self) -> Arc<RwLock<PoolState>> {
		let (pool, state) = TestTransactionPoolExt::new();
		self.pool = Some(pool);
		state
	}

	pub(crate) fn with_offchain(&mut self) -> Arc<RwLock<OffchainState>> {
		let (offchain, state) = TestOffchainExt::new();
		self.offchain = Some(offchain);
		state
	}

	pub(crate) fn build(self) -> TestExternalities {
		let mut storage = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		let _ = crate::pallet::GenesisConfig::<Runtime> { authorities: self.authorities }
			.assimilate_storage(&mut storage);

		let mut ext: TestExternalities = storage.into();

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
		ext
	}
}

pub(crate) trait RollTo<BlockNumber> {
	fn with(i: BlockNumber);
}

pub(crate) struct Trivial;

impl RollTo<BlockNumber> for Trivial {
	fn with(_i: BlockNumber) {}
}

pub(crate) struct WithWorkerHook;

impl RollTo<BlockNumber> for WithWorkerHook {
	fn with(i: BlockNumber) {
		TaskScheduler::offchain_worker(i);
	}
}

pub(crate) fn roll_to<T: RollTo<BlockNumber>>(n: BlockNumber) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		TaskScheduler::on_initialize(i);
		T::with(i);
		TaskScheduler::on_finalize(i);
	}
}
