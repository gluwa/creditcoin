use parking_lot::RwLock;
use sp_core::offchain::TransactionPool;
use std::sync::Arc;

#[derive(Default)]
pub struct PoolState {
	/// A vector of transactions submitted from the runtime.
	pub transactions: Vec<Vec<u8>>,
}

#[derive(Default)]
pub struct TestTransactionPoolExt(Arc<RwLock<PoolState>>);

impl TestTransactionPoolExt {
	/// Create new `TestTransactionPoolExt` and a reference to the internal state.
	pub fn new() -> (Self, Arc<RwLock<PoolState>>) {
		let ext = Self::default();
		let state = ext.0.clone();
		(ext, state)
	}
}

impl TransactionPool for TestTransactionPoolExt {
	fn submit_transaction(&mut self, extrinsic: Vec<u8>) -> Result<(), ()> {
		CREATE_TRANSACTION_FAIL.with(|should_fail| {
			if should_fail.get() {
				tracing::error!(target: "pool", "Mocking submit_transaction failing!");
				Err(())
			} else {
				tracing::warn!(target: "pool", "Mocking submit_transaction not failing!");
				self.0.write().transactions.push(extrinsic);
				Ok(())
			}
		})
	}
}

use core::cell::Cell;
thread_local! {
	pub static CREATE_TRANSACTION_FAIL: Cell<bool> = Cell::new(false);
}

pub fn with_failing_submit_transaction<R>(f: impl FnOnce() -> R) -> R {
	CREATE_TRANSACTION_FAIL.with(|c| {
		c.set(true);
		let result = f();
		c.set(false);
		result
	})
}
