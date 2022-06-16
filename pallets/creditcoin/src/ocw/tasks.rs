pub mod collect_coins;
pub mod verify_transfer;

use crate::ocw::errors::{OffchainError, VerificationResult};
use crate::types::{
	CollectedCoins, Task, TaskOracleData, TaskOutput, Transfer, UnverifiedCollectedCoins,
	UnverifiedTransfer,
};
use crate::Config;
use sp_runtime::traits::{UniqueSaturatedFrom, UniqueSaturatedInto};

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
}

impl UnverifiedCollectedCoins {
	pub fn verify_ocw<T>(&self) -> VerificationResult<T::Balance>
	where
		T: Config,
	{
		crate::Pallet::<T>::verify_collect_coins_ocw(self).map(|c| c.amount)
	}

	pub fn into_output<T: Config>(self, amount: T::Balance) -> CollectedCoins<T::Hash, T::Balance>
	where
		T: Config,
	{
		let Self { to, tx_id } = self;
		let to = crate::AddressId::new::<T>(&collect_coins::CONTRACT_CHAIN, to.as_slice());
		CollectedCoins { amount, to, tx_id }
	}
}

impl<AccountId, BlockNum, Hash, Moment> Task<AccountId, BlockNum, Hash, Moment>
where
	Moment: UniqueSaturatedInto<u64> + UniqueSaturatedFrom<u64>,
	BlockNum: UniqueSaturatedInto<u64>,
{
	pub fn verify_ocw<T>(&self) -> VerificationResult<TaskOracleData<T::Balance, Moment>>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		match self {
			Task::VerifyTransfer(transfer) => {
				transfer.verify_ocw::<T>().map(|t| TaskOracleData::VerifyTransfer(t))
			},
			Task::CollectCoins(collect_coins) => {
				collect_coins.verify_ocw::<T>().map(|a| TaskOracleData::CollectCoins(a))
			},
		}
	}

	pub fn into_output<T: Config>(
		self,
		data: TaskOracleData<T::Balance, T::Moment>,
	) -> Result<
		TaskOutput<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
		OffchainError,
	>
	where
		T: Config<AccountId = AccountId, BlockNumber = BlockNum, Hash = Hash, Moment = Moment>,
	{
		match (self, data) {
			(Task::VerifyTransfer(transfer), TaskOracleData::VerifyTransfer(data)) => {
				let transfer = transfer.into_output::<T>(data);
				Ok(TaskOutput::VerifyTransfer(transfer))
			},
			(Task::CollectCoins(collect_coins), TaskOracleData::CollectCoins(data)) => {
				let collect_coins = collect_coins.into_output::<T>(data);
				Ok(TaskOutput::CollectCoins(collect_coins))
			},
			_ => Err(OffchainError::InvalidData),
		}
	}
}
