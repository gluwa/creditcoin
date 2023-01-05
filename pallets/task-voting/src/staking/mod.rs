use frame_support::dispatch::{
	Decode, DispatchError, DispatchResult, Encode, MaxEncodedLen, RuntimeDebug,
};
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

/// A struct that reflects stake that an account has in the staking system. Provides a set of
/// methods to operate on it's properties. Aimed at making `StakingInterface` more concise.
pub struct Stake<T: StakingInterface + ?Sized> {
	/// The stash account whose balance is actually locked and at stake.
	pub stash: T::AccountId,
	/// The total stake that `stash` has in the staking system. This includes the
	/// `active` stake, and any funds currently in the process of unbonding via
	/// [`StakingInterface::unbond`].
	///
	/// # Note
	///
	/// This is only guaranteed to reflect the amount locked by the staking system. If there are
	/// non-staking locks on the bonded pair's balance this amount is going to be larger in
	/// reality.
	pub total: T::Balance,
	/// The total amount of the stash's balance that will be at stake in any forthcoming
	/// rounds.
	pub active: T::Balance,
}

/// A generic representation of a staking implementation.
pub trait StakingInterface {
	/// Balance type used by the staking system.
	type Balance: PartialEq;

	/// AccountId type used by the staking system
	type AccountId;

	/// The minimum amount required to bond in order to set validation intentions.
	fn minimum_staking_bond() -> Self::Balance;

	/// Number of eras that staked funds must remain bonded for.
	fn bonding_duration() -> EraIndex;

	/// Returns the stake of `who`.
	fn stake(who: &Self::AccountId) -> Result<Stake<Self>, DispatchError>;

	fn total_stake(who: &Self::AccountId) -> Result<Self::Balance, DispatchError> {
		Self::stake(who).map(|s| s.total)
	}

	fn active_stake(who: &Self::AccountId) -> Result<Self::Balance, DispatchError> {
		Self::stake(who).map(|s| s.active)
	}

	fn is_unbonding(who: &Self::AccountId) -> Result<bool, DispatchError> {
		Self::stake(who).map(|s| s.active != s.total)
	}

	fn fully_unbond(who: &Self::AccountId) -> DispatchResult {
		Self::unbond(who, Self::stake(who)?.active)
	}

	/// Bond (lock) `value` of `who`'s balance, while forwarding any rewards to `payee`.
	fn bond(who: &Self::AccountId, value: Self::Balance, payee: &Self::AccountId)
		-> DispatchResult;

	/// Bond some extra amount in `who`'s free balance against the active bonded balance of
	/// the account. The amount extra actually bonded will never be more than `who`'s free
	/// balance.
	fn bond_extra(who: &Self::AccountId, extra: Self::Balance) -> DispatchResult;

	/// Schedule a portion of the active bonded balance to be unlocked at era
	/// [Self::current_era] + [`Self::bonding_duration`].
	///
	/// Once the unlock era has been reached, [`Self::withdraw_unbonded`] can be called to unlock
	/// the funds.
	///
	/// The amount of times this can be successfully called is limited based on how many distinct
	/// eras funds are schedule to unlock in. Calling [`Self::withdraw_unbonded`] after some unlock
	/// schedules have reached their unlocking era should allow more calls to this function.
	fn unbond(stash: &Self::AccountId, value: Self::Balance) -> DispatchResult;

	/// Unlock any funds schedule to unlock before or at the current era.
	///
	/// Returns whether the stash was killed because of this withdraw or not.
	fn withdraw_unbonded(
		stash: Self::AccountId,
		num_slashing_spans: u32,
	) -> Result<bool, DispatchError>;

	/// Force a current staker to become completely unstaked, immediately.
	fn force_unstake(who: Self::AccountId) -> DispatchResult;
}
