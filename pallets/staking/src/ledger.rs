use super::pallet::BalanceOf;
use super::TypeInfo;
use crate::pallet::Config;
use frame_support::dispatch::{Decode, Encode, HasCompact, MaxEncodedLen, RuntimeDebug};
use frame_support::traits::{Defensive, Get};
use frame_support::{BoundedVec, RuntimeDebugNoBound};
use sp_arithmetic::traits::Saturating;
use sp_arithmetic::traits::Zero;
use sp_arithmetic::{Perquintill, Rounding};
use sp_staking::EraIndex;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub(super) struct UnlockChunk<Balance: HasCompact + MaxEncodedLen> {
	/// Amount of funds to be unlocked.
	#[codec(compact)]
	pub value: Balance,
	/// Era number at which point it'll be unlocked.
	#[codec(compact)]
	pub era: EraIndex,
}

#[derive(TypeInfo, RuntimeDebugNoBound, Encode, Decode, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct StakingLedger<T: Config> {
	/// The stash account whose balance is actually locked and at stake.
	pub(super) stash: T::AccountId,
	/// The total amount of the stash's balance that we are currently accounting for.
	/// It's just `active` plus all the `unlocking` balances.
	#[codec(compact)]
	pub(super) total: BalanceOf<T>,
	/// The total amount of the stash's balance that will be at stake in any forthcoming
	/// rounds.
	#[codec(compact)]
	pub(super) active: BalanceOf<T>,
	/// Any balance that is becoming free, which may eventually be transferred out of the stash
	/// (assuming it doesn't get slashed first). It is assumed that this will be treated as a first
	/// in, first out queue where the new (higher value) eras get pushed on the back.
	pub(super) unlocking: BoundedVec<UnlockChunk<BalanceOf<T>>, T::MaxUnlockingChunks>,
	/// List of eras for which the stakers behind a validator have claimed rewards. Only updated
	/// for validators.
	pub(super) claimed_rewards: BoundedVec<EraIndex, T::HistoryDepth>,
}

impl<T: Config> StakingLedger<T> {
	/// Initializes the default object using the given `validator`.
	#[cfg(test)]
	pub fn default_from(stash: T::AccountId) -> Self {
		Self {
			stash,
			total: Zero::zero(),
			active: Zero::zero(),
			unlocking: Default::default(),
			claimed_rewards: Default::default(),
		}
	}

	/// Remove entries from `unlocking` that are sufficiently old and reduce the
	/// total by the sum of their balances.
	pub(super) fn consolidate_unlocked(self, at_era: EraIndex) -> Self {
		let mut total = self.total;
		let unlocking: BoundedVec<_, _> = self
			.unlocking
			.into_iter()
			.filter(|chunk| {
				if chunk.era > at_era {
					true
				} else {
					total = total.saturating_sub(chunk.value);
					false
				}
			})
			.collect::<Vec<_>>()
			.try_into()
			.expect(
				"filtering items from a bounded vec always leaves length less than bounds. qed",
			);

		Self {
			stash: self.stash,
			total,
			active: self.active,
			unlocking,
			claimed_rewards: self.claimed_rewards,
		}
	}

	/// Re-bond funds that were scheduled for unlocking.
	///
	/// Returns the updated ledger, and the amount actually rebonded.
	pub(super) fn rebond(mut self, value: BalanceOf<T>) -> (Self, BalanceOf<T>) {
		let mut unlocking_balance = BalanceOf::<T>::zero();

		while let Some(last) = self.unlocking.last_mut() {
			if unlocking_balance + last.value <= value {
				unlocking_balance += last.value;
				self.active += last.value;
				self.unlocking.pop();
			} else {
				let diff = value - unlocking_balance;

				unlocking_balance += diff;
				self.active += diff;
				last.value -= diff;
			}

			if unlocking_balance >= value {
				break;
			}
		}

		(self, unlocking_balance)
	}

