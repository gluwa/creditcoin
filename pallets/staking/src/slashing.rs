use super::{
	pallet::{BalanceOf, NegativeImbalanceOf},
	Config, Pallet,
};
use crate::{EraIndex, Store};
use frame_support::{
	defensive,
	dispatch::{Decode, Encode, HasCompact, RuntimeDebug, TypeInfo},
	traits::{Currency, Defensive, Imbalance},
};
use sp_arithmetic::traits::{Saturating, Zero};
use sp_std::vec::Vec;

/// Clear slashing metadata for an obsolete era.
pub(crate) fn clear_era_metadata<T: Config>(obsolete_era: EraIndex) {
	if <Pallet<T> as Store>::ValidatorSlashInEra::clear_prefix(obsolete_era, u32::MAX, None)
		.maybe_cursor
		.is_some()
	{
		defensive!("Fully clear old era slashes");
	}
}

// apply the slash to a stash account, deducting any missing funds from the reward
// payout, saturating at 0. this is mildly unfair but also an edge-case that
// can only occur when overlapping locked funds have been slashed.
pub fn do_slash<T: Config>(
	stash: &T::AccountId,
	value: BalanceOf<T>,
	reward_payout: &mut BalanceOf<T>,
	slashed_imbalance: &mut NegativeImbalanceOf<T>,
	slash_era: EraIndex,
) {
	let Some(controller) = <Pallet<T>>::bonded(stash).defensive() else {
        return
	};

	let Some(mut ledger) = <Pallet<T>>::ledger(&controller) else{
        return
	};

	let value = ledger.slash(value, T::Currency::minimum_balance(), slash_era);

	if !value.is_zero() {
		let (imbalance, missing) = T::Currency::slash(stash, value);
		slashed_imbalance.subsume(imbalance);

		if !missing.is_zero() {
			// deduct overslash from the reward payout
			*reward_payout = reward_payout.saturating_sub(missing);
		}

		<Pallet<T>>::update_ledger(&controller, &ledger);

		// trigger the event
		<Pallet<T>>::deposit_event(super::Event::<T>::Slashed {
			staker: stash.clone(),
			amount: value,
		});
	}
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct UnappliedSlash<AccountId, Balance: HasCompact> {
	/// The stash ID of the offending validator.
	validator: AccountId,
	/// The validator's own slash.
	own: Balance,
	/// Reporters of the offence; bounty payout recipients.
	reporters: Vec<AccountId>,
	/// The amount of payout.
	payout: Balance,
}

/// Apply a previously-unapplied slash.
pub(crate) fn apply_slash<T: Config>(
	unapplied_slash: UnappliedSlash<T::AccountId, BalanceOf<T>>,
	slash_era: EraIndex,
) {
	let mut slashed_imbalance = NegativeImbalanceOf::<T>::zero();
	let mut reward_payout = unapplied_slash.payout;

	do_slash::<T>(
		&unapplied_slash.validator,
		unapplied_slash.own,
		&mut reward_payout,
		&mut slashed_imbalance,
		slash_era,
	);

	pay_disputers::<T>(reward_payout, slashed_imbalance, &unapplied_slash.reporters);
}

/// Apply a reward payout to some reporters, paying the rewards out of the slashed imbalance.
fn pay_disputers<T: Config>(
	_reward_payout: BalanceOf<T>,
	_slashed_imbalance: NegativeImbalanceOf<T>,
	_reporters: &[T::AccountId],
) {
}
