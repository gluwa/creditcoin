#![cfg_attr(not(feature = "std"), no_std)]
pub mod sampling;

pub use pallet::{Config, Pallet};

#[frame_support::pallet]
pub mod pallet {

	use crate::sampling::GetOne;
	use frame_support::pallet_prelude::{ValueQuery, *};
	use sp_runtime::Perquintill;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn sample_size)]
	pub type SampleSize<T: Config> = StorageValue<_, Perquintill, ValueQuery, GetOne>;
}
