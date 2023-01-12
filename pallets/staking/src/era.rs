use super::pallet::{
	ActiveEra, BondedEras, Config, ErasValidatorReward, Event, OffendingValidators, Pallet,
};
use crate::slashing;
use crate::ActiveEraInfo;
use frame_support::defensive;
use frame_support::traits::Defensive;
use frame_support::traits::{Currency, Get, UnixTime};
use pallet_staking_substrate::EraPayout;
use sp_arithmetic::traits::SaturatedConversion;
use sp_arithmetic::traits::Saturating;

pub trait EraInterface<BlockNumber> {
	fn start_era(block_number: BlockNumber);
	fn end_active_era();
	//Use it to set ActiveEra.start
	fn is_era_timestamped() -> bool;
	fn set_era_timestamp();
	fn next_era_start(current_session_index: BlockNumber) -> BlockNumber;
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
			let new_session_index = Self::next_era_start(block_number);

			crate::pallet::ErasStartSessionIndex::<T>::insert(&new_index, &new_session_index);

			*active_era = Some(ActiveEraInfo {
				index: new_index,
				// Set new active era start in next `on_finalize`. To guarantee usage of `Time`
				start: None,
			});

			// Clean old era information.
			if let Some(old_era) = new_index.checked_sub(T::HistoryDepth::get() + 1) {
				Self::clear_era_information(old_era);
			}

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

	fn is_era_timestamped() -> bool {
		ActiveEra::<T>::get()
			.defensive_proof("Missing Active Era")
			.and_then(|info| info.start)
			.is_some()
	}

	fn set_era_timestamp() {
		ActiveEra::<T>::mutate(|era_info| {
			if let Some(info) = era_info {
				info.start = Some(T::UnixTime::now().as_millis().saturated_into::<u64>());
			}
		});
	}

	/// Fetch the active era index. It will be missing the first time the pallet is initialized.
	/// Fetch the active era session (block number) start index. It will be missing the first time.
	/// If the active era session start is missing, return the current sesion to start a new era immediately.
	fn next_era_start(current_session_index: T::BlockNumber) -> T::BlockNumber {
		let era_at = Self::current_era();
		Self::eras_start_session_index(era_at)
			.defensive_proof("Missing era start session index")
			.map(|start| start.saturating_add(T::BlocksPerEra::get().saturated_into()))
			.unwrap_or(current_session_index)
	}
}

impl EraInterface<u32> for () {
	fn start_era(_block_number: u32) {}

	fn end_active_era() {}

	fn set_era_start_time() {}

	fn next_era_start(_: u32) -> u32 {
		defensive!("always 0");
		0
	}
}
