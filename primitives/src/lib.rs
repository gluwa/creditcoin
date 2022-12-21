#![cfg_attr(not(feature = "std"), no_std)]

pub mod vrf;

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

#[cfg(test)]
mod test {
	use super::metrics::*;
	use substrate_prometheus_endpoint::Registry;

	#[test]
	fn metrics_works_when_starting_without_registry() {
		let metrics = MiningMetrics::new(None).unwrap();
		let initial_count = metrics.count();
		assert_eq!(initial_count, 0);

		// increase
		metrics.inc();
		assert_eq!(metrics.count(), initial_count + 1);

		// add
		metrics.add(4);
		assert_eq!(metrics.count(), initial_count + 5);

		// elapsed time
		assert!(metrics.elapsed().as_nanos() > 0);
	}

	#[test]
	fn metrics_works_when_starting_with_registry() {
		let registry = Registry::new();
		let metrics = MiningMetrics::new(Some(&registry)).unwrap();
		let initial_count = metrics.count();
		assert_eq!(initial_count, 0);

		// increase
		metrics.inc();
		assert_eq!(metrics.count(), initial_count + 1);

		// add
		metrics.add(4);
		assert_eq!(metrics.count(), initial_count + 5);

		// elapsed time
		assert!(metrics.elapsed().as_nanos() > 0);

		// there's only 1 metric counter registered in MiningMetricsInner::register()
		let results = registry.gather();
		assert_eq!(results.len(), 1);
	}

	#[test]
	fn metrics_clone_trait_works() {
		let metrics = MiningMetrics::new(None).unwrap();
		metrics.inc();

		let new_metrics = metrics.clone();
		drop(metrics);
		assert_eq!(new_metrics.count(), 1);
	}
}
