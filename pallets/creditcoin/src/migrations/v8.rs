use super::AccountIdOf;
use super::Config as CreditcoinConfig;
use super::Migrate;
use super::StorageVersion;
use super::Weight;
use super::WeightInfo;
use frame_support::storage::migration::move_storage_from_pallet;
use frame_support::storage_alias;
use frame_support::traits::PalletInfoAccess;
use frame_support::traits::StorageInstance;
use frame_support::Blake2_128Concat;
use sp_runtime::traits::SaturatedConversion;

pub static SCHEDULER_PREFIX: &str = "TaskScheduler";

use sp_std::marker::PhantomData;

#[storage_alias]
type Authorities<T: CreditcoinConfig> =
	StorageMap<crate::Pallet<T>, Blake2_128Concat, AccountIdOf<T>, ()>;

pub(crate) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime> Migration<Runtime> {
	pub(crate) fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: CreditcoinConfig> Migrate for Migration<T> {
	fn pre_upgrade(&self) {
		let count = Authorities::<T>::iter().count();
		assert!(count != 0, "Authorities not found during migration");
	}

	fn migrate(&self) -> Weight {
		let count: u32 = Authorities::<T>::iter().count().saturated_into();

		let creditcoin = <crate::Pallet<T> as PalletInfoAccess>::name();

		move_storage_from_pallet(
			Authorities_Storage_Instance::<T>::STORAGE_PREFIX.as_bytes(),
			creditcoin.as_bytes(),
			SCHEDULER_PREFIX.as_bytes(),
		);

		crate::weights::WeightInfo::<T>::migration_v8(count)
	}

	fn post_upgrade(&self) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			8,
			"expected storage version to be 8 after migrations complete"
		);

		let count = Authorities::<T>::iter().count();
		assert!(count == 0, "Authorities not fully migrated");
	}
}
