use crate::pallet::WeightInfo;
use crate::types::Task;
use crate::Config;
use frame_support::weights::Weight;
use pallet_offchain_task_scheduler::tasks::TaskScheduler;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use sp_runtime::traits::UniqueSaturatedInto;

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut n = 0u32;
	for (i, (k1, _, v)) in crate::PendingTasks::<T>::drain().enumerate() {
		n = i.unique_saturated_into();
		let id: T::Hash = match &v {
			Task::CollectCoins(pending) => TaskV2::<T>::to_id(pending),
			Task::VerifyTransfer(pending) => TaskV2::<T>::to_id(pending),
		};

		T::TaskScheduler::insert(&k1, &id, v);
	}
	crate::weights::WeightInfo::<T>::migration_v7(n)
}

#[cfg(test)]
pub mod tests {
	use super::*;
	use crate::helpers::extensions::IntoBounded;
	use crate::mock::ExtBuilder;
	use crate::mock::Test;
	use crate::types;
	use crate::CollectedCoinsId;

	#[test]
	fn migrate_pending_tasks() {
		ExtBuilder::default().build_and_execute(|| {
			let pending = types::UnverifiedCollectedCoins {
				to: [0u8; 256].into_bounded(),
				tx_id: [0u8; 256].into_bounded(),
				contract: Default::default(),
			};
			let id = TaskV2::<Test>::to_id(&pending);

			crate::PendingTasks::<Test>::insert(
				1u64,
				crate::TaskId::from(CollectedCoinsId::from(id)),
				Task::from(pending.clone()),
			);

			migrate::<Test>();

			let migrated_pending = {
				if let Task::CollectCoins(pending) =
					pallet_offchain_task_scheduler::pallet::PendingTasks::<Test>::get(1, id)
						.unwrap()
				{
					pending
				} else {
					unreachable!()
				}
			};
			assert_eq!(pending, migrated_pending);
		});
	}
}
