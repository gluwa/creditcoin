use super::{vec, Vec};
use super::{AccountIdOf, BlockNumberOf, HashOf, Migrate, MomentOf, PhantomData};

use crate::types::CollectedCoinsStruct;

use crate::{AddressId, Config, TaskId, UnverifiedTransfer};
use crate::{CollectedCoinsId, ExternalTxId};
use frame_support::storage_alias;
use frame_support::weights::Weight;
use frame_support::{pallet_prelude::*, traits::Get};
use parity_scale_codec::{Decode, Encode};

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OldTask<AccountId, BlockNum, Hash, Moment> {
	VerifyTransfer(UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>),
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>>
	for OldTask<AccountId, BlockNum, Hash, Moment>
{
	fn from(transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>) -> Self {
		OldTask::VerifyTransfer(transfer)
	}
}

#[derive(Clone, Encode, Decode)]
pub struct OldCollectedCoinsStruct<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	pub tx_id: ExternalTxId,
}

pub(super) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime: Config> Migration<Runtime> {
	pub(super) fn new() -> Self {
		Self(PhantomData)
	}
}

#[storage_alias]
pub type PendingTasks<T: Config> = StorageDoubleMap<
	TaskScheduler,
	Identity,
	BlockNumberOf<T>,
	Identity,
	TaskId<HashOf<T>>,
	OldTask<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

mod new {

	use crate::Task;

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

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn post_upgrade(&self, _blob: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			8,
			"expected storage version to be 8 after migrations complete"
		);
	}
	fn migrate(&self) -> Weight {
		let mut weight: Weight = Weight::zero();
		let weight_each = T::DbWeight::get().reads_writes(1, 1);

		crate::CollectedCoins::<T>::translate::<OldCollectedCoinsStruct<T::Hash, T::Balance>, _>(
			|_k: CollectedCoinsId<T::Hash>, y: OldCollectedCoinsStruct<T::Hash, T::Balance>| {
				weight = weight.saturating_add(weight_each);

				Some(CollectedCoinsStruct {
					to: y.to,
					amount: y.amount,
					tx_id: y.tx_id,
					contract_type: crate::types::collect_coins::ContractType::GCRE,
				})
			},
		);

		new::PendingTasks::<T>::translate(
			|_: BlockNumberOf<T>,
			 _: TaskId<HashOf<T>>,
			 z: OldTask<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>| {
				weight = weight.saturating_add(weight_each);

				match z {
					OldTask::VerifyTransfer(pending) => {
						let new = UnverifiedTransfer {
							transfer: pending.transfer,
							from_external: pending.from_external,
							to_external: pending.to_external,
							deadline: pending.deadline,
						};

						Some(crate::types::Task::VerifyTransfer(new))
					},
				}
			},
		);

		weight
	}
}

#[cfg(test)]
mod tests {
	use pallet_offchain_task_scheduler::tasks::TaskV2;

	use super::*;
	use crate::{
		mock::{self, ExtBuilder, Test},
		types::{collect_coins::ContractType, test::create_unverified_transfer},
		Task,
	};

	#[frame_support::storage_alias]
	type CollectedCoins<T: crate::Config> = StorageMap<
		crate::Pallet<T>,
		Identity,
		CollectedCoinsId<<mock::Test as frame_system::Config>::Hash>,
		OldCollectedCoinsStruct<
			<mock::Test as frame_system::Config>::Hash,
			<mock::Test as pallet_balances::Config>::Balance,
		>,
	>;

	type OldCollectedCoinsStorage = CollectedCoins<Test>;

	#[test]
	fn test_migrate_collected_coins_struct() {
		ExtBuilder::default().build_and_execute(|| {
			let tx_id = BoundedVec::default();

			let storage_id = crate::CollectedCoinsId::new::<crate::mock::Test>(
				&crate::Blockchain::Ethereum,
				&tx_id,
			);

			let address: [u8; 20] = [1; 20];

			let to = AddressId::new::<Test>(&crate::Blockchain::Ethereum, &address);
			let old = OldCollectedCoinsStruct { to, amount: 100, tx_id };

			OldCollectedCoinsStorage::insert(&storage_id, &old);

			super::Migration::<Test>::new().migrate();

			let new = crate::CollectedCoins::<Test>::try_get(&storage_id).unwrap();

			assert_eq!(old.to, new.to);
			assert_eq!(old.amount, new.amount);
			assert_eq!(old.tx_id, new.tx_id);
			assert_eq!(new.contract_type, ContractType::GCRE);
		})
	}

	#[test]
	fn migrate_verify_transfer() {
		ExtBuilder::default().build_and_execute(|| {
			let pending = create_unverified_transfer();

			let id = TaskId::VerifyTransfer(crate::types::TransferId::from(TaskV2::<Test>::to_id(
				&pending,
			)));

			PendingTasks::<Test>::insert(1u64, id.clone(), OldTask::from(pending.clone()));

			super::Migration::<Test>::new().migrate();

			let Task::VerifyTransfer(migrated_pending) =
				new::PendingTasks::<Test>::get(1, id).unwrap();

			assert_eq!(pending, migrated_pending);
		});
	}
}
