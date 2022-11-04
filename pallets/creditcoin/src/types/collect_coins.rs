use super::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedCoins<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	pub tx_id: ExternalTxId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnverifiedCollectedCoins {
	pub to: ExternalAddress,
	pub tx_id: ExternalTxId,
	pub contract: GCreContract,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedCoinsId<Hash>(Hash);

impl<H> CollectedCoinsId<H> {
	pub fn new<Config>(contract_chain: &Blockchain, blockchain_tx_id: &[u8]) -> CollectedCoinsId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(contract_chain.as_bytes().iter(), blockchain_tx_id);
		CollectedCoinsId(Config::Hashing::hash(&key))
	}
}

impl<H> From<H> for CollectedCoinsId<H> {
	fn from(hash: H) -> Self {
		Self(hash)
	}
}

use crate::ocw::errors::SchedulerError;
use crate::ocw::tasks::OffchainVerification;
use crate::ocw::VerificationFailureCause;
use crate::ocw::VerificationResult;
use crate::types::concatenate;
use crate::Config;
use crate::TaskOutput;
use pallet_offchain_task_scheduler::tasks::error::TaskError;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use sp_runtime::traits::Hash;

impl<T: Config> OffchainVerification<T> for UnverifiedCollectedCoins {
	type Output = T::Balance;

	fn verify(&self) -> VerificationResult<Self::Output> {
		crate::Pallet::<T>::verify_collect_coins_ocw(self)
	}
}

impl<T: Config> TaskV2<T> for UnverifiedCollectedCoins
where
	UnverifiedCollectedCoins: OffchainVerification<T>,
	<UnverifiedCollectedCoins as OffchainVerification<T>>::Output: Into<T::Balance>,
{
	type Call = crate::pallet::Call<T>;
	type EvaluationError = VerificationFailureCause;
	type SchedulerError = SchedulerError;
	fn to_id(&self) -> T::Hash {
		let key = concatenate!(self.contract.chain.as_bytes().as_ref(), self.tx_id.as_slice());
		T::Hashing::hash(&key)
	}

	fn persistence_call(
		&self,
		deadline: T::BlockNumber,
		id: &T::Hash,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		use crate::ocw::OffchainError::*;
		match self.verify() {
			Ok(amount) => {
				let coins = self.clone().into_output::<T>(amount.into());
				let id = CollectedCoinsId::from(*id);
				Ok(Self::Call::persist_task_output {
					deadline,
					task_output: TaskOutput::from((id, coins)),
				})
			},
			Err(InvalidTask(cause)) if cause.is_fatal() => {
				log::warn!("Failed to verify pending task {:?} : {:?}", self, cause);
				let id = CollectedCoinsId::from(*id);
				Ok(Self::Call::fail_task { deadline, task_id: id.into(), cause })
			},
			Err(InvalidTask(e)) => Err(TaskError::Evaluation(e)),
			Err(NoRpcUrl(e)) => Err(TaskError::Scheduler(e.into())),
			Err(RpcError(e)) => Err(TaskError::Scheduler(e.into())),
			Err(IncorrectChainId) => Err(TaskError::Scheduler(SchedulerError::IncorrectChainId)),
		}
	}

	fn is_persisted(id: &T::Hash) -> bool {
		let id = CollectedCoinsId::from(*id);
		crate::pallet::CollectedCoins::<T>::contains_key(&id)
	}
}
