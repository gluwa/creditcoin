use std::{convert::TryInto, time::Duration};

use creditcoin_node_runtime::AccountId;
use futures::join;
use parity_scale_codec::Decode;
use sc_client_api::Backend;
use sc_service::{Arc, RpcHandlers};
use sp_keystore::CryptoStore;
use sp_runtime::{
	app_crypto::Ss58Codec, offchain::OffchainStorage, traits::IdentifyAccount, MultiSigner,
};
use substrate_prometheus_endpoint::Registry;
use thiserror::Error;

use crate::cli::NonceMonitorTarget;

use super::FullBackend;

#[derive(Debug, Error)]
#[error("{0}")]
enum Error {
	Serde(sc_telemetry::serde_json::Error),
	JsonRpc(jsonrpsee::core::error::Error),
	Rpc(String),
	Codec(parity_scale_codec::Error),
	KeyStore(String),
	Signer(String),
}

impl From<jsonrpsee::core::Error> for Error {
	fn from(value: jsonrpsee::core::Error) -> Self {
		Error::JsonRpc(value)
	}
}

async fn rpc_request(handlers: &RpcHandlers, request: &str) -> Result<serde_json::Value, Error> {
	let (response, _stream) = handlers.rpc_query(request).await?;

	let result = serde_json::from_str(&response).map_err(Error::Serde)?;

	Ok(result)
}

async fn get_on_chain_nonce(handlers: &RpcHandlers, acct: &AccountId) -> Result<u64, Error> {
	let request = format!(
		r#"{{
            "jsonrpc": "2.0",
            "method": "system_accountNextIndex",
            "params": ["{}"],
            "id": 0
        }}"#,
		acct.to_ss58check()
	);

	let result = rpc_request(handlers, &request).await?;

	result.as_u64().ok_or_else(|| Error::Rpc("expected u64 response".into()))
}

async fn get_off_chain_nonce_key(
	handlers: &RpcHandlers,
	acct: &AccountId,
) -> Result<Vec<u8>, Error> {
	let request = format!(
		r#"{{
            "jsonrpc": "2.0",
            "method": "task_getOffchainNonceKey",
            "params": ["{}"],
            "id": 0
        }}"#,
		acct.to_ss58check()
	);

	let result = rpc_request(handlers, &request).await?;

	let key: Vec<u8> = jsonrpc_core::serde_json::from_value(result).map_err(Error::Serde)?;

	Ok(key)
}

async fn get_off_chain_nonce(backend: &FullBackend, key: &[u8]) -> Result<Option<u64>, Error> {
	let off = backend
		.offchain_storage()
		.expect(
			"offchain storage must be accessible in a creditcoin node. \
				we only support the file-backed storage backend which always has offchain storage; qed",
		)
		.get(sp_offchain::STORAGE_PREFIX, key);

	let off = match off {
		Some(v) => v,
		None => return Ok(None),
	};
	let nonce = u32::decode(&mut off.as_slice()).map_err(Error::Codec)?;

	Ok(Some(nonce.into()))
}

type UIntGauge = substrate_prometheus_endpoint::prometheus::core::GenericGauge<
	substrate_prometheus_endpoint::prometheus::core::AtomicU64,
>;

fn register_u64_gauge(registry: &Registry, name: &str, help: &str) -> UIntGauge {
	substrate_prometheus_endpoint::register(
		UIntGauge::new(name, help).expect("gauge creation should not fail"),
		registry,
	)
	.expect("registering prometheus gauge should not fail")
}

type Keystore = Arc<dyn CryptoStore>;

async fn get_authority_account(
	target: NonceMonitorTarget,
	keystore: &Keystore,
) -> Result<Option<AccountId>, Error> {
	Ok(match target {
		NonceMonitorTarget::Auto => {
			let keys = keystore
				.keys(sp_runtime::KeyTypeId(*b"gots"))
				.await
				.map_err(|e| Error::KeyStore(e.to_string()))?;
			keys.into_iter()
				.next()
				.map(|key| {
					Ok::<_, Error>(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(
						key.1.try_into().map_err(|e| {
							Error::Signer(format!(
								"Invalid authority account from public key: {}",
								hex::encode(e)
							))
						})?,
					)))
				})
				.transpose()?
				.map(|signer| signer.into_account())
		},

		NonceMonitorTarget::Account(acct) => Some(acct),
	})
}

const POLL_INTERVAL: Duration = Duration::from_secs(30);

pub(super) struct TaskArgs {
	pub(super) registry: Registry,
	pub(super) monitor_target: NonceMonitorTarget,
	pub(super) handlers: RpcHandlers,
	pub(super) backend: Arc<FullBackend>,
	pub(super) keystore: Keystore,
}

pub(super) async fn task(
	TaskArgs { registry, monitor_target, handlers, backend, keystore }: TaskArgs,
) {
	let offchain_gauge = register_u64_gauge(
		&registry,
		"authority_offchain_nonce",
		"the nonce for the authority in offchain storage",
	);
	let onchain_gauge = register_u64_gauge(
		&registry,
		"authority_onchain_nonce",
		"the nonce for the authority in onchain storage",
	);

	let nonce_account =
		loop {
			match get_authority_account(monitor_target.clone(), &keystore).await {
				Ok(Some(acct)) => break acct,
				Ok(None) => {
					log::info!("No authority account found");
					tokio::time::sleep(POLL_INTERVAL * 2).await
				},
				Err(e) => {
					log::error!("Encountered error when trying to get authority account for monitoring: {e}");
					return;
				},
			}
		};

	let key = match get_off_chain_nonce_key(&handlers, &nonce_account).await {
		Ok(key) => key,
		Err(e) => {
			log::error!("Failed to get key for the offchain nonce of {nonce_account}: {e}");
			return;
		},
	};

	loop {
		let (onchain, offchain) = join!(
			get_on_chain_nonce(&handlers, &nonce_account),
			get_off_chain_nonce(&backend, &key)
		);

		match (onchain, offchain) {
			(Ok(on), Ok(off)) => {
				log::info!("Onchain: {}, offchain: {:?}", on, off);
				offchain_gauge.set(off.unwrap_or(on));
				onchain_gauge.set(on);
			},
			(Err(e), Err(e2)) => {
				log::error!("Errors during nonce monitoring: {e} ; {e2}");
			},
			(Err(e), _) | (_, Err(e)) => {
				log::error!("Error during nonce monitoring: {e}");
			},
		}
		tokio::time::sleep(POLL_INTERVAL).await;
	}
}
