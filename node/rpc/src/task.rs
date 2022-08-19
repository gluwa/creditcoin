use super::Error;
use core::marker::PhantomData;
use core::str::FromStr;
use creditcoin_runtime_api::TaskApi;
use jsonrpc_core::Result as RpcResult;
use jsonrpc_core::{Error as RpcError, ErrorCode};
use jsonrpc_derive::rpc;
use sc_rpc::DenyUnsafe;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits};
use std::sync::Arc;

type AccountId = <creditcoin_node_runtime::Runtime as frame_system::Config>::AccountId;

#[rpc]
pub trait TaskRpc<AccountId> {
	#[rpc(name = "task_getOffchainNonceKey")]
	fn offchain_nonce_key(&self, acc: String) -> RpcResult<Vec<u8>>;
}

pub struct Task<C, B> {
	client: Arc<C>,
	deny_unsafe: DenyUnsafe,
	_p: PhantomData<B>,
}

impl<C, B> Task<C, B> {
	pub fn new(client: Arc<C>, deny_unsafe: DenyUnsafe) -> Self {
		Self { deny_unsafe, client, _p: Default::default() }
	}
}

impl<C, B> TaskRpc<AccountId> for Task<C, B>
where
	C: sp_api::ProvideRuntimeApi<B>,
	C: HeaderBackend<B>,
	C: Send + Sync + 'static,
	C::Api: TaskApi<B, AccountId>,
	B: traits::Block,
{
	fn offchain_nonce_key(&self, acc: String) -> RpcResult<Vec<u8>> {
		self.deny_unsafe.check_if_safe()?;
		let api = self.client.runtime_api();
		let at = {
			let best = self.client.info().best_hash;
			BlockId::hash(best)
		};

		let acc = AccountId::from_str(&acc).map_err(|e| RpcError {
			code: ErrorCode::InvalidParams,
			message: "Not a valid hex-string or SS58 address".into(),
			data: Some(format!("{:?}", e).into()),
		})?;

		api.offchain_nonce_key(&at, &acc).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query offchain nonce key.".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
