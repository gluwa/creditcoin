#![cfg(test)]

pub(crate) mod runtime;
pub(crate) mod task;

use frame_support::traits::GenesisBuild;
use frame_system as system;
use runtime::{AccountId, Runtime};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt};
pub(crate) use sp_runtime::offchain::{
	testing::{TestOffchainExt, TestTransactionPoolExt},
	OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
};
pub(crate) use std::sync::Arc;

#[derive(Default)]
pub struct ExtBuilder {
	keystore: Option<KeyStore>,
	pool: Option<TestTransactionPoolExt>,
	pub offchain: Option<TestOffchainExt>,
	authorities: Vec<AccountId>,
}

impl ExtBuilder {
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
