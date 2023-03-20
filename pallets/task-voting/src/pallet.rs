pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::sampling::GetOne;
	use crate::types::Entry;
	use crate::VotingPowerProvider;
	use frame_support::pallet_prelude::{ValueQuery, *};
	use frame_support::BoundedBTreeSet;
	use parity_scale_codec::Decode;
	use parity_scale_codec::FullEncode;
	use scale_info::TypeInfo;
	use sp_runtime::Perquintill;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		// must be hash safe
		type ItemId: Parameter + MaxEncodedLen;
		type Item: Parameter + Ord + MaxEncodedLen;
		type Who: Parameter + Ord + MaxEncodedLen;
		type PowerUnit: Decode + FullEncode + TypeInfo + MaxEncodedLen;
		type PowerProvider: VotingPowerProvider<
			Who = Self::Who,
			ItemId = Self::ItemId,
			Unit = Self::PowerUnit,
		>;
		#[pallet::constant]
		type MaxVoters: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn sample_size)]
	pub type SampleSize<T: Config> = StorageValue<_, Perquintill, ValueQuery, GetOne>;

	#[pallet::storage]
	pub type Entries<T: Config> =
		StorageMap<_, Identity, T::ItemId, Entry<T::Who, T::Item, T::PowerUnit, T::MaxVoters>>;

	#[pallet::storage]
	pub type Index<T: Config> =
		StorageMap<_, Identity, T::ItemId, BoundedBTreeSet<T::Who, T::MaxVoters>>;

	#[pallet::error]
	pub enum Error<T> {
		/// A voter is attempting to vote multiple times for the same task.
		DoubleVoting,
		/// A task is at its max voter capacity.
		TooManyVoters,
	}
}
