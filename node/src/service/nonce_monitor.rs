use std::time::Duration;

use codec::Decode;
use creditcoin_node_runtime::AccountId;
use jsonrpc_core::{futures::channel::mpsc, Failure, Response, Success};
use sc_client_api::Backend;
use sc_service::{Arc, RpcHandlers, RpcSession};
use sp_runtime::{app_crypto::Ss58Codec, offchain::OffchainStorage};
use substrate_prometheus_endpoint::Registry;

use super::FullBackend;

async fn rpc_request(
	handlers: &RpcHandlers,
	request: &str,
) -> Result<jsonrpc_core::serde_json::Value, String> {
	let (tx, _rx) = mpsc::unbounded();
	let session = RpcSession::new(tx);

	let response = handlers
		.rpc_query(&session, request)
		.await
		.ok_or_else(|| "empty response".to_string())?;

	let response: Response =
		jsonrpc_core::serde_json::from_str(&response).map_err(|e| e.to_string())?;

	let result = match response {
		Response::Single(out) => match out {
			jsonrpc_core::Output::Success(Success { result, .. }) => result,
			jsonrpc_core::Output::Failure(Failure { error, .. }) => return Err(error.to_string()),
		},
		Response::Batch(_) => unreachable!(),
	};

	Ok(result)
}

async fn get_on_chain_nonce(handlers: &RpcHandlers, acct: &AccountId) -> Result<u64, String> {
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

	result.as_u64().ok_or_else(|| "expected u64 response".to_string())
}

async fn get_off_chain_nonce_key(
	handlers: &RpcHandlers,
	acct: &AccountId,
) -> Result<Vec<u8>, String> {
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

	let key: Vec<u8> = jsonrpc_core::serde_json::from_value(result).map_err(|e| e.to_string())?;

	Ok(key)
}

async fn get_off_chain_nonce(backend: &FullBackend, key: &[u8]) -> Result<Option<u64>, String> {
	let off = backend.offchain_storage().unwrap().get(sp_offchain::STORAGE_PREFIX, key);

	let off = match off {
		None => return Ok(None),
		Some(v) => v,
	};
	let nonce = u32::decode(&mut off.as_slice()).map_err(|e| e.to_string())?;

	Ok(Some(nonce.into()))
}

type UIntGauge = substrate_prometheus_endpoint::prometheus::core::GenericGauge<
	substrate_prometheus_endpoint::prometheus::core::AtomicU64,
>;

fn register_u64_gauge(registry: &Registry, name: &str, help: &str) -> UIntGauge {
	substrate_prometheus_endpoint::register(
		substrate_prometheus_endpoint::prometheus::core::GenericGauge::<
			substrate_prometheus_endpoint::prometheus::core::AtomicU64,
		>::new(name, help)
		.unwrap(),
		&registry,
	)
	.unwrap()
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

	let acc = AccountId::from_string(&nonce_account).unwrap();
	let key = get_off_chain_nonce_key(&handlers, &acc).await.unwrap();

	loop {
		let onchain = get_on_chain_nonce(&handlers, &acc).await;
		let offchain = get_off_chain_nonce(&backend, &key).await;
		match (onchain, offchain) {
			(Ok(on), Ok(off)) => {
				log::info!("Onchain: {}, offchain: {:?}", on, off);
				offchain_gauge.set(off.unwrap_or(on));
				onchain_gauge.set(on);
			},
			(Err(e), _) | (_, Err(e)) => {
				log::error!("Error during nonce monitoring: {e}");
			},
		}
		tokio::time::sleep(POLL_INTERVAL).await;
	}
}
