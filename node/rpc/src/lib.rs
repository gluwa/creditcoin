use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::PubSubMetadata;
use primitives::metrics::MiningMetrics;

#[rpc]
pub trait CreditcoinApi {
	type Metadata: PubSubMetadata;

	#[rpc(name = "creditcoin_hashrate")]
	fn mining_stats(&self) -> RpcResult<MiningStats>;
}

pub struct CreditcoinRpc {
	mining_metrics: MiningMetrics,
}

impl CreditcoinRpc {
	pub fn new(mining_metrics: MiningMetrics) -> Self {
		Self { mining_metrics }
	}
}

pub enum Error {
	StorageError = 1,
	DecodeError,
	SubscriptionError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		e as i64
	}
}

impl CreditcoinApi for CreditcoinRpc {
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

#[cfg(test)]
mod test {
	use crate::*;
	use assert_matches::assert_matches;
	use std::time::Duration;

	#[test]
	#[allow(clippy::redundant_clone)]
	fn metrics_work() {
		let metrics = MiningMetrics::new(None).unwrap();
		let rpc = CreditcoinRpc::new(metrics);

		let result = rpc.mining_stats();
		assert_matches!(result.clone(), Ok(stats) => {
			assert_eq!(stats.hash_count, 0);
			assert!(stats.elapsed > Duration::from_secs(0));
			assert_eq!(stats.rate, 0.0);
		});

		// exercise Clone trait
		let stats = result.unwrap();
		let _ = stats.clone();
	}

	#[test]
	fn can_create_integer_from_error() {
		assert_eq!(i64::from(Error::StorageError), 1);
		assert_eq!(i64::from(Error::DecodeError), 2);
		assert_eq!(i64::from(Error::SubscriptionError), 3);
	}
}
