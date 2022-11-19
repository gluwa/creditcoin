#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryFrom;
use frame_support::traits::Currency;
pub use pallet::*;
use sp_runtime::{traits::Saturating, FixedPointNumber};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;

#[allow(clippy::unnecessary_cast)]
pub mod weights;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub const REWARD_HALF_LIFE: u64 = 2_500_000;
pub const BASE_REWARD_IN_CTC: u64 = 28;
pub const CREDO_PER_CTC: u64 = 1_000_000_000_000_000_000;
pub const SAWTOOTH_PORT_HEIGHT: u64 = 1_123_966;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{UniqueSaturatedFrom, UniqueSaturatedInto},
		FixedU128,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config
	where
		<Self as frame_system::Config>::BlockNumber: UniqueSaturatedInto<u64>,
		BalanceOf<Self>: UniqueSaturatedFrom<u128>,
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<AccountIdOf<Self>>;

		type WeightInfo: WeightInfo;
	}

	pub trait WeightInfo {
		fn on_finalize() -> Weight;
		fn on_initialize() -> Weight;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn block_author)]
	pub type BlockAuthor<T> = StorageValue<_, AccountIdOf<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Reward was issued. [block_author, amount]
		RewardIssued(AccountIdOf<T>, BalanceOf<T>),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_now: BlockNumberFor<T>) -> Weight {
			let block_author = frame_system::Pallet::<T>::digest().convert_first(|item| {
				item.pre_runtime_try_to::<AccountIdOf<T>>(&sp_consensus_pow::POW_ENGINE_ID)
			});

			if let Some(author) = block_author {
				BlockAuthor::<T>::put(author);
			}

			T::WeightInfo::on_finalize().saturating_add(T::WeightInfo::on_initialize())
		}

		fn on_finalize(block_number: BlockNumberFor<T>) {
			if let Some(author) = BlockAuthor::<T>::get() {
				let reward = Self::reward_amount(block_number);
				Self::issue_reward(author, reward);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn reward_amount(block_number: BlockNumberFor<T>) -> BalanceOf<T> {
			let block_number: u64 = Self::sawtooth_adjusted_height(block_number);
			let period = usize::try_from(block_number / REWARD_HALF_LIFE).expect("assuming a 32-bit usize, we would need to be on block number 2^32 * REWARD_HALF_LIFE for this conversion to fail.\
	given a 1s block time it would take >340 million years to reach this point; qed");
			let decay_rate_inv = FixedU128::saturating_from_rational(19, 20);
			let multiplier = decay_rate_inv.saturating_pow(period);
			let reward_in_ctc: u128 =
				multiplier.saturating_mul_int(CREDO_PER_CTC).unique_saturated_into();
			let reward_amount = reward_in_ctc.saturating_mul(BASE_REWARD_IN_CTC.into());
			<BalanceOf<T>>::unique_saturated_from(reward_amount)
		}
		pub fn issue_reward(recipient: AccountIdOf<T>, amount: BalanceOf<T>) {
			drop(T::Currency::deposit_creating(&recipient, amount));
			Self::deposit_event(Event::<T>::RewardIssued(recipient, amount));
		}

		pub fn sawtooth_adjusted_height(block_number: BlockNumberFor<T>) -> u64 {
			let block_number: u64 = block_number.unique_saturated_into();
			block_number.saturating_add(SAWTOOTH_PORT_HEIGHT)
		}
	}
}
