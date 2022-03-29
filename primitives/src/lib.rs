#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::U256;

pub type Difficulty = U256;

#[cfg(all(feature = "std", feature = "prometheus"))]
pub mod metrics {
	use std::sync::Arc;
	use substrate_prometheus_endpoint::{prometheus::IntCounter, PrometheusError, Registry};

	#[derive(Clone)]
	pub struct MiningMetrics {
		inner: Arc<MiningMetricsInner>,
	}

	impl MiningMetrics {
		pub fn new(registry: Option<&Registry>) -> Result<Self, PrometheusError> {
			Ok(MiningMetrics { inner: Arc::new(MiningMetricsInner::register(registry)?) })
		}

		pub fn inc(&self) {
			self.inner.count.inc();
		}

		pub fn add(&self, count: u64) {
			self.inner.count.inc_by(count);
		}

		pub fn count(&self) -> u64 {
			self.inner.count.get()
		}

		pub fn elapsed(&self) -> std::time::Duration {
			self.inner.start.elapsed()
		}
	}

	pub struct MiningMetricsInner {
		count: IntCounter,
		start: std::time::Instant,
	}

	impl MiningMetricsInner {
		fn register(registry: Option<&Registry>) -> Result<Self, PrometheusError> {
			Ok(MiningMetricsInner {
				count: if let Some(registry) = registry {
					substrate_prometheus_endpoint::register(
						IntCounter::new(
							"creditcoin_node_hash_count",
							"number of hashes produced by the node while mining",
						)?,
						registry,
					)?
				} else {
					IntCounter::new(
						"creditcoin_node_hash_count",
						"number of hashes produced by the node while mining",
					)?
				},
				start: std::time::Instant::now(),
			})
		}
	}
}
