use crate::{Config, Pallet};
use frame_support::{traits::StorageVersion, weights::Weight};
use std::collections::HashMap;

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

	let callbacks = HashMap::from([
		(1, (v1::pre_upgrade::<T>, v1::migrate::<T>, v1::post_upgrade::<T>)),
		(2, (v2::pre_upgrade::<T>, v2::migrate::<T>, v2::post_upgrade::<T>)),
		(3, (v3::pre_upgrade::<T>, v3::migrate::<T>, v3::post_upgrade::<T>)),
		(4, (v4::pre_upgrade::<T>, v4::migrate::<T>, v4::post_upgrade::<T>)),
		(5, (v5::pre_upgrade::<T>, v5::migrate::<T>, v5::post_upgrade::<T>)),
		(6, (v6::pre_upgrade::<T>, v6::migrate::<T>, v6::post_upgrade::<T>)),
		(7, (v7::pre_upgrade::<T>, v7::migrate::<T>, v7::post_upgrade::<T>)),
	]);

	for migration_number in 1 + version.into()..=callbacks.keys().len() {
		let (pre_hook, migrate_hook, post_hook) = callbacks
			.get(migration_number)
			.expect("No callbacks found for version {version:?}");

		#[cfg(feature = "try-runtime")]
		pre_hook();

		weight.saturating_accrue(migrate_hook());
		StorageVersion::new(migration_number).put::<Pallet<T>>();

		#[cfg(feature = "try-runtime")]
		post_hook();
	}

	weight
}

type HashOf<T> = <T as frame_system::Config>::Hash;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
