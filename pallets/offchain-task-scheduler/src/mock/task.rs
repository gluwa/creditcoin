use crate::tasks::{error::TaskError, ForwardTask, TaskV2};
use crate::Config;
use core::cell::Cell;
use frame_system::pallet::Call as SystemCall;
use scale_info::TypeInfo;
use sp_runtime::codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::traits::Hash;
use std::thread_local;

thread_local! { static PERSISTED:Cell<bool> = Cell::new(false); }

pub(crate) fn is_persisted_replace(new: bool) -> bool {
	tracing::warn!("forcing {new} is_persisted!");
	PERSISTED.with(|cell| cell.replace(new))
}

#[derive(Debug, MaxEncodedLen, Encode, TypeInfo, Decode, Clone)]
/// The task's result depends on the variant.
pub enum MockTask<T> {
	Remark(T),
	Evaluation,
	Scheduler,
}

impl<T: Config, Nonce: Encode> ForwardTask<T> for MockTask<Nonce>
where
	T::TaskCall: From<SystemCall<T>>,
{
	type Call = T::TaskCall;
	type EvaluationError = ();
	type SchedulerError = ();
	fn forward_task(
		&self,
		deadline: T::BlockNumber,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		TaskV2::<T>::forward_task(self, deadline).map(|c| c.into())
	}
}

impl<Runtime: Config, Nonce: Encode> TaskV2<Runtime> for MockTask<Nonce> {
	type Call = SystemCall<Runtime>;
	type EvaluationError = ();
	type SchedulerError = ();

	fn to_id(&self) -> Runtime::Hash {
		Runtime::Hashing::hash(&self.encode())
	}
	//A MockTask is never written into storage. Check [frame_system::pallet::Call::remark]
	fn is_persisted(_id: &Runtime::Hash) -> bool {
		PERSISTED.with(|cell| cell.get())
	}
	fn persistence_call(
		&self,
		_deadline: Runtime::BlockNumber,
		_id: &Runtime::Hash,
	) -> Result<SystemCall<Runtime>, TaskError<(), ()>> {
		match self {
			MockTask::Remark(nonce) => {
				Ok(frame_system::pallet::Call::remark_with_event { remark: nonce.encode() })
			},
			MockTask::Evaluation => Err(TaskError::Evaluation(())),
			MockTask::Scheduler => Err(TaskError::Scheduler(())),
		}
	}
}
