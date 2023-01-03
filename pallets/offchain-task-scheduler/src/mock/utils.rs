use super::runtime::{AccountId, Runtime};
use crate::GenesisConfig;
use runtime_utils::ExtBuilder;
use sp_keystore::SyncCryptoStore;
use sp_runtime::{traits::IdentifyAccount, RuntimeAppPublic};

pub(crate) fn generate_authority(
	builder: &mut ExtBuilder<GenesisConfig<Runtime>>,
) -> sp_core::sr25519::Public {
	const PHRASE: &str =
		"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
	let pubkey = builder
		.keystore
		.as_ref()
		.expect("A keystore")
		.sr25519_generate_new(
			crate::crypto::Public::ID,
			Some(&format!("{}/auth{}", PHRASE, builder.genesis_config.authorities.len() + 1)),
		)
		.unwrap();
	builder.genesis_config.authorities.push(AccountId::new(pubkey.into_account().0));
	pubkey
}
