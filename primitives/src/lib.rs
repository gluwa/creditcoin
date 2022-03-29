#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::U256;

pub type Difficulty = U256;

#[cfg(feature = "std")]
pub mod metrics {
	use std::sync::atomic::AtomicU64;
	use std::sync::Arc;
	#[derive(Clone)]
	pub struct MiningMetrics {
		inner: Arc<MiningMetricsInner>,
	}

	impl MiningMetrics {
		pub fn new() -> Self {
			MiningMetrics { inner: Arc::new(MiningMetricsInner::new()) }
		}

		pub fn inc(&self) {
			self.inner.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		}

		pub fn add(&self, count: u64) {
			self.inner.count.fetch_add(count, std::sync::atomic::Ordering::Relaxed);
		}

		pub fn count(&self) -> u64 {
			self.inner.count.load(std::sync::atomic::Ordering::Relaxed)
		}

		pub fn elapsed(&self) -> std::time::Duration {
			self.inner.start.elapsed()
		}
	}

	pub struct MiningMetricsInner {
		count: AtomicU64,
		start: std::time::Instant,
	}

	impl MiningMetricsInner {
		fn new() -> Self {
			MiningMetricsInner { count: AtomicU64::new(0), start: std::time::Instant::now() }
		}
	}
}
