//use crate::mock::runtime::Call;
use crate::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_system::pallet::Call as SystemCall;
use sp_runtime::traits::Hash;

#[derive(Debug, MaxEncodedLen, Encode, TypeInfo, Decode)]
//#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct MockTask;

use crate::tasks::{error::TaskError, TaskV2, ForwardTask};

impl<T: Config> ForwardTask<T> for MockTask
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
		let task = MockTask{};
		TaskV2::<T>::forward_task(&task,deadline).map(|c| c.into())
	}
}

impl<Runtime: Config> TaskV2<Runtime> for MockTask {
	type Call = SystemCall<Runtime>;
	type EvaluationError = ();
	type SchedulerError = ();

	fn to_id(&self) -> Runtime::Hash {
		Runtime::Hashing::hash(&self.encode())
	}
	//A MockTask is never written into storage. Check [frame_system::pallet::Call::remark]
	fn is_persisted(_id: &Runtime::Hash) -> bool {
		false
	}
	fn persistence_call(
		&self,
		deadline: Runtime::BlockNumber,
		_id: &Runtime::Hash,
	) -> Result<SystemCall<Runtime>, TaskError<(), ()>> {
		Ok(frame_system::pallet::Call::remark { remark: deadline.encode() }.into())
	}
}
