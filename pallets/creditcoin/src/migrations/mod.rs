use crate::{Config, Pallet};
use frame_support::{traits::StorageVersion, weights::Weight};

mod v1;
mod v2;
mod v3;
mod v4;
mod v5;
mod v6;

#[cfg(feature = "try-runtime")]
pub(crate) fn pre_upgrade<T: Config>(version: StorageVersion) -> Result<(), &'static str> {
	// potentially do some checks on the state before the migration
	if version < 6 {
		v6::pre_upgrade::<T>()?;
	}
	Ok(())
}

#[cfg(feature = "try-runtime")]
pub(crate) fn post_upgrade<T: Config>(version: StorageVersion) -> Result<(), &'static str> {
	// do some post-migration checks
	if version < 6 {
		v6::post_upgrade::<T>()?;
	}
	Ok(())
}

pub(crate) fn migrate<T: Config>() -> Weight {
	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = Weight::zero();

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

	if version < 6 {
		weight = weight.saturating_add(v6::migrate::<T>());
		StorageVersion::new(6).put::<Pallet<T>>();
	}

	weight
}

type HashOf<T> = <T as frame_system::Config>::Hash;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
