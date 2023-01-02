use frame_support::dispatch::{Decode, Encode, MaxEncodedLen, RuntimeDebug};
use frame_support::traits::StorageVersion;
use scale_info::TypeInfo;
use sp_runtime::traits::Convert;
use sp_staking::EraIndex;

pub mod ledger;
pub mod pallet;

#[cfg(feature = "std")]
pub use pallet::GenesisConfig;
pub use pallet::{Config, Error, Event, Pallet};
pub use pallet::{
	__InherentHiddenInstance, __substrate_call_check, __substrate_event_check,
	__substrate_genesis_config_check, tt_default_parts, tt_error_token,
};

pub const _STORAGE_VERSION: StorageVersion = StorageVersion::new(0);
pub(crate) const LOG_TARGET: &str = "runtime::staking";

macro_rules! logger {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::staking::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

use logger;

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ActiveEraInfo {
	/// Index of era.
	pub index: EraIndex,
	/// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
	///
	/// Start can be none if start hasn't been set for the era yet,
	/// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
	start: Option<u64>,
}

/// A `Convert` implementation that finds the stash of the given controller account,
/// if any.
pub struct StashOf<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> Convert<T::AccountId, Option<T::AccountId>> for StashOf<T> {
	fn convert(controller: T::AccountId) -> Option<T::AccountId> {
		<Pallet<T>>::ledger(&controller).map(|l| l.stash)
	}
}
