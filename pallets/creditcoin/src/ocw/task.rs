use super::{Config, Pallet, VerificationFailureCause, VerificationResult};
use alloc::vec::Vec;
use codec::Encode;
use frame_support::{storage::PrefixIterator, traits::Get};
use frame_system::Pallet as System;
use sp_runtime::offchain::{storage_lock::StorageLock, Duration};
pub(super) use sp_runtime::{
	offchain::storage_lock::{BlockAndTime, StorageLockGuard},
	SaturatedConversion,
};

pub(super) trait Task<T: Config, D: core::fmt::Debug, K2: Encode> {
	type VerifiedTask;
	fn verify(&self) -> VerificationResult<Self::VerifiedTask>;
	fn status_key(id: &K2) -> Vec<u8> {
		id.using_encoded(|encoded_id| {
			b"creditcoin/task/lock/".iter().chain(encoded_id).copied().collect()
		})
	}
	fn failure_call(&self, deadline: D, cause: VerificationFailureCause) -> crate::Call<T>;
	fn success_call(&self, deadline: D, verified_task: Self::VerifiedTask) -> crate::Call<T>;
	fn is_complete(persistent_storage_key: K2) -> bool;
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct TaskIterator<K2, V, T: Config, A: Clone> {
	iter_over: PrefixIterator<(T::BlockNumber, K2, V)>,
	auth_id: A,
}

impl<K2, V, T: Config, A: Clone> TaskIterator<K2, V, T, A> {
	pub fn new(iter_over: PrefixIterator<(T::BlockNumber, K2, V)>, auth_id: A) -> Self {
		Self { iter_over, auth_id }
	}
}

impl<T, K2, V> Iterator for TaskIterator<K2, V, T, T::FromAccountId>
where
	T: Config,
	V: Task<T, T::BlockNumber, K2> + core::fmt::Debug,
	K2: core::fmt::Debug + Clone + Encode,
	<V as Task<T, T::BlockNumber, K2>>::VerifiedTask: core::fmt::Debug + Clone,
{
	type Item = ();

	fn next(&mut self) -> Option<Self::Item> {
		match self.iter_over.next() {
			Some((k1, k2, v)) => {
				let key = V::status_key(&k2);
				let offset =
					T::UnverifiedTaskTimeout::get().saturated_into::<u32>().saturating_sub(2u32);
				let mut lock = StorageLock::<BlockAndTime<System<T>>>::with_block_and_time_deadline(
					&key,
					offset,
					Duration::from_millis(0),
				);
				let guard = match lock.try_lock() {
					Ok(g) => g,
					Err(_) => return Some(()),
				};

				if V::is_complete(k2.clone()) {
					log::debug!("Already handled Task {:?}", k2);
					guard.forget();
					return Some(());
				}

				let result = V::verify(&v);

				let auth_id = self.auth_id.clone();

				let on_success = |verified_task: V::VerifiedTask| {
					Pallet::<T>::offchain_signed_tx(auth_id.clone(), |_| {
						v.success_call(k1, verified_task.clone())
					})
				};

				let on_failure = |cause: VerificationFailureCause| {
					Pallet::<T>::offchain_signed_tx(auth_id.clone(), |_| v.failure_call(k1, cause))
				};

				Pallet::<T>::ocw_result_handler(result, on_success, on_failure, guard, &v);

				Some(())
			},
			None => None,
		}
	}
}

mod tests;
