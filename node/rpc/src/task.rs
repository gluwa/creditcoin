use super::Error;
use core::marker::PhantomData;
use core::str::FromStr;
use creditcoin_node_runtime as runtime;
use jsonrpc_core::Result as RpcResult;
use jsonrpc_core::{Error as RpcError, ErrorCode};
use jsonrpc_derive::rpc;
use sc_rpc::DenyUnsafe;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits};
use std::sync::Arc;
use task_scheduler_runtime_api::TaskApi;

type AccountId = <runtime::Runtime as frame_system::Config>::AccountId;

#[rpc]
pub trait TaskRpc<AccountId> {
	#[rpc(name = "task_getOffchainNonceKey")]
	fn offchain_nonce_key(&self, account_id: String) -> RpcResult<Vec<u8>>;
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
	fn offchain_nonce_key(&self, account_id: String) -> RpcResult<Vec<u8>> {
		self.deny_unsafe.check_if_safe()?;
		let api = self.client.runtime_api();
		let at = {
			let best = self.client.info().best_hash;
			BlockId::hash(best)
		};

		let account_id = AccountId::from_str(&account_id).map_err(|e| RpcError {
			code: ErrorCode::InvalidParams,
			message: "Not a valid hex-string or SS58 address".into(),
			data: Some(format!("{:?}", e).into()),
		})?;

		api.offchain_nonce_key(&at, &account_id).map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query offchain nonce key.".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use creditcoin_node_runtime::Block;

	#[test]
	fn offchain_nonce_key_works() {
		let client = Arc::new(test_client::new());
		let t = Task::<_, Block>::new(client, DenyUnsafe::No);
		//$ ./node key inspect //Alice
		t.offchain_nonce_key("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".into())
			.unwrap();
		t.offchain_nonce_key(
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		)
		.unwrap();
	}

	#[test]
	fn offchain_nonce_key_should_error_when_input_is_not_a_valid_hex_string() {
		let client = Arc::new(test_client::new());
		let t = Task::<_, Block>::new(client, DenyUnsafe::No);

		match t.offchain_nonce_key("0xThisIsNotValid".into()) {
			Err(e) => {
				assert_eq!(e.to_string(), "Invalid params: Not a valid hex-string or SS58 address");
			},
			Ok(_) => panic!("This is not expected"),
		}
	}
}
