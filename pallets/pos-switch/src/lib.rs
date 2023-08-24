#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;
pub type InitialValidators<T> = Vec<InitialValidator<T>>;

#[derive(Encode, Decode, PartialEq, Eq, TypeInfo)]
#[codec(encode_bound(T: ))]
#[codec(decode_bound(T: ))]
#[codec(mel_bound(T: ))]
#[scale_info(skip_type_params(T))]
pub struct InitialValidator<T: pallet::Config> {
	pub stash: T::AccountId,
	pub controller: T::AccountId,
	pub bonded: T::Balance,
	pub controller_balance: T::Balance,
	pub babe: sp_consensus_babe::AuthorityId,
	pub grandpa: sp_consensus_grandpa::AuthorityId,
	pub im_online: pallet_im_online::sr25519::AuthorityId,
	pub invulnerable: bool,
}

impl<T: pallet::Config> sp_std::fmt::Debug for InitialValidator<T>
where
	T::AccountId: sp_std::fmt::Debug,
	T::Balance: sp_std::fmt::Debug,
{
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		f.debug_struct("InitialValidator")
			.field("stash", &self.stash)
			.field("controller", &self.controller)
			.field("bonded", &self.bonded)
			.field("controller_balance", &self.controller_balance)
			.field("babe", &self.babe)
			.field("grandpa", &self.grandpa)
			.field("im_online", &self.im_online)
			.field("invulnerable", &self.invulnerable)
			.finish()
	}
}

impl<T: pallet::Config> Clone for InitialValidator<T> {
	fn clone(&self) -> Self {
		Self {
			stash: self.stash.clone(),
			controller: self.controller.clone(),
			bonded: self.bonded.clone(),
			babe: self.babe.clone(),
			grandpa: self.grandpa.clone(),
			im_online: self.im_online.clone(),
			controller_balance: self.controller_balance.clone(),
			invulnerable: self.invulnerable,
		}
	}
}

pub trait OnSwitch {
	type Config: pallet::Config;
	fn on_switch(initial_validators: InitialValidators<Self::Config>);
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

		type OnSwitch: OnSwitch<Config = Self>;

		type Balance: Parameter;
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
		pub fn switch_to_pos(
			origin: OriginFor<T>,
			initial_validators: InitialValidators<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(SwitchBlockNumber::<T>::get().is_none(), Error::<T>::AlreadySwitched);

			let block_number = frame_system::Pallet::<T>::block_number();
			let block_number: sp_core::U256 =
				T::RuntimeBlockNumber::from_ref(&block_number).clone().into();
			SwitchBlockNumber::<T>::put(block_number);

			pallet_difficulty::CurrentDifficulty::<T>::put(U256::MAX);

			Self::deposit_event(Event::Switched);

			T::OnSwitch::on_switch(initial_validators);

			Ok(().into())
		}
	}
}
