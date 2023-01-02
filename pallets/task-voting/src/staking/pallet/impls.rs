use super::*;
use crate::staking::ledger::StakingLedger;
use crate::staking::EraIndex;
use frame_support::dispatch::DispatchResult;
use frame_support::dispatch::{
	DispatchError, DispatchResultWithPostInfo, Weight, WithPostDispatchInfo,
};
use frame_support::ensure;
use frame_support::traits::DefensiveResult;
use frame_support::traits::Get;
use frame_support::traits::{Currency, Imbalance, LockableCurrency, OnUnbalanced, WithdrawReasons};
use pallet_staking::RewardDestination;
use pallet_staking::WeightInfo;
use sp_arithmetic::traits::Zero;

use pallet::STAKING_ID;

impl<T: Config> Pallet<T> {
	/// Update the ledger for a controller.
	///
	/// This will also update the stash lock.
	pub(crate) fn update_ledger(controller: &T::AccountId, ledger: &StakingLedger<T>) {
		T::Currency::set_lock(STAKING_ID, &ledger.stash, ledger.total, WithdrawReasons::all());
		<Ledger<T>>::insert(controller, ledger);
	}

	pub(super) fn do_withdraw_unbonded(
		controller: &T::AccountId,
		num_slashing_spans: u32,
	) -> Result<Weight, DispatchError> {
		let mut ledger = Self::ledger(controller).ok_or(Error::<T>::NotController)?;
		let (stash, old_total) = (ledger.stash.clone(), ledger.total);
		if let Some(current_era) = Self::current_era() {
			ledger = ledger.consolidate_unlocked(current_era)
		}

		let used_weight =
			if ledger.unlocking.is_empty() && ledger.active < T::Currency::minimum_balance() {
				// This account must have called `unbond()` with some value that caused the active
				// portion to fall below existential deposit + will have no more unlocking chunks
				// left. We can now safely remove all staking-related information.
				Self::kill_stash(&stash, num_slashing_spans)?;
				// Remove the lock.
				T::Currency::remove_lock(STAKING_ID, &stash);

				T::WeightInfo::withdraw_unbonded_kill(num_slashing_spans)
			} else {
				// This was the consequence of a partial unbond. just update the ledger and move on.
				Self::update_ledger(controller, &ledger);

				// This is only an update, so we use less overall weight.
				T::WeightInfo::withdraw_unbonded_update(num_slashing_spans)
			};

		// `old_total` should never be less than the new total because
		// `consolidate_unlocked` strictly subtracts balance.
		if ledger.total < old_total {
			// Already checked that this won't overflow by entry condition.
			let value = old_total - ledger.total;
			Self::deposit_event(Event::<T>::Withdrawn { stash, amount: value });
		}

		Ok(used_weight)
	}

	/// Remove all associated data of a stash account from the staking system.
	///
	/// Assumes storage is upgraded before calling.
	///
	/// This is called:
	/// - after a `withdraw_unbonded()` call that frees all of a stash's bonded balance.
	/// - through `reap_stash()` if the balance has fallen to zero (through slashing).
	pub(crate) fn kill_stash(stash: &T::AccountId, _: u32) -> DispatchResult {
		let controller = <Bonded<T>>::get(stash).ok_or(Error::<T>::NotStash)?;

		Self::clear_stash_metadata(stash);

		<Bonded<T>>::remove(stash);
		<Ledger<T>>::remove(&controller);

		<Payee<T>>::remove(stash);

		frame_system::Pallet::<T>::dec_consumers(stash);

		Ok(())
	}

	/// Clear slashing metadata for a dead account.
	pub(crate) fn clear_stash_metadata(stash: &T::AccountId) {
		<Pallet<T> as Store>::SlashingSpans::remove(stash);
	}

