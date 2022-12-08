pub mod collect_coins;
pub mod verify_transfer;

use crate::ocw::errors::VerificationResult;
use crate::types::Task;
use crate::Config;
pub use sp_runtime::offchain::storage_lock::{BlockAndTime, Lockable, StorageLock};

pub(crate) trait OffchainVerification<T: Config> {
	type Output;
	fn verify(&self) -> VerificationResult<Self::Output>;
}

use crate::ocw::errors::SchedulerError;
use crate::ocw::VerificationFailureCause;
use pallet_offchain_task_scheduler::tasks::error::TaskError;
pub use pallet_offchain_task_scheduler::tasks::ForwardTask;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use pallet_offchain_task_scheduler::Config as TaskConfig;

impl<T: Config + TaskConfig> ForwardTask<T>
	for Task<T::AccountId, T::BlockNumber, T::Hash, T::Moment>
where
	<T as TaskConfig>::TaskCall: From<crate::pallet::Call<T>>,
{
	type Call = T::TaskCall;
	type EvaluationError = VerificationFailureCause;
	type SchedulerError = SchedulerError;
	fn forward_task(
		&self,
		deadline: T::BlockNumber,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		use Task::*;
		match self {
			VerifyTransfer(unverified) => {
				unverified.forward_task(deadline).map(|c: crate::pallet::Call<T>| c.into())
			},
			CollectCoins(unverified) => {
				unverified.forward_task(deadline).map(|c: crate::pallet::Call<T>| c.into())
			},
		}
	}
}

mod tests;
