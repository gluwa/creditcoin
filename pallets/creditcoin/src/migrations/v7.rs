use super::Vec;
use super::WeightInfo;
use super::{AccountIdOf, Config};
use super::{Migrate, StorageVersion, Weight};
use crate::Pallet as TaskScheduler;
use frame_support::{
	storage::migration::move_storage_from_pallet,
	storage_alias,
	traits::{GetStorageVersion, PalletInfoAccess},
	Blake2_128Concat, StoragePrefixedMap,
};
use sp_io::hashing::twox_128;
use sp_runtime::traits::SaturatedConversion;
use sp_std::{marker::PhantomData, vec};

pub static SCHEDULER_PREFIX: &str = "TaskScheduler";

#[storage_alias]
pub type Authorities<T: Config> =
	StorageMap<crate::Pallet<T>, Blake2_128Concat, AccountIdOf<T>, ()>;

pub(crate) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime> Migration<Runtime> {
	pub(crate) fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		let count = Authorities::<T>::iter().count();

		assert!(count != 0, "Authorities not found during migration");

		let old_pallet = TaskScheduler::<T>::name();
		let new_pallet = SCHEDULER_PREFIX;

		if old_pallet == new_pallet {
			log::info!(
				target: "runtime::Creditcoin",
				"pre-migrate V7, nothing to do.",
			);
			return vec![];
		}

		let storage_prefix = Authorities::<T>::storage_prefix();

		let new_pallet_prefix = twox_128(new_pallet.as_bytes());
		let authorities_prefix = [&new_pallet_prefix, &twox_128(storage_prefix)[..]].concat();

		let new_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			authorities_prefix.clone(),
			authorities_prefix,
			|key| Ok(key.to_vec()),
		);

		assert!(
			new_pallet_prefix_iter.count() == 0,
			"Expected new authorities storage to be empty"
		);

		assert!(<crate::Pallet<T> as GetStorageVersion>::on_chain_storage_version() < 8);

		count.to_le_bytes().to_vec()
	}

	fn migrate(&self) -> Weight {
		let count: u32 = Authorities::<T>::iter().count().saturated_into();

		let creditcoin = TaskScheduler::<T>::name();

		move_storage_from_pallet(
			Authorities::<T>::storage_prefix(),
			creditcoin.as_bytes(),
			SCHEDULER_PREFIX.as_bytes(),
		);

		crate::weights::WeightInfo::<T>::migration_v7(count)
	}

	fn post_upgrade(&self, ctx: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			7,
			"expected storage version to be 7 after migrations complete"
		);

		let new_pallet = SCHEDULER_PREFIX;
		let new_pallet_prefix = twox_128(new_pallet.as_bytes());
		let new_pallet_prefix_iter = frame_support::storage::KeyPrefixIterator::new(
			new_pallet_prefix.to_vec(),
			new_pallet_prefix.to_vec(),
			|key| Ok(key.to_vec()),
		);

		let past_count = usize::from_le_bytes(ctx.try_into().unwrap());

		assert_eq!(new_pallet_prefix_iter.count(), past_count);
	}
}

#[cfg(test)]
pub mod tests {
	use super::Authorities as OldAuthorities;
	use super::Migrate;
	use crate::mock::AccountId;
	use crate::mock::ExtBuilder;
	use crate::mock::Test;
	use pallet_offchain_task_scheduler::Authorities as NewAuthorities;

	#[test]
	fn migrate_authorities() {
		ExtBuilder::default().build_and_execute(|| {
			let auth1: AccountId = AccountId::new([11; 32]);
			OldAuthorities::<Test>::insert(auth1.clone(), ());

			let auth2: AccountId = AccountId::new([22; 32]);
			OldAuthorities::<Test>::insert(auth2.clone(), ());

			let previous_count = OldAuthorities::<Test>::iter().count();

			super::Migration::<Test>::new().migrate();

			assert_eq!(0, OldAuthorities::<Test>::iter().count());
			assert_eq!(previous_count, NewAuthorities::<Test>::iter().count());
			assert!(NewAuthorities::<Test>::contains_key(auth1));
			assert!(NewAuthorities::<Test>::contains_key(auth2));
		});
	}
}
