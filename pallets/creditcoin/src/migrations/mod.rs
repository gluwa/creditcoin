use crate::{Config, Pallet};
use frame_support::{traits::StorageVersion, weights::Weight};

mod v1;

pub(crate) fn migrate<T: Config>() -> Weight {
	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 1 {
		weight = weight.saturating_add(v1::migrate::<T>());
		StorageVersion::new(1).put::<Pallet<T>>();
	}

	weight
}
