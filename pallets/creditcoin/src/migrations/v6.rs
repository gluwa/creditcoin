use super::{vec, Vec};
use super::{AccountIdOf, BlockNumberOf, HashOf, Migrate, MomentOf, PhantomData};
use crate::ocw::errors::SchedulerError;
use crate::ocw::tasks::collect_coins::DeployedContract;
use crate::ocw::tasks::OffchainVerification;
use crate::ocw::{VerificationFailureCause, VerificationResult};
use crate::pallet::WeightInfo;
use crate::{CollectedCoinsId, StorageVersion, TaskId, UnverifiedTransfer};
use crate::{Config, ExternalAddress, ExternalTxId};
use frame_support::weights::Weight;
use frame_support::RuntimeDebug;
use frame_support::{storage_alias, Identity};
use pallet_offchain_task_scheduler::tasks::error::TaskError;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::UniqueSaturatedInto;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OldUnverifiedCollectedCoins {
	pub to: ExternalAddress,
	pub tx_id: ExternalTxId,
	pub contract: DeployedContract,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Task<AccountId, BlockNum, Hash, Moment> {
	VerifyTransfer(UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>),
}

impl<T: Config> TaskV2<T> for OldUnverifiedCollectedCoins
where
	OldUnverifiedCollectedCoins: OffchainVerification<T>,
	<OldUnverifiedCollectedCoins as OffchainVerification<T>>::Output: Into<T::Balance>,
{
	type Call = crate::pallet::Call<T>;
	type EvaluationError = VerificationFailureCause;
	type SchedulerError = SchedulerError;
	fn to_id(&self) -> T::Hash {
		CollectedCoinsId::inner_hash::<T::Hashing>(&self.contract.chain, self.tx_id.as_slice())
	}

	fn persistence_call(
		&self,
		_deadline: T::BlockNumber,
		_id: &T::Hash,
	) -> Result<Self::Call, TaskError<Self::EvaluationError, Self::SchedulerError>> {
		unreachable!("")
	}

	fn is_persisted(_id: &T::Hash) -> bool {
		unreachable!("")
	}
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>) -> Self {
		Task::VerifyTransfer(transfer)
	}
}

impl<T: Config> OffchainVerification<T> for OldUnverifiedCollectedCoins {
	type Output = T::Balance;

	fn verify(&self) -> VerificationResult<Self::Output> {
		unreachable!("")
	}
}

#[storage_alias]
pub type PendingTasks<T: Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Identity,
	BlockNumberOf<T>,
	Identity,
	TaskId<HashOf<T>>,
	Task<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

mod new {

	use super::*;

	#[storage_alias]
	pub type PendingTasks<T: Config> = StorageDoubleMap<
		TaskScheduler, // the prefix for the storage item, which is generally the name of the pallet that defines the storage. We use an identifier instead of a string here because that's what the macro expects.
		Identity,
		BlockNumberOf<T>,
		Identity,
		TaskId<HashOf<T>>,
		Task<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>, // the `Task` we copied into this migration
	>;
}

pub(crate) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime> Migration<Runtime> {
	pub(crate) fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn migrate(&self) -> Weight {
		let mut n = 0u32;
		for (i, (k1, _, v)) in PendingTasks::<T>::drain().enumerate() {
			n = i.unique_saturated_into();
			let id: TaskId<T::Hash> = match &v {
				Task::VerifyTransfer(pending) => TaskId::VerifyTransfer(
					crate::types::TransferId::from(TaskV2::<T>::to_id(pending)),
				),
			};
			new::PendingTasks::<T>::insert(k1, id, v);
		}
		crate::weights::WeightInfo::<T>::migration_v6(n)
	}
	fn post_upgrade(&self, _ctx: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			6,
			"expected storage version to be 6 after migrations complete"
		);
	}
}

#[cfg(test)]
pub mod tests {
	use super::Migrate;
	use super::*;
	use crate::mock::ExtBuilder;
	use crate::mock::Test;
	use crate::types::test::create_unverified_transfer;

	#[test]
	fn migrate_verify_transfer() {
		ExtBuilder::default().build_and_execute(|| {
			let pending = create_unverified_transfer();

			let id = TaskId::VerifyTransfer(crate::types::TransferId::from(TaskV2::<Test>::to_id(
				&pending,
			)));

			PendingTasks::<Test>::insert(1u64, id.clone(), Task::from(pending.clone()));

			super::Migration::<Test>::new().migrate();

			let migrated_pending = {
				let Task::VerifyTransfer(pending) = new::PendingTasks::<Test>::get(1, id).unwrap();
				pending
			};
			assert_eq!(pending, migrated_pending);
		});
	}
}
