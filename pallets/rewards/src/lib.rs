#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<AccountIdOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn reward_amount)]
	pub type RewardAmount<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

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

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub reward_amount: BalanceOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { reward_amount: <T::Currency as Currency<AccountIdOf<T>>>::minimum_balance() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			RewardAmount::<T>::put(self.reward_amount);
		}
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

			0
		}

		fn on_finalize(_block_number: BlockNumberFor<T>) {
			if let Some(author) = BlockAuthor::<T>::get() {
				let reward = Self::reward_amount();
				Self::issue_reward(author, reward);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn issue_reward(recipient: AccountIdOf<T>, amount: BalanceOf<T>) {
			drop(T::Currency::deposit_creating(&recipient, amount));
			Self::deposit_event(Event::<T>::RewardIssued(recipient, amount));
		}
	}
}
