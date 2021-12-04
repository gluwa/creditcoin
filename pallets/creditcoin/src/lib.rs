#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::sp_runtime;

use frame_support::inherent::Vec;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Address<AccountId> {
	pub blockchain: Vec<u8>,
	pub value: Vec<u8>,
	pub network: Vec<u8>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Transfer<AccountId, BlockNum> {
	pub blockchain: Vec<u8>,
	pub src_address: Vec<u8>,
	pub dst_address: Vec<u8>,
	pub order: Vec<u8>,
	pub amount: Vec<u8>,
	pub tx: Vec<u8>,
	pub block: BlockNum,
	pub processed: bool,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Offer<AccountId, BlockNum> {
	pub blockchain: Vec<u8>,
	pub ask_order: Vec<u8>,
	pub bid_order: Vec<u8>,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct AskOrder<AccountId, Balance, BlockNum> {
	pub blockchain: Vec<u8>,
	pub address: Vec<u8>,
	pub amount: Vec<u8>,
	pub interest: Vec<u8>,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Fee<AccountId, BlockNum> {
	pub sighash: AccountId,
	pub block: BlockNum,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct BidOrder<AccountId, Balance, BlockNum> {
	pub blockchain: Vec<u8>,
	pub address: Vec<u8>,
	pub amount: Vec<u8>,
	pub interest: Vec<u8>,
	pub maturity: Vec<u8>,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct RepaymentOrder<AccountId, BlockNum> {
	pub blockchain: Vec<u8>,
	pub src_address: Vec<u8>,
	pub dst_address: Vec<u8>,
	pub amount: Vec<u8>,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub deal: Vec<u8>,
	pub previous_owner: AccountId,
	pub transfer: Vec<u8>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct DealOrder<AccountId, Balance, BlockNum> {
	pub blockchain: Vec<u8>,
	pub src_address: AccountId,
	pub dst_address: AccountId,
	pub amount: Vec<u8>,
	pub interest: Vec<u8>,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub loan_transfer: Option<Vec<u8>>,
	pub repayment_transfer: Option<Vec<u8>>,
	pub lock: Option<AccountId>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct DealOrderId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct AddressId<Hash>(Hash);

impl<H> AddressId<H> {
	pub fn new<Config>(
		blockchain: &[u8],
		address: &[u8],
		network: &[u8],
	) -> AddressId<<<Config as frame_system::Config>::Hashing as Hash>::Output>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key: Vec<u8> = blockchain.into_iter().chain(address).chain(network).copied().collect();
		AddressId(Config::Hashing::hash(&key.as_ref()))
	}
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::Randomness;
	use frame_support::Blake2_128Concat;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn deal_orders)]
	pub type DealOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DealOrderId<T::Hash>,
		DealOrder<T::AccountId, T::Balance, T::BlockNumber>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn addresses)]
	pub type Addresses<T: Config> =
		StorageMap<_, Blake2_128Concat, AddressId<T::Hash>, Address<T::AccountId>>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		AddressRegistered(Address<T::AccountId>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The specified address has already been registered to another account
		AddressAlreadyRegistered,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn register_address(
			origin: OriginFor<T>,
			blockchain: Vec<u8>,
			address: Vec<u8>,
			network: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let address_id = AddressId::new::<T>(&blockchain, &address, &network);
			let registration = Self::addresses(&address_id);
			ensure!(registration.is_none(), Error::<T>::AddressAlreadyRegistered);

			let entry = Address { blockchain, value: address, network, sighash: who };
			Self::deposit_event(Event::<T>::AddressRegistered(entry.clone()));
			<Addresses<T>>::insert(address_id, entry);

			Ok(())
		}
	}
}
