use crate::{Config, Pallet};
use frame_support::{traits::StorageVersion, weights::Weight};

mod v1;
mod v2;
mod v3;
mod v4;
mod v5;

pub(crate) fn migrate<T: Config>() -> Weight {
	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 1 {
		weight = weight.saturating_add(v1::migrate::<T>());
		StorageVersion::new(1).put::<Pallet<T>>();
	}

	if version < 2 {
		weight = weight.saturating_add(v2::migrate::<T>());
		StorageVersion::new(2).put::<Pallet<T>>();
	}

	if version < 3 {
		weight = weight.saturating_add(v3::migrate::<T>());
		StorageVersion::new(3).put::<Pallet<T>>();
	}

	if version < 4 {
		weight = weight.saturating_add(v4::migrate::<T>());
		StorageVersion::new(4).put::<Pallet<T>>();
	}

	if version < 5 {
		weight = weight.saturating_add(v5::migrate::<T>());
		StorageVersion::new(5).put::<Pallet<T>>();
	}

	weight
}
