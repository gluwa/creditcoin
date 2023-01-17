use crate::{Config, Pallet};
use core::marker::PhantomData;
use frame_support::{traits::StorageVersion, weights::Weight};
use sp_runtime::traits::UniqueSaturatedInto;

pub(crate) trait Migrate {
	fn pre_upgrade(&self);
	fn migrate(&self) -> Weight;
	fn post_upgrade(&self);
}

mod v1;
mod v2;
mod v3;
mod v4;
mod v5;
mod v6;
mod v7;

pub(crate) fn migrate<T: Config>() -> Weight {
	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = Weight::zero();

	let callbacks: &[&dyn Migrate] = &[
		&v1::Migration::<T>::new(),
		&v2::Migration::<T>::new(),
		&v3::Migration::<T>::new(),
		&v4::Migration::<T>::new(),
		&v5::Migration::<T>::new(),
		&v6::Migration::<T>::new(),
		&v7::Migration::<T>::new(),
	];

	for (idx, &calls) in callbacks.iter().enumerate() {
		let migration_idx = (idx + 1).unique_saturated_into();
		if version < migration_idx {
			#[cfg(feature = "try-runtime")]
			calls.pre_upgrade();
			weight.saturating_accrue(calls.migrate());
			StorageVersion::new(migration_idx).put::<Pallet<T>>();
			#[cfg(feature = "try-runtime")]
			calls.post_upgrade();
		}
	}

	weight
}

type HashOf<T> = <T as frame_system::Config>::Hash;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
