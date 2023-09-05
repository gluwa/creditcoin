#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use sp_core::U256;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_difficulty::Config {
		type RuntimeBlockNumber: IsType<<Self as frame_system::Config>::BlockNumber>
			+ Clone
			+ Into<sp_core::U256>;
	}

	#[pallet::storage]
	#[pallet::getter(fn switch_block_number)]
	pub type SwitchBlockNumber<T> = StorageValue<_, U256, OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

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
}
