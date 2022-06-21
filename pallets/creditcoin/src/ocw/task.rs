use alloc::vec::Vec;
use codec::Encode;
use frame_support::storage::PrefixIterator;
use guard::{Action, LocalTaskStatus, Traps, TASK_GUARD};

use super::{Config, Pallet, VerificationFailureCause, VerificationResult};

pub mod guard;

pub(super) trait Task<T: Config, D: core::fmt::Debug, K2> {
	type VerifiedTask;
	fn verify(&self) -> VerificationResult<Self::VerifiedTask>;
	fn status_key<Id: Encode>(id: &Id) -> Vec<u8> {
		id.using_encoded(|encoded_id| {
			TASK_GUARD.iter().chain(b"/".iter()).chain(encoded_id).copied().collect()
		})
	}
	fn status(storage_key: &[u8]) -> LocalTaskStatus<'_> {
		LocalTaskStatus::new(storage_key)
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
				let status = V::status(&key);

				let action = match status.try_fast() {
					Ok(_) => Action::Verify,
					Err(Traps::Skip) => return Some(()),
					Err(Traps::Unscheduled) => match status.try_slow() {
						Some(_) => {
							if V::is_complete(k2.clone()) {
								log::debug!("Already handled Task {:?}", k2);
								Action::Drop
							} else {
								Action::Verify
							}
						},
						None => return Some(()),
					},
				};

				let result = match action {
					Action::Verify => V::verify(&v),
					//test is fatal
					Action::Drop => Err(VerificationFailureCause::TaskDuplicated.into()),
				};

				let auth_id = self.auth_id.clone();

				let on_success = |verified_task: V::VerifiedTask| {
					Pallet::<T>::offchain_signed_tx(auth_id.clone(), |_| {
						v.success_call(k1, verified_task.clone())
					})
				};

				let on_failure = |cause: VerificationFailureCause| {
					Pallet::<T>::offchain_signed_tx(auth_id.clone(), |_| v.failure_call(k1, cause))
				};

				Pallet::<T>::ocw_result_handler(result, on_success, on_failure, status, &v);

				Some(())
			},
			None => None,
		}
	}
}

mod tests;
