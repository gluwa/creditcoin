use futures::prelude::*;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{
	PubSubMetadata,
};
use sc_client_api::{BlockchainEvents};
use sp_api::{ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};
pub(crate) type Hash = <creditcoin_node_runtime::Runtime as frame_system::Config>::Hash;
pub(crate) type AccountId = <creditcoin_node_runtime::Runtime as frame_system::Config>::AccountId;
pub(crate) type BlockNumber =
	<creditcoin_node_runtime::Runtime as frame_system::Config>::BlockNumber;
pub(crate) type Moment = <creditcoin_node_runtime::Runtime as pallet_timestamp::Config>::Moment;
use primitives::metrics::MiningMetrics;

#[rpc]
pub trait CreditcoinApi<BlockHash> {
	type Metadata: PubSubMetadata;

	#[rpc(name = "creditcoin_hashrate")]
	fn mining_stats(&self) -> RpcResult<MiningStats>;
}

pub struct CreditcoinRpc<C, P> {
	client: Arc<C>,
	mining_metrics: MiningMetrics,
	_marker: PhantomData<P>,
}

impl<C, P> CreditcoinRpc<C, P> {
	pub fn new( client: Arc<C>, mining_metrics: MiningMetrics) -> Self {
		Self {
			client,
			_marker: PhantomData,
			mining_metrics,
		}
	}
}

pub enum Error {
	StorageError=1,
	DecodeError,
	SubscriptionError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		e as i64
	}
}

// manual currying so you can do `thing.map_err(decode_error("foo"))`
// instead of `thing.map_err(|e| handle_decode_error("foo", e))`
fn decode_error(name: impl AsRef<str>) -> impl FnOnce(codec::Error) -> RpcError {
	let name = name.as_ref().to_owned();
	move |e| handle_decode_error(name, e)
}

fn handle_decode_error(name: impl AsRef<str>, error: codec::Error) -> RpcError {
	RpcError {
		code: ErrorCode::ServerError(Error::DecodeError.into()),
		message: format!("Unable to decode {}: {}", name.as_ref(), error),
		data: None,
	}
}

impl<C, Block> CreditcoinApi<Block::Hash> for CreditcoinRpc<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>
{
	type Metadata = sc_rpc::Metadata;

	fn mining_stats(&self) -> RpcResult<MiningStats> {
		let hash_count = self.mining_metrics.count();
		let elapsed = self.mining_metrics.elapsed();
		let rate = hash_count as f64 / elapsed.as_secs_f64();
		Ok(MiningStats { hash_count, elapsed, rate })
	}
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MiningStats {
	hash_count: u64,
	elapsed: std::time::Duration,
	rate: f64,
}
