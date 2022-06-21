use codec::{Decode, Encode};
use core::cell::Cell;
use frame_support::RuntimeDebug;
use sp_runtime::offchain::storage::{MutateStorageError, StorageRetrievalError, StorageValueRef};

pub(crate) const TASK_GUARD: &[u8] = b"creditcoin/task/guard";

pub(super) enum Action {
	Verify,
	Drop,
}

#[allow(dead_code)]
pub struct LocalTaskStatus<'a> {
	pub storage_ref: StorageValueRef<'a>,
	//key: &'a [u8],
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
	pub fn new(storage_key: &'a [u8]) -> Self {
		Self {
			storage_ref: StorageValueRef::local(storage_key),
			//key: storage_key,
			keep_alive: Cell::new(false),
		}
	}

	pub fn keep_alive(&self) {
		self.keep_alive.set(true);
	}

	fn kill(&self) -> bool {
		!self.keep_alive.get()
	}

	//happy path, pick a task or skip it if it is already being processed.
	pub fn try_fast(&self) -> Result<(), Traps> {
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
	pub fn try_slow(&self) -> Option<()> {
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
}
