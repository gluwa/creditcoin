use super::Error;
use core::marker::PhantomData;
use core::str::FromStr;
use creditcoin_node_runtime as runtime;
use sc_rpc::DenyUnsafe;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits};
use std::sync::Arc;
use task_scheduler_runtime_api::TaskApi;

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::{
		error::{CallError, ErrorCode},
		ErrorObject,
	},
};

type AccountId = <runtime::Runtime as frame_system::Config>::AccountId;

#[rpc(client, server)]
pub trait TaskApi<AccountId> {
	#[method(name = "task_getOffchainNonceKey")]
	async fn offchain_nonce_key(&self, account_id: String) -> RpcResult<Vec<u8>>;
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

#[async_trait]
impl<C, B> TaskApiServer<AccountId> for Task<C, B>
where
	C: sp_api::ProvideRuntimeApi<B>,
	C: HeaderBackend<B>,
	C: Send + Sync + 'static,
	C::Api: TaskApi<B, AccountId>,
	B: traits::Block,
{
	async fn offchain_nonce_key(&self, account_id: String) -> RpcResult<Vec<u8>> {
		self.deny_unsafe.check_if_safe()?;
		let api = self.client.runtime_api();
		let at = {
			let best = self.client.info().best_hash;
			BlockId::hash(best)
		};
		let account_id = AccountId::from_str(&account_id).map_err(|e| {
			JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
				ErrorCode::InvalidParams.code(),
				"Not a valid hex-string or SS58 address.",
				Some(format!("{:?}", e)),
			)))
		})?;

		api.offchain_nonce_key(&at, &account_id).map_err(|e| {
			JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
				ErrorCode::ServerError(Error::RuntimeError.into()).code(),
				"Unable to query offchain nonce key.",
				Some(format!("{e:?}")),
			)))
		})
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use creditcoin_node_runtime::Block;

	#[tokio::test]
	async fn offchain_nonce_key_works() {
		let client = Arc::new(test_client::new());
		let t = Task::<_, Block>::new(client, DenyUnsafe::No);
		//$ ./node key inspect //Alice
		t.offchain_nonce_key("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".into())
			.await
			.unwrap();
		t.offchain_nonce_key(
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".into(),
		)
		.await
		.unwrap();
	}

	#[tokio::test]
	async fn offchain_nonce_key_should_error_when_input_is_not_a_valid_hex_string() {
		let client = Arc::new(test_client::new());
		let t = Task::<_, Block>::new(client, DenyUnsafe::No);

		match t.offchain_nonce_key("0xThisIsNotValid".into()).await {
			Err(e) => {
				assert_eq!(
					e.to_string(),
					r#"RPC call failed: ErrorObject { code: InvalidParams, message: "Not a valid hex-string or SS58 address.", data: Some(RawValue("\"invalid ss58 address.\"")) }"#
				);
			},
			Ok(_) => panic!("This is not expected"),
		}
	}
}