	pub(super) fn do_payout_stakers(
		validator_stash: T::AccountId,
		era: EraIndex,
	) -> DispatchResultWithPostInfo {
		// Validate input data
		let current_era = CurrentEra::<T>::get().ok_or_else(|| {
			Error::<T>::InvalidEraToReward
				.with_weight(T::WeightInfo::payout_stakers_alive_staked(0))
		})?;
		let history_depth = T::HistoryDepth::get();
		ensure!(
			era <= current_era && era >= current_era.saturating_sub(history_depth),
			Error::<T>::InvalidEraToReward
				.with_weight(T::WeightInfo::payout_stakers_alive_staked(0))
		);

		// Note: if era has no reward to be claimed, era may be future. better not to update
		// `ledger.claimed_rewards` in this case.
		let _era_payout = <ErasValidatorReward<T>>::get(era).ok_or_else(|| {
			Error::<T>::InvalidEraToReward
				.with_weight(T::WeightInfo::payout_stakers_alive_staked(0))
		})?;

		let controller = Self::bonded(&validator_stash).ok_or_else(|| {
			Error::<T>::NotStash.with_weight(T::WeightInfo::payout_stakers_alive_staked(0))
		})?;
		let mut ledger = <Ledger<T>>::get(&controller).ok_or(Error::<T>::NotController)?;

		ledger
			.claimed_rewards
			.retain(|&x| x >= current_era.saturating_sub(history_depth));

		match ledger.claimed_rewards.binary_search(&era) {
			Ok(_) => {
				return Err(Error::<T>::AlreadyClaimed
					.with_weight(T::WeightInfo::payout_stakers_alive_staked(0)))
			},
			Err(pos) => ledger
				.claimed_rewards
				.try_insert(pos, era)
				// Since we retain era entries in `claimed_rewards` only upto
				// `HistoryDepth`, following bound is always expected to be
				// satisfied.
				.defensive_map_err(|_| Error::<T>::BoundNotMet)?,
		}

		// Input data seems good, no errors allowed after this point

		<Ledger<T>>::insert(&controller, &ledger);

		// Get Era reward points. It has TOTAL and INDIVIDUAL
		// Find the fraction of the era reward that belongs to the validator
		// Take that fraction of the eras rewards to split to nominator and validator
		//
		// Then look at the validator, figure out the proportion of their reward
		// which goes to them and each of their nominators.

		let era_reward_points = <ErasRewardPoints<T>>::get(era);
		let validator_reward_points = era_reward_points
			.individual
			.get(&ledger.stash)
			.copied()
			.unwrap_or_else(Zero::zero);

		// Nothing to do if they have no reward points.
		if validator_reward_points.is_zero() {
			return Ok(Some(T::WeightInfo::payout_stakers_alive_staked(0)).into());
		}

		Self::deposit_event(Event::<T>::PayoutStarted {
			era_index: era,
			validator_stash: ledger.stash.clone(),
		});

		let mut total_imbalance = PositiveImbalanceOf::<T>::zero();
		// We can now make total validator payout:
		//TODO
		if let Some(imbalance) = Self::make_payout(&ledger.stash, validator_reward_points.into()) {
			Self::deposit_event(Event::<T>::Rewarded {
				stash: ledger.stash,
				amount: imbalance.peek(),
			});
			total_imbalance.subsume(imbalance);
		}

		T::Reward::on_unbalanced(total_imbalance);
		Ok(Some(T::WeightInfo::payout_stakers_alive_staked(0u32)).into())
	}

	/// Actually make a payment to a staker. This uses the currency's reward function
	/// to pay the right payee for the given staker account.
	fn make_payout(stash: &T::AccountId, amount: BalanceOf<T>) -> Option<PositiveImbalanceOf<T>> {
		let dest = Self::payee(stash);
		match dest {
			RewardDestination::Controller => Self::bonded(stash)
				.map(|controller| T::Currency::deposit_creating(&controller, amount)),
			RewardDestination::Stash => T::Currency::deposit_into_existing(stash, amount).ok(),
			RewardDestination::Staked => Self::bonded(stash)
				.and_then(|c| Self::ledger(&c).map(|l| (c, l)))
				.and_then(|(controller, mut l)| {
					l.active += amount;
					l.total += amount;
					let r = T::Currency::deposit_into_existing(stash, amount).ok();
					Self::update_ledger(&controller, &l);
					r
				}),
			RewardDestination::Account(dest_account) => {
				Some(T::Currency::deposit_creating(&dest_account, amount))
			},
			RewardDestination::None => None,
		}
	}
}
