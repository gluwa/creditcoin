#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_system::offchain::{Account, SendSignedTransaction, Signer};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Hash,
	KeyTypeId, RuntimeDebug,
};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ctcs");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	use sp_runtime::{traits::Verify, MultiSignature, MultiSigner};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct CtcAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for CtcAuthId {
		type RuntimeAppPublic = Public;

		type GenericPublic = sp_core::sr25519::Public;

		type GenericSignature = sp_core::sr25519::Signature;
	}
}

pub type ExternalAmount = u64;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Address<AccountId> {
	pub blockchain: Vec<u8>,
	pub value: Vec<u8>,
	pub network: Vec<u8>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Transfer<AccountId, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub order: OrderId<Hash>,
	pub amount: ExternalAmount,
	pub tx: Vec<u8>,
	pub block: BlockNum,
	pub processed: bool,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Offer<AccountId, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub ask_order: AskOrderId<Hash>,
	pub bid_order: BidOrderId<Hash>,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AskOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Fee<BlockNum> {
	pub block: BlockNum,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BidOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct RepaymentOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub deal: DealOrderId<Hash>,
	pub previous_owner: AccountId,
	pub transfer: TransferId<Hash>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DealOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Vec<u8>,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub loan_transfer: Option<TransferId<Hash>>,
	pub repayment_transfer: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AddressId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AskOrderId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BidOrderId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DealOrderId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct RepaymentOrderId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum OrderId<Hash> {
	Deal(DealOrderId<Hash>),
	Repayment(RepaymentOrderId<Hash>),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct OfferId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TransferId<Hash>(Hash);

impl<H> AddressId<H> {
	pub fn new<Config>(blockchain: &[u8], address: &[u8], network: &[u8]) -> AddressId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key: Vec<u8> = blockchain.into_iter().chain(address).chain(network).copied().collect();
		AddressId(Config::Hashing::hash(&key.as_ref()))
	}
}

impl<H> AskOrderId<H> {
	pub fn new<Config>(guid: Vec<u8>) -> AskOrderId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		AskOrderId(Config::Hashing::hash(&guid.as_ref()))
	}
}

impl<H> BidOrderId<H> {
	pub fn new<Config>(guid: Vec<u8>) -> BidOrderId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		BidOrderId(Config::Hashing::hash(&guid.as_ref()))
	}
}

pub type BalanceFor<T> = <T as pallet_balances::Config>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::{Currency, LockableCurrency, Randomness};
	use frame_support::Blake2_128Concat;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::offchain::{AppCrypto, CreateSignedTransaction};
	use frame_system::{ensure_signed, pallet_prelude::*};

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + CreateSignedTransaction<Call<Self>>
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Call: From<Call<Self>>;

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		type Currency: Currency<Self::AccountId> + LockableCurrency<Self::AccountId>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn deal_orders)]
	pub type DealOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DealOrderId<T::Hash>,
		DealOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn repayment_orders)]
	pub type RepaymentOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		RepaymentOrderId<T::Hash>,
		RepaymentOrder<T::AccountId, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn addresses)]
	pub type Addresses<T: Config> =
		StorageMap<_, Blake2_128Concat, AddressId<T::Hash>, Address<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn ask_orders)]
	pub type AskOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AskOrderId<T::Hash>,
		AskOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn bid_orders)]
	pub type BidOrders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BidOrderId<T::Hash>,
		BidOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	pub type Offers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		OfferId<T::Hash>,
		Offer<T::AccountId, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transfers)]
	pub type Transfers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		TransferId<T::Hash>,
		Transfer<T::AccountId, T::BlockNumber, T::Hash>,
	>;

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

		NonExistentAddress,

		DuplicateId,

		NotAddressOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {}
	}

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

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn add_ask_order(
			origin: OriginFor<T>,
			address_id: AddressId<T::Hash>,
			amount: ExternalAmount,
			interest: ExternalAmount,
			maturity: BlockNumberFor<T>,
			fee: BalanceFor<T>,
			expiration: BlockNumberFor<T>,
			guid: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order_id = AskOrderId::new::<T>(guid);
			let existing_order = Self::ask_orders(&ask_order_id);
			ensure!(existing_order.is_none(), Error::<T>::DuplicateId);

			let address = Self::addresses(&address_id);
			if let Some(address) = address {
				ensure!(address.sighash == who, Error::<T>::NotAddressOwner);
				let ask_order = AskOrder {
					blockchain: address.blockchain,
					address: address_id,
					amount,
					interest,
					maturity,
					fee,
					expiration,
					block: <frame_system::Pallet<T>>::block_number(),
					sighash: who,
				};

				AskOrders::<T>::insert(ask_order_id, ask_order);
				Ok(())
			} else {
				Err(Error::<T>::NonExistentAddress.into())
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn add_bid_order(
			origin: OriginFor<T>,
			address_id: AddressId<T::Hash>,
			amount: ExternalAmount,
			interest: ExternalAmount,
			maturity: BlockNumberFor<T>,
			fee: BalanceFor<T>,
			expiration: BlockNumberFor<T>,
			guid: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bid_order_id = BidOrderId::new::<T>(guid);
			let existing_order = Self::bid_orders(&bid_order_id);
			ensure!(existing_order.is_none(), Error::<T>::DuplicateId);

			let address = Self::addresses(&address_id);
			if let Some(address) = address {
				ensure!(address.sighash == who, Error::<T>::NotAddressOwner);
				let bid_order = BidOrder {
					blockchain: address.blockchain,
					address: address_id,
					amount,
					interest,
					maturity,
					fee,
					expiration,
					block: <frame_system::Pallet<T>>::block_number(),
					sighash: who,
				};

				BidOrders::<T>::insert(bid_order_id, bid_order);
				Ok(())
			} else {
				Err(Error::<T>::NonExistentAddress.into())
			}
		}

impl<T: Config> Pallet<T> {
	pub fn offchain_signed_tx(call: impl Fn(&Account<T>) -> Call<T>) -> Result<(), Error<T>> {
		let signer = Signer::<T, T::AuthorityId>::any_account();
		let result = signer.send_signed_transaction(call);

		if let Some((acc, res)) = result {
			if res.is_err() {
				tracing::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(Error::OffchainSignedTxFailed);
			} else {
				return Ok(());
			}
		}

		tracing::error!("No local account available");
		Err(Error::NoLocalAcctForSignedTx)
	}
}
