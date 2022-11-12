pub mod error;
pub mod macros;

pub use super::pallet::Config;
use crate::SystemConfig;
use alloc::fmt::Debug;
use error::TaskError;
use frame_support::dispatch::Vec;
use frame_support::traits::Get;
use sp_core::offchain::Duration;
use sp_core::Encode;
use sp_runtime::offchain::storage_lock::BlockAndTime;
use sp_runtime::offchain::storage_lock::StorageLock;
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::SaturatedConversion;

#[inline]
pub(crate) fn storage_key<Id: Encode>(id: &Id) -> Vec<u8> {
	const TASK_GUARD: &[u8] = b"offchain-task-scheduler/task/guard/";
	id.using_encoded(|encoded_id| TASK_GUARD.iter().chain(encoded_id).copied().collect())
}

type Lock<'a, BlockNumberProvider> = StorageLock<'a, BlockAndTime<BlockNumberProvider>>;

pub(crate) fn task_lock<Runtime: Config>(storage_key: &[u8]) -> Lock<frame_system::Pallet<Runtime>>
where
	frame_system::Pallet<Runtime>: BlockNumberProvider,
{
	let offset = Runtime::UnverifiedTaskTimeout::get()
		.saturated_into::<u32>()
		.saturating_sub(2u32);
	Lock::<frame_system::Pallet<Runtime>>::with_block_and_time_deadline(
		storage_key,
		offset,
		Duration::from_millis(0),
	)
}

pub trait ForwardTask<Runtime: SystemConfig> {
	type Call;
	type EvaluationError: Debug;
	type SchedulerError: Debug;
	fn forward_task(
		&self,
		deadline: Runtime::BlockNumber,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>>;
}

pub trait TaskV2<Runtime: SystemConfig> {
	type Call;
	type EvaluationError;
	type SchedulerError;
	/// A task generates its own id. This Id is used as a task id in the scheduler and also to check onchain storage persistence.
	fn to_id(&self) -> Runtime::Hash;
	//A task will know how to check onchain storage persistence.
	fn is_persisted(id: &Runtime::Hash) -> bool;
	/// A call to persist state is expected after successfully processing a task.
	/// This does not mean that the task result was successful. A succesful task's result may be a failure that needs state persistance.
	fn persistence_call(
		&self,
		deadline: Runtime::BlockNumber,
		id: &Runtime::Hash,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>>;
	/// complete task verification flow.
	fn forward_task(
		&self,
		deadline: Runtime::BlockNumber,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		let id = self.to_id();
		if Self::is_persisted(&id) {
			return Err(TaskError::FinishedTask);
		}
		self.persistence_call(deadline, &id)
	}
}

pub trait TaskScheduler<BlockNumber, Hash, Task> {
	fn deadline() -> BlockNumber;
	fn is_scheduled(deadline: &BlockNumber, id: &Hash) -> bool;
	fn insert(deadline: &BlockNumber, id: &Hash, task: Task);
}
