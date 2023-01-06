use super::pallet::{
	ActiveEra, BondedEras, Config, ErasValidatorReward, Event, OffendingValidators, Pallet,
};
use crate::slashing;
use crate::ActiveEraInfo;
use frame_support::defensive;
use frame_support::traits::{Currency, Defensive, DefensiveOption, Get, UnixTime};
use pallet_staking_substrate::EraPayout;
use sp_arithmetic::traits::SaturatedConversion;
use sp_arithmetic::traits::Saturating;

pub trait EraInterface<BlockNumber> {
	fn start_era(block_number: BlockNumber);
	fn end_active_era();
	//Use it to set ActiveEra.start
	fn set_era_start_time();
	fn next_era_start() -> BlockNumber;
}

impl<T: Config> EraInterface<T::BlockNumber> for Pallet<T> {
	/// Start a new era. It does:
	///
	/// * Increment `active_era.index`,
	/// * reset `active_era.start`,
	/// * update `BondedEras` and apply slashes.
	fn start_era(block_number: T::BlockNumber) {
		let active_era = ActiveEra::<T>::mutate(|active_era| {
			let new_index = active_era.as_ref().map(|info| info.index + 1).unwrap_or(0);
			*active_era = Some(ActiveEraInfo {
				index: new_index,
				// Set new active era start in next `on_finalize`. To guarantee usage of `Time`
				start: None,
			});
			new_index
		});

		let bonding_duration = T::BondingDuration::get();

		BondedEras::<T>::mutate(|bonded| {
			bonded.push((active_era, block_number));

			if active_era > bonding_duration {
				let first_kept = active_era - bonding_duration;

				// Prune out everything that's from before the first-kept index.
				let n_to_prune =
					bonded.iter().take_while(|&&(era_idx, _)| era_idx < first_kept).count();

				// Kill slashing metadata.
				for (pruned_era, _) in bonded.drain(..n_to_prune) {
					slashing::clear_era_metadata::<T>(pruned_era);
				}
			}
		});

		Self::apply_unapplied_slashes(active_era);
	}

	fn end_active_era() {
		let Some(era_info) = Self::active_era()else{
			defensive!("Active Era info not set");
			return
		};
		// Note: active_era_start can be None if end era is called during genesis config.
		let Some(active_era_start) = era_info.start else {
			defensive!("Era time start not set");
			return
		};

		let now_as_millis_u64 = T::UnixTime::now().as_millis().saturated_into::<u64>();

		let era_duration = (now_as_millis_u64 - active_era_start).saturated_into::<u64>();
		let staked = Self::eras_total_stake(era_info.index);
		let issuance = T::Currency::total_issuance();
		let (validator_payout, remainder) =
			T::EraPayout::era_payout(staked, issuance, era_duration);

		Self::deposit_event(Event::<T>::EraPaid {
			era_index: era_info.index,
			validator_payout,
			remainder,
		});

		// Set ending era reward.
		<ErasValidatorReward<T>>::insert(era_info.index, validator_payout);
		//T::RewardRemainder::on_unbalanced(T::Currency::issue(remainder));

		// Clear offending validators.
		<OffendingValidators<T>>::kill();
	}

	fn set_era_start_time() {
		ActiveEra::<T>::mutate(|era_info| {
			if let Some(info) = era_info {
				info.start = Some(T::UnixTime::now().as_millis().saturated_into::<u64>());
			}
		});
	}

	fn next_era_start() -> T::BlockNumber {
		let era_at = Self::active_era().defensive_map(|e| e.index).unwrap_or_default();
		Self::eras_start_session_index(era_at)
			.defensive_unwrap_or_default()
			.saturating_add(60u32.saturated_into())
	}
}
