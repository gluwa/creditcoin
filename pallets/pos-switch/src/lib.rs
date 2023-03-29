#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, SetCode};
	use sp_core::U256;
	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_difficulty::Config
	where
		<Self as frame_system::Config>::BlockNumber: Into<sp_core::U256>,
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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
	pub enum Event<T: Config>
	where
		<T as frame_system::Config>::BlockNumber: Into<sp_core::U256>,
	{
		/// Switched to PoS. []
		Switched,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		<T as frame_system::Config>::BlockNumber: Into<sp_core::U256>,
	{
		/// Switch to PoS
		#[pallet::call_index(0)]
		#[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
		pub fn switch_to_pos(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let block_number: sp_core::U256 = frame_system::Pallet::<T>::block_number().into();
			SwitchBlockNumber::<T>::put(block_number);

			frame_system::Pallet::<T>::can_set_code(&code)?;
			<T as frame_system::Config>::OnSetCode::set_code(code)?;
			pallet_difficulty::CurrentDifficulty::<T>::put(U256::MAX);

			Self::deposit_event(Event::Switched);

			Ok(().into())
		}
	}
}
