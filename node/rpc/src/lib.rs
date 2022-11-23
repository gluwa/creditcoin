use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use primitives::metrics::MiningMetrics;

#[rpc(client, server)]
pub trait CreditcoinApi {
	#[method(name = "creditcoin_hashrate")]
	async fn mining_stats(&self) -> RpcResult<MiningStats>;
}

pub struct Creditcoin {
	mining_metrics: MiningMetrics,
}

impl Creditcoin {
	pub fn new(mining_metrics: MiningMetrics) -> Self {
		Self { mining_metrics }
	}
}

pub enum Error {
	StorageError = 1,
	DecodeError,
	SubscriptionError,
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		e as i32
	}
}

#[async_trait]
impl CreditcoinApiServer for Creditcoin {
	async fn mining_stats(&self) -> RpcResult<MiningStats> {
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

mod task;
pub use task::{Task, TaskApiServer};

#[cfg(test)]
mod test {
	use super::*;
	use assert_matches::assert_matches;
	use std::time::Duration;

	#[tokio::test]
	#[allow(clippy::redundant_clone)]
	async fn metrics_work() {
		let metrics = MiningMetrics::new(None).unwrap();
		let rpc = Creditcoin::new(metrics);

		let result = rpc.mining_stats().await.map_err(|_| ());
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
		assert_eq!(i32::from(Error::StorageError), 1);
		assert_eq!(i32::from(Error::DecodeError), 2);
		assert_eq!(i32::from(Error::SubscriptionError), 3);
	}
}
