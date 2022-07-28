use sp_keyring::AccountKeyring;
use std::error::Error;
use subxt::{ClientBuilder, DefaultConfig, PairSigner, SubstrateExtrinsicParams};

#[subxt::subxt(runtime_metadata_path = "creditcoin-metadata.scale")]
mod creditcoin {}

#[tokio::test]
async fn transfer() -> Result<(), Box<dyn Error + 'static>> {
	let signer = PairSigner::new(AccountKeyring::Alice.pair());

	let api: creditcoin::RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>> =
		ClientBuilder::new().build().await?.to_runtime_api();

	let dest = AccountKeyring::Bob.to_account_id().into();

	let extrinsic = api.tx().balances().transfer(dest, 100)?;

	let tx_hash = extrinsic.sign_and_submit_default(&signer).await?;

	Ok(())
}
