use crate::{Config, Pallet};
use frame_support::{traits::StorageVersion, weights::Weight};

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

	if version < 1 {
		weight.saturating_accrue(v1::migrate::<T>());
		StorageVersion::new(1).put::<Pallet<T>>();
	}
	if version < 2 {
		weight.saturating_accrue(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}
	if version < 3 {
		weight.saturating_accrue(v3::migrate::<T>());
		StorageVersion::new(3).put::<Pallet<T>>();
	}
	if version < 4 {
		weight.saturating_accrue(v4::migrate::<T>());
		StorageVersion::new(4).put::<Pallet<T>>();
	}
	if version < 5 {
		weight.saturating_accrue(v5::migrate::<T>());
		StorageVersion::new(5).put::<Pallet<T>>();
	}
	if version < 6 {
		weight.saturating_accrue(v6::migrate::<T>());
		StorageVersion::new(6).put::<Pallet<T>>();
	}
	if version < 7 {
		weight.saturating_accrue(v7::migrate::<T>());
		StorageVersion::new(7).put::<Pallet<T>>();
	}

	weight
}

type HashOf<T> = <T as frame_system::Config>::Hash;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