	/// Slash the staker for a given amount of balance.
	///
	/// This implements a proportional slashing system, whereby we set our preference to slash as
	/// such:
	///
	/// - If any unlocking chunks exist that are scheduled to be unlocked at `slash_era +
	///   bonding_duration` and onwards, the slash is divided equally between the active ledger and
	///   the unlocking chunks.
	/// - If no such chunks exist, then only the active balance is slashed.
	///
	/// Note that the above is only a *preference*. If for any reason the active ledger, with or
	/// without some portion of the unlocking chunks that are more justified to be slashed are not
	/// enough, then the slashing will continue and will consume as much of the active and unlocking
	/// chunks as needed.
	///
	/// This will never slash more than the given amount. If any of the chunks become dusted, the
	/// last chunk is slashed slightly less to compensate. Returns the amount of funds actually
	/// slashed.
	///
	/// `slash_era` is the era in which the slash (which is being enacted now) actually happened.
	///
	/// This calls `Config::OnStakerSlash::on_slash` with information as to how the slash was
	/// applied.
	pub fn _slash(
		&mut self,
		slash_amount: BalanceOf<T>,
		minimum_balance: BalanceOf<T>,
		slash_era: EraIndex,
	) -> BalanceOf<T> {
		if slash_amount.is_zero() {
			return Zero::zero();
		}

		use sp_runtime::PerThing as _;
		use sp_staking::OnStakerSlash as _;
		let mut remaining_slash = slash_amount;
		let pre_slash_total = self.total;

		// for a `slash_era = x`, any chunk that is scheduled to be unlocked at era `x + 28`
		// (assuming 28 is the bonding duration) onwards should be slashed.
		let slashable_chunks_start = slash_era + T::BondingDuration::get();

		// `Some(ratio)` if this is proportional, with `ratio`, `None` otherwise. In both cases, we
		// slash first the active chunk, and then `slash_chunks_priority`.
		let (maybe_proportional, slash_chunks_priority) = {
			if let Some(first_slashable_index) =
				self.unlocking.iter().position(|c| c.era >= slashable_chunks_start)
			{
				// If there exists a chunk who's after the first_slashable_start, then this is a
				// proportional slash, because we want to slash active and these chunks
				// proportionally.

				// The indices of the first chunk after the slash up through the most recent chunk.
				// (The most recent chunk is at greatest from this era)
				let affected_indices = first_slashable_index..self.unlocking.len();
				let unbonding_affected_balance =
					affected_indices.clone().fold(BalanceOf::<T>::zero(), |sum, i| {
						if let Some(chunk) = self.unlocking.get(i).defensive() {
							sum.saturating_add(chunk.value)
						} else {
							sum
						}
					});
				let affected_balance = self.active.saturating_add(unbonding_affected_balance);
				let ratio = Perquintill::from_rational_with_rounding(
					slash_amount,
					affected_balance,
					Rounding::Up,
				)
				.unwrap_or_else(|_| Perquintill::one());
				(
					Some(ratio),
					affected_indices.chain((0..first_slashable_index).rev()).collect::<Vec<_>>(),
				)
			} else {
				// We just slash from the last chunk to the most recent one, if need be.
				(None, (0..self.unlocking.len()).rev().collect::<Vec<_>>())
			}
		};

		// Helper to update `target` and the ledgers total after accounting for slashing `target`.
		crate::logger!(
			debug,
			"slashing {:?} for era {:?} out of {:?}, priority: {:?}, proportional = {:?}",
			slash_amount,
			slash_era,
			self,
			slash_chunks_priority,
			maybe_proportional,
		);

		let mut slash_out_of = |target: &mut BalanceOf<T>, slash_remaining: &mut BalanceOf<T>| {
			let mut slash_from_target = if let Some(ratio) = maybe_proportional {
				ratio.mul_ceil(*target)
			} else {
				*slash_remaining
			}
			// this is the total that that the slash target has. We can't slash more than
			// this anyhow!
			.min(*target)
			// this is the total amount that we would have wanted to slash
			// non-proportionally, a proportional slash should never exceed this either!
			.min(*slash_remaining);

			// slash out from *target exactly `slash_from_target`.
			*target -= slash_from_target;
			if *target < minimum_balance {
				// Slash the rest of the target if it's dust. This might cause the last chunk to be
				// slightly under-slashed, by at most `MaxUnlockingChunks * ED`, which is not a big
				// deal.
				slash_from_target =
					sp_std::mem::replace(target, Zero::zero()).saturating_add(slash_from_target)
			}

			self.total = self.total.saturating_sub(slash_from_target);
			*slash_remaining = slash_remaining.saturating_sub(slash_from_target);
		};

		// If this is *not* a proportional slash, the active will always wiped to 0.
		slash_out_of(&mut self.active, &mut remaining_slash);

		let mut slashed_unlocking = BTreeMap::<_, _>::new();
		for i in slash_chunks_priority {
			if remaining_slash.is_zero() {
				break;
			}

			if let Some(chunk) = self.unlocking.get_mut(i).defensive() {
				slash_out_of(&mut chunk.value, &mut remaining_slash);
				// write the new slashed value of this chunk to the map.
				slashed_unlocking.insert(chunk.era, chunk.value);
			} else {
				break;
			}
		}

		// clean unlocking chunks that are set to zero.
		self.unlocking.retain(|c| !c.value.is_zero());

		T::OnStakerSlash::on_slash(&self.stash, self.active, &slashed_unlocking);
		pre_slash_total.saturating_sub(self.total)
	}
}

#[cfg(test)]
pub mod tests {
	use super::*;
	use crate::mock::runtime::Runtime;
	use runtime_utils::ExtBuilder;

	#[test]
	fn total_stake_deducted_after_unlocking_chunks() {
		ExtBuilder::<()>::default().build_sans_config().execute_with(|| {});
		let stash = runtime_utils::generate_account("//stash");
		let mut ledger = StakingLedger::<Runtime>::default_from(stash);
		ledger.active += 500;
		ledger.total = ledger.active;
		let current_era = 1;
		ledger.unlocking =
			[UnlockChunk { era: 0, value: 500 }].to_vec().try_into().expect("short enough");
		assert_eq!(ledger.total, 500);

		ledger = ledger.consolidate_unlocked(current_era);

		assert!(ledger.unlocking.is_empty());
		assert_eq!(ledger.total, 0);
	}
}
