use super::*;
use crate::ocw::errors::SchedulerError;
use crate::ocw::tasks::OffchainVerification;
use crate::types::concatenate;
use crate::Config;
use crate::TaskOutput;
use pallet_offchain_task_scheduler::tasks::error::TaskError;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use pallet_timestamp::Config as TimestampConfig;
use sp_runtime::traits::Hash;
use sp_runtime::traits::UniqueSaturatedFrom;
use sp_runtime::traits::UniqueSaturatedInto;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Transfer<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub kind: TransferKind,
	pub from: AddressId<Hash>,
	pub to: AddressId<Hash>,
	pub deal_order_id: DealOrderId<BlockNum, Hash>,
	pub amount: ExternalAmount,
	pub tx_id: ExternalTxId,
	pub block: BlockNum,
	pub is_processed: bool,
	pub account_id: AccountId,
	pub timestamp: Option<Moment>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> {
	pub transfer: Transfer<AccountId, BlockNum, Hash, Moment>,
	pub from_external: ExternalAddress,
	pub to_external: ExternalAddress,
	pub deadline: BlockNum,
	pub currency_to_check: CurrencyOrLegacyTransferKind,
}

impl<AccountId, BlockNum, Hash, Moment> UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>
where
	Moment: UniqueSaturatedInto<u64> + UniqueSaturatedFrom<u64>,
	BlockNum: UniqueSaturatedInto<u64>,
{
	pub fn into_output<T: Config>(
		self,
		timestamp: Option<T::Moment>,
	) -> Transfer<AccountId, BlockNum, Hash, Moment>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		Transfer { timestamp, ..self.transfer }
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransferId<Hash>(Hash);

impl<H> TransferId<H> {
	#[cfg(test)]
	pub fn leaked_inner_hash<C: SystemConfig>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> H
	where
		<C as SystemConfig>::Hashing: Hash<Output = H>,
	{
		Self::inner_hash::<C::Hashing>(blockchain, blockchain_tx_id)
	}

	fn inner_hash<Hasher>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> H
	where
		Hasher: Hash<Output = H>,
	{
		let key = concatenate!(&*blockchain.as_bytes(), blockchain_tx_id);
		<Hasher as Hash>::hash(&key)
	}

	pub fn new<C: SystemConfig>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> TransferId<H>
	where
		<C as SystemConfig>::Hashing: Hash<Output = H>,
	{
		let hash = Self::inner_hash::<C::Hashing>(blockchain, blockchain_tx_id);
		TransferId(hash)
	}
}

impl<H> From<H> for TransferId<H> {
	fn from(hash: H) -> Self {
		Self(hash)
	}
}

type UnverifiedTransferFor<T> = UnverifiedTransfer<
	<T as SystemConfig>::AccountId,
	<T as SystemConfig>::BlockNumber,
	<T as SystemConfig>::Hash,
	<T as TimestampConfig>::Moment,
>;

impl<T: Config> TaskV2<T> for UnverifiedTransferFor<T>
where
	UnverifiedTransferFor<T>: OffchainVerification<T>,
	<UnverifiedTransferFor<T> as OffchainVerification<T>>::Output: Into<Option<T::Moment>>,
{
	type Call = crate::pallet::Call<T>;
	type EvaluationError = VerificationFailureCause;
	type SchedulerError = SchedulerError;

	fn to_id(&self) -> T::Hash {
		let Transfer { blockchain, tx_id, .. } = &self.transfer;
		TransferId::inner_hash::<T::Hashing>(blockchain, tx_id)
	}

	fn persistence_call(
		&self,
		deadline: T::BlockNumber,
		id: &T::Hash,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		use crate::ocw::OffchainError::*;
		match self.verify() {
			Ok(timestamp) => {
				let transfer = self.clone().into_output::<T>(timestamp.into());
				let id = TransferId::from(*id);
				Ok(Self::Call::persist_task_output {
					deadline,
					task_output: TaskOutput::from((id, transfer)),
				})
			},
			Err(InvalidTask(cause)) if cause.is_fatal() => {
				log::warn!("Failed to verify pending task {:?} : {:?}", self, cause);
				let id = TransferId::from(*id);
				Ok(Self::Call::fail_task { deadline, task_id: id.into(), cause })
			},
			Err(InvalidTask(e)) => Err(TaskError::Evaluation(e)),
			Err(NoRpcUrl(e)) => Err(TaskError::Scheduler(e.into())),
			Err(RpcError(e)) => Err(TaskError::Scheduler(e.into())),
			Err(IncorrectChainId) => Err(TaskError::Scheduler(SchedulerError::IncorrectChainId)),
		}
	}

	fn is_persisted(id: &T::Hash) -> bool {
		let id = TransferId::from(*id);
		crate::pallet::Transfers::<T>::contains_key(&id)
	}
}

impl<T: Config> OffchainVerification<T> for UnverifiedTransferFor<T> {
	type Output = Option<T::Moment>;

	fn verify(&self) -> VerificationResult<Self::Output> {
		crate::Pallet::<T>::verify_transfer_ocw(self)
	}
}
