#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub trait OnSwitch {
	fn on_switch();
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::U256;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_difficulty::Config {
		type RuntimeBlockNumber: IsType<<Self as frame_system::Config>::BlockNumber>
			+ Clone
			+ Into<sp_core::U256>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type OnSwitch: OnSwitch;
	}

	#[pallet::storage]
	#[pallet::getter(fn switch_block_number)]
	pub type SwitchBlockNumber<T> = StorageValue<_, U256, OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Switched to PoS. []
		Switched,
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadySwitched,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub switch_block_number: Option<T::BlockNumber>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { switch_block_number: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(switch_block_number) = self.switch_block_number {
				let switch_block_number = T::RuntimeBlockNumber::from_ref(&switch_block_number);
				let switch_block_number: sp_core::U256 = switch_block_number.clone().into();
				SwitchBlockNumber::<T>::put(switch_block_number);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Switch to PoS
		#[pallet::call_index(0)]
		#[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
		pub fn switch_to_pos(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(SwitchBlockNumber::<T>::get().is_none(), Error::<T>::AlreadySwitched);

			let block_number = frame_system::Pallet::<T>::block_number();
			let block_number: sp_core::U256 =
				T::RuntimeBlockNumber::from_ref(&block_number).clone().into();
			SwitchBlockNumber::<T>::put(block_number);

			pallet_difficulty::CurrentDifficulty::<T>::put(U256::MAX);

			Self::deposit_event(Event::Switched);

			T::OnSwitch::on_switch();

			Ok(().into())
		}
	}
}
