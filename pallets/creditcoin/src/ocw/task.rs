use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::cell::Cell;
use frame_support::{storage::PrefixIterator, RuntimeDebug};
use sp_runtime::offchain::storage::{MutateStorageError, StorageRetrievalError, StorageValueRef};

use super::{Config, Pallet, VerificationFailureCause, VerificationResult};

pub(crate) const TASK_GUARD: &[u8] = b"creditcoin/task/guard";

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

enum Action {
	Verify,
	Drop,
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

pub(crate) struct LocalTaskStatus<'a> {
	storage_ref: StorageValueRef<'a>,
	key: &'a [u8],
	keep_alive: Cell<bool>,
}

#[derive(RuntimeDebug)]
pub enum Traps {
	Skip,
	Unscheduled,
}

trait MachineState {
	type State;

	//Communicate through errors when values are not supposed to be swapped
	fn fast_path(
		v: Result<Option<Self::State>, StorageRetrievalError>,
	) -> Result<Self::State, Traps>;

	fn slow_path(
		v: Result<Option<Self::State>, StorageRetrievalError>,
	) -> Result<Self::State, Traps>;
}

#[derive(Encode, Decode, RuntimeDebug)]
enum PendingState {
	Enqueued,
	Acquired,
}

impl<'a> MachineState for LocalTaskStatus<'a> {
	type State = PendingState;

	//Communicate through Traps when values are not swapped
	fn fast_path(
		v: Result<Option<Self::State>, StorageRetrievalError>,
	) -> Result<Self::State, Traps> {
		use PendingState::*;
		use Traps::*;
		//test that it is safe to enqueue when decoding failed.
		let v = if let Ok(a) = v {
			a
		} else {
			log::debug!("Undecodable Task Status");
			None
		};

		let x = match v {
			//Missing, is_complete? fail to remove unverified transfer : enqueue
			None => Err(Unscheduled)?,
			//Acquired, raise skip error.
			Some(Acquired) => Err(Skip)?,
			//Queued, free to take.
			Some(Enqueued) => Acquired,
		};
		Ok(x)
	}

	fn slow_path(
		v: Result<Option<Self::State>, StorageRetrievalError>,
	) -> Result<Self::State, Traps> {
		use PendingState::*;
		use Traps::*;
		let v = if let Ok(a) = v {
			a
		} else {
			log::debug!("Undecodable Task Status");
			None
		};

		let x = match v {
			//The caller has dissambiguated the state, acquire the guard and proceed accordingly.
			None => Acquired,
			Some(Acquired) => Err(Skip)?,
			Some(Enqueued) => Acquired,
		};
		Ok(x)
	}
}

impl<'a> Drop for LocalTaskStatus<'a> {
	fn drop(&mut self) {
		if self.kill() {
			self.storage_ref.clear();
		} else {
			self.storage_ref.set(&PendingState::Enqueued);
		}
	}
}

impl<'a> LocalTaskStatus<'a> {
	pub(crate) fn new(storage_key: &'a [u8]) -> Self {
		Self {
			storage_ref: StorageValueRef::local(storage_key),
			key: storage_key,
			keep_alive: Cell::new(false),
		}
	}

	//happy path, pick a task or skip it if it is already being processed.
	pub(super) fn try_fast(&self) -> Result<(), Traps> {
		match self.storage_ref.mutate::<PendingState, _, _>(Self::fast_path) {
			Ok(_) => Ok(()),
			Err(MutateStorageError::ValueFunctionFailed(e)) => Err(e),
			Err(pending_state @ MutateStorageError::ConcurrentModification(_)) => {
				//test if contention triggers this, it shouldn't.
				log::warn!("Task {pending_state:?}");
				Err(Traps::Skip)
			},
		}
	}

	// The task in unscheduled, take the guard if available and process it.
	pub(super) fn try_slow(&self) -> Option<()> {
		match self.storage_ref.mutate::<PendingState, _, _>(Self::slow_path) {
			Ok(_) => Some(()),
			Err(MutateStorageError::ValueFunctionFailed(Traps::Skip)) => None,
			Err(MutateStorageError::ValueFunctionFailed(Traps::Unscheduled)) => unreachable!(),
			Err(pending_state @ MutateStorageError::ConcurrentModification(_)) => {
				//test if contention triggers this, it shouldn't.
				log::warn!("Task {pending_state:?}");
				None
			},
		}
	}

	pub fn keep_alive(&self) {
		self.keep_alive.set(true);
	}

	fn kill(&self) -> bool {
		!self.keep_alive.get()
	}
}
