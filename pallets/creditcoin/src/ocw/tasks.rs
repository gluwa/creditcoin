pub mod collect_coins;
pub mod verify_transfer;

use crate::ocw::errors::{OffchainError, VerificationResult};
use crate::types::{
	CollectedCoins as CollectedCoinsT, Task, TaskOutput, Transfer, UnverifiedCollectedCoins,
	UnverifiedTransfer,
};
use crate::{CollectedCoinsId, Config, TaskData, TransferId};
use codec::Encode;
use collect_coins::GCreContract;
pub use sp_runtime::offchain::storage_lock::{BlockAndTime, Lockable, StorageLock};
use sp_runtime::traits::{UniqueSaturatedFrom, UniqueSaturatedInto};
use sp_std::vec::Vec;

/// Needed at a pallet level, either Task exclusive or per pallet.
#[inline]
pub(crate) fn storage_key<Id: Encode>(id: &Id) -> Vec<u8> {
	const TASK_GUARD: &[u8] = b"creditcoin/task/guard/";
	id.using_encoded(|encoded_id| TASK_GUARD.iter().chain(encoded_id).copied().collect())
}

impl<AccountId, BlockNum, Hash, Moment> UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>
where
	Moment: UniqueSaturatedInto<u64> + UniqueSaturatedFrom<u64>,
	BlockNum: UniqueSaturatedInto<u64>,
{
	pub fn verify_ocw<T>(&self) -> VerificationResult<Option<T::Moment>>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		crate::Pallet::<T>::verify_transfer_ocw(self)
	}

	pub fn into_output<T: Config>(
		self,
		timestamp: Option<T::Moment>,
	) -> Transfer<AccountId, BlockNum, Hash, Moment>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		Transfer { timestamp, ..self.transfer }
	}

	pub fn to_id<T: Config>(&self) -> TransferId<Hash>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		TransferId::new::<T>(&self.transfer.blockchain, &self.transfer.tx_id)
	}
}

impl UnverifiedCollectedCoins {
	pub fn verify_ocw<T>(&self) -> VerificationResult<T::Balance>
	where
		T: Config,
	{
		crate::Pallet::<T>::verify_collect_coins_ocw(self)
	}

	pub fn into_output<T>(self, amount: T::Balance) -> CollectedCoinsT<T::Hash, T::Balance>
	where
		T: Config,
	{
		let Self { to, tx_id, contract: GCreContract { chain, .. } } = self;
		let to = crate::AddressId::new::<T>(&chain, to.as_slice());
		CollectedCoinsT { amount, to, tx_id }
	}

	pub fn to_id<T>(&self) -> CollectedCoinsId<T::Hash>
	where
		T: Config,
	{
		CollectedCoinsId::new::<T>(&self.contract.chain, &self.tx_id)
	}
}

impl<AccountId, BlockNum, Hash, Moment> Task<AccountId, BlockNum, Hash, Moment>
where
	Moment: UniqueSaturatedInto<u64> + UniqueSaturatedFrom<u64>,
	BlockNum: UniqueSaturatedInto<u64>,
{
	pub fn verify_ocw<T>(
		self,
	) -> Result<TaskData<AccountId, T::Balance, BlockNum, Hash, Moment>, (Self, OffchainError)>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		match self {
			Task::VerifyTransfer(transfer) => match transfer.verify_ocw::<T>() {
				Ok(data) => Ok(TaskData::VerifyTransfer(transfer, data)),
				Err(e) => Err((transfer.into(), e)),
			},
			Task::CollectCoins(collect_coins) => match collect_coins.verify_ocw::<T>() {
				Ok(data) => Ok(TaskData::CollectCoins(collect_coins, data)),
				Err(e) => Err((collect_coins.into(), e)),
			},
		}
	}
}

impl<AccountId, Balance, BlockNum, Hash, Moment>
	TaskData<AccountId, Balance, BlockNum, Hash, Moment>
where
	Moment: UniqueSaturatedInto<u64> + UniqueSaturatedFrom<u64>,
	BlockNum: UniqueSaturatedInto<u64>,
{
	pub fn into_output<T: Config>(
		self,
	) -> TaskOutput<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>
	where
		T: Config<
			AccountId = AccountId,
			Balance = Balance,
			BlockNumber = BlockNum,
			Hash = Hash,
			Moment = Moment,
		>,
	{
		match self {
			TaskData::VerifyTransfer(transfer, data) => {
				let id = transfer.to_id::<T>();
				TaskOutput::VerifyTransfer(id, transfer.into_output::<T>(data))
			},
			TaskData::CollectCoins(collected_coins, data) => {
				let id = collected_coins.to_id::<T>();
				TaskOutput::CollectCoins(id, collected_coins.into_output::<T>(data))
			},
		}
	}
}

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
