use super::runtime::{AccountId, Runtime};
use crate::GenesisConfig;
use frame_support::traits::GenesisBuild;
use runtime_utils::ExtBuilder;
use sp_keystore::SyncCryptoStore;
use sp_runtime::{traits::IdentifyAccount, RuntimeAppPublic};

pub(crate) fn generate_authority(builder: &mut ExtBuilder, n: u32) -> sp_core::sr25519::Public {
	let pubkey = builder
		.keystore
		.as_ref()
		.expect("A keystore")
		.sr25519_generate_new(crate::crypto::Public::ID, Some(&format!("//{n}")))
		.unwrap();

	GenesisConfig::<Runtime> { authorities: vec![AccountId::new(pubkey.into_account().0)] }
		.assimilate_storage(&mut builder.storage)
		.unwrap();

	pubkey
}
