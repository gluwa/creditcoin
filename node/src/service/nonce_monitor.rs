use std::time::Duration;

use codec::Decode;
use creditcoin_node_runtime::AccountId;
use jsonrpc_core::{futures::channel::mpsc, Failure, Response, Success};
use sc_client_api::Backend;
use sc_service::{Arc, RpcHandlers, RpcSession};
use sp_runtime::{app_crypto::Ss58Codec, offchain::OffchainStorage};
use substrate_prometheus_endpoint::Registry;
use thiserror::Error;

use super::FullBackend;

#[derive(Clone, Copy, PartialEq, Debug, Error)]
enum ErrorKind {
	#[error("Serde error")]
	Serde,
	#[error("RPC error")]
	Rpc,
	#[error("Codec error")]
	Codec,
}

#[derive(Debug, Error)]
#[error("{kind}: {value}")]
struct Error {
	kind: ErrorKind,
	#[source]
	value: Box<dyn std::error::Error + Send>,
}

macro_rules! error_fn {
	($($fn_id: ident : $kind: ident),+ $(,)?) => {
		impl Error {
			$(
				fn $fn_id(err: impl std::error::Error + Send + 'static) -> Self {
					Self::new(ErrorKind::$kind, err)
				}
			)+
		}
	};
}

error_fn!(serde: Serde, rpc: Rpc, codec: Codec);

impl Error {
	fn new(kind: ErrorKind, err: impl std::error::Error + Send + 'static) -> Self {
		Self { kind, value: Box::new(err) }
	}
}

#[derive(Clone, PartialEq, Debug, Error)]
#[error("{message}")]
struct AdhocError {
	message: String,
}

fn adhoc(message: impl Into<String>) -> AdhocError {
	AdhocError { message: message.into() }
}

async fn rpc_request(
	handlers: &RpcHandlers,
	request: &str,
) -> Result<jsonrpc_core::serde_json::Value, Error> {
	let (tx, _rx) = mpsc::unbounded();
	let session = RpcSession::new(tx);

	let response = handlers
		.rpc_query(&session, request)
		.await
		.ok_or_else(|| Error::rpc(adhoc("empty response")))?;

	let response: Response = jsonrpc_core::serde_json::from_str(&response).map_err(Error::serde)?;

	let result = match response {
		Response::Single(out) => match out {
			jsonrpc_core::Output::Success(Success { result, .. }) => result,
			jsonrpc_core::Output::Failure(Failure { error, .. }) => return Err(Error::rpc(error)),
		},
		Response::Batch(_) => {
			unreachable!("we don't send any batch requests, so we cannot receive batch responses")
		},
	};

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

	result.as_u64().ok_or_else(|| Error::rpc(adhoc("expected u64 response")))
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

	let key: Vec<u8> = jsonrpc_core::serde_json::from_value(result).map_err(Error::serde)?;

	Ok(key)
}

async fn get_off_chain_nonce(backend: &FullBackend, key: &[u8]) -> Result<Option<u64>, Error> {
	let off = backend.offchain_storage().expect("offchain storage must be accessible in a creditcoin node. \
		we only support the file-backed storage backend which always has offchain storage; qed").get(sp_offchain::STORAGE_PREFIX, key);

	let off = match off {
		None => return Ok(None),
		Some(v) => v,
	};
	let nonce = u32::decode(&mut off.as_slice()).map_err(Error::codec)?;

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

const POLL_INTERVAL: Duration = Duration::from_secs(30);

pub(super) async fn task(
	registry: Registry,
	nonce_account: String,
	handlers: RpcHandlers,
	backend: Arc<FullBackend>,
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

	let acc = AccountId::from_string(&nonce_account)
		.expect("Invalid account ID provided for nonce monitoring");
	let key = get_off_chain_nonce_key(&handlers, &acc)
		.await
		.expect("Failed to get key for the offchain nonce");

	loop {
		let onchain = get_on_chain_nonce(&handlers, &acc).await;
		let offchain = get_off_chain_nonce(&backend, &key).await;
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
