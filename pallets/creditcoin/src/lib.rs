#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::{Decode, Encode, EncodeLike};

use frame_support::pallet_prelude::*;
use frame_system::{
	offchain::{Account, SendSignedTransaction, Signer},
	pallet_prelude::*,
};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::Hash, KeyTypeId, RuntimeAppPublic, RuntimeDebug};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod ocw;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ctcs");

pub mod crypto {
	use crate::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct CtcAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for CtcAuthId {
		type RuntimeAppPublic = Public;

		type GenericPublic = sp_core::sr25519::Public;

		type GenericSignature = sp_core::sr25519::Signature;
	}
}

pub type ExternalAmount = u64;
type BlockchainLen = ConstU32<256>;
pub type Blockchain = BoundedVec<u8, BlockchainLen>;
type NetworkLen = ConstU32<256>;
pub type Network = BoundedVec<u8, NetworkLen>;
type GuidLen = ConstU32<256>;
pub type Guid = BoundedVec<u8, GuidLen>;
type ExternalAddressLen = ConstU32<256>;
pub type ExternalAddress = BoundedVec<u8, ExternalAddressLen>;
type ExternalTxIdLen = ConstU32<256>;
pub type ExternalTxId = BoundedVec<u8, ExternalTxIdLen>;
type VerifyStringLen = ConstU32<2560>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Address<AccountId> {
	pub blockchain: Blockchain,
	pub value: ExternalAddress,
	pub network: Network,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Transfer<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub network: Network,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub order: OrderId<BlockNum, Hash>,
	pub amount: ExternalAmount,
	pub tx: ExternalTxId,
	pub block: BlockNum,
	pub processed: bool,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PendingTransfer<AccountId, BlockNum, Hash> {
	verify_string: BoundedVec<u8, VerifyStringLen>,
	transfer: Transfer<AccountId, BlockNum, Hash>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Offer<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub ask_order: AskOrderId<BlockNum, Hash>,
	pub bid_order: BidOrderId<BlockNum, Hash>,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Fee<BlockNum> {
	pub block: BlockNum,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: BlockNum,
	pub fee: Balance,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RepaymentOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub expiration: BlockNum,
	pub block: BlockNum,
	pub deal: DealOrderId<BlockNum, Hash>,
	pub previous_owner: AccountId,
	pub transfer: TransferId<Hash>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DealOrder<AccountId, Balance, BlockNum, Hash> {
	pub blockchain: Blockchain,
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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AddressId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DealOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RepaymentOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OrderId<BlockNum, Hash> {
	Deal(DealOrderId<BlockNum, Hash>),
	Repayment(RepaymentOrderId<BlockNum, Hash>),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OfferId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransferId<Hash>(Hash);

fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
	const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
	let mut hex = Vec::with_capacity(bytes.len() * 2);
	for byte in bytes {
		hex.push(HEX_CHARS_LOWER[(byte >> 4) as usize]);
		hex.push(HEX_CHARS_LOWER[(byte & 0x0F) as usize]);
	}
	hex
}

macro_rules! replace_expr {
	($_t:tt $sub:expr) => {
		$sub
	};
}

macro_rules! count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

macro_rules! concatenate {
	($($bytes: expr),+) => {
		{
			let mut buf = Vec::with_capacity(strip_plus!($(+ $bytes.len())+));
			$(buf.extend($bytes);)+
			buf
		}
	};

	($($bytes: expr),+; $last_bytes: expr; sep = $sep: literal) => {
		{
			let mut buf = Vec::with_capacity(strip_plus!($(+ $bytes.len())+) + count_tts!($($bytes)+) );
			$(
				buf.extend($bytes);
				buf.push($sep);
			)+
			buf.extend($last_bytes);
			buf
		}
	}
}

impl<B, H> OrderId<B, H>
where
	H: AsRef<[u8]>,
{
	pub fn to_hex(&self) -> Vec<u8> {
		let bytes = match self {
			OrderId::Deal(deal) => deal.1.as_ref(),
			OrderId::Repayment(repay) => repay.1.as_ref(),
		};
		bytes_to_hex(bytes)
	}
}

impl<H> AddressId<H> {
	pub fn new<Config>(blockchain: &[u8], address: &[u8], network: &[u8]) -> AddressId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(blockchain, address, network);
		AddressId(Config::Hashing::hash(&key))
	}
}

impl<B, H> AskOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> AskOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		AskOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<B, H> BidOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> BidOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		BidOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<B, H> RepaymentOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> RepaymentOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		RepaymentOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<H> TransferId<H> {
	pub fn new<Config>(blockchain: &[u8], network: &[u8], blockchain_tx_id: &[u8]) -> TransferId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(blockchain, network, blockchain_tx_id);
		TransferId(Config::Hashing::hash(&key))
	}
}

impl<B, H> OfferId<B, H> {
	pub fn new<Config>(
		expiration_block: B,
		ask_order_id: &AskOrderId<BlockNumberFor<Config>, H>,
		bid_order_id: &BidOrderId<BlockNumberFor<Config>, H>,
	) -> Self
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		let ask_bytes = ask_order_id.1.as_ref();
		let bid_bytes = bid_order_id.1.as_ref();
		let key = concatenate!(ask_bytes, bid_bytes);
		OfferId(expiration_block, Config::Hashing::hash(&key))
	}
}

impl<B, H> DealOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, offer_id: &OfferId<BlockNumberFor<Config>, H>) -> Self
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		DealOrderId(expiration_block, Config::Hashing::hash(offer_id.1.as_ref()))
	}
}

use codec::FullCodec;
use extend::ext;
use frame_support::{
	storage::types::QueryKindTrait,
	traits::{ConstU32, Get, StorageInstance},
	BoundedVec, StorageHasher,
};

trait Id<BlockNum, Hash> {
	fn expiration(&self) -> BlockNum;
	fn hash(&self) -> Hash;
}

macro_rules! impl_id {
	($id: ident) => {
		impl<BlockNum, Hash> Id<BlockNum, Hash> for $id<BlockNum, Hash>
		where
			BlockNum: Clone,
			Hash: Clone,
		{
			fn expiration(&self) -> BlockNum {
				self.0.clone()
			}
			fn hash(&self) -> Hash {
				self.1.clone()
			}
		}

		impl<'a, BlockNum, Hash> Id<BlockNum, Hash> for &'a $id<BlockNum, Hash>
		where
			BlockNum: Clone,
			Hash: Clone,
		{
			fn expiration(&self) -> BlockNum {
				self.0.clone()
			}
			fn hash(&self) -> Hash {
				self.1.clone()
			}
		}
	};
}

impl_id!(DealOrderId);
impl_id!(AskOrderId);
impl_id!(BidOrderId);
impl_id!(OfferId);
impl_id!(RepaymentOrderId);

#[ext(name = DoubleMapExt)]
impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, QueryKind, OnEmpty, MaxValues, IdTy>
	frame_support::storage::types::StorageDoubleMap<
		Prefix,
		Hasher1,
		Key1,
		Hasher2,
		Key2,
		Value,
		QueryKind,
		OnEmpty,
		MaxValues,
	> where
	Prefix: StorageInstance,
	Hasher1: StorageHasher,
	Hasher2: StorageHasher,
	Key1: FullCodec + Clone,
	Key2: FullCodec + Clone,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: Get<QueryKind::Query> + 'static,
	MaxValues: Get<Option<u32>>,
	IdTy: Id<Key1, Key2>,
{
	fn insert_id<V>(id: IdTy, val: V)
	where
		V: EncodeLike<Value>,
	{
		Self::insert(id.expiration(), id.hash(), val);
	}

	fn try_get_id(id: &IdTy) -> frame_support::dispatch::result::Result<Value, ()> {
		Self::try_get(id.expiration(), id.hash())
	}

	fn contains_id(id: &IdTy) -> bool {
		Self::contains_key(id.expiration(), id.hash())
	}
}

pub type BalanceFor<T> = <T as pallet_balances::Config>::Balance;

#[allow(unused_macros)]
macro_rules! try_get {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		crate::pallet::$storage::<$t>::try_get($key).map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

macro_rules! try_get_id {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		<crate::pallet::$storage<$t> as DoubleMapExt<_, _, _, _, _, _, _, _, _, _>>::try_get_id(
			$key,
		)
		.map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

#[frame_support::pallet]
pub mod pallet {
	use core::convert::TryInto;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, Blake2_128Concat};
	use frame_system::{
		ensure_signed,
		offchain::{AppCrypto, CreateSignedTransaction},
		pallet_prelude::*,
	};

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + CreateSignedTransaction<Call<Self>>
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Call: From<Call<Self>>;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		type FromAccountId: From<sp_core::sr25519::Public>
			+ Into<Self::AccountId>
			+ From<Self::AccountId>
			+ Clone
			+ core::fmt::Debug
			+ PartialEq<Self::FromAccountId>
			+ AsRef<[u8; 32]>;

		type InternalPublic: sp_core::crypto::UncheckedFrom<[u8; 32]>;

		type PublicSigning: From<Self::InternalPublic> + Into<Self::Public>;

		type PendingTransferLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn authorities)]
	pub type Authorities<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	#[pallet::storage]
	#[pallet::getter(fn pending_transfers)]
	pub type PendingTransfers<T: Config> = StorageValue<
		_,
		BoundedVec<PendingTransfer<T::AccountId, T::BlockNumber, T::Hash>, T::PendingTransferLimit>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn deal_orders)]
	pub type DealOrders<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		DealOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn repayment_orders)]
	pub type RepaymentOrders<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		RepaymentOrder<T::AccountId, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn addresses)]
	pub type Addresses<T: Config> =
		StorageMap<_, Blake2_128Concat, AddressId<T::Hash>, Address<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn ask_orders)]
	pub type AskOrders<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		AskOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn bid_orders)]
	pub type BidOrders<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		BidOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	pub type Offers<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		Offer<T::AccountId, T::BlockNumber, T::Hash>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transfers)]
	pub type Transfers<T: Config> = StorageMap<
		_,
		Identity,
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
		AddressRegistered(AddressId<T::Hash>, Address<T::AccountId>),

		TransferRegistered(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),

		TransferFinalized(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),

		AskOrderAdded(
			AskOrderId<T::BlockNumber, T::Hash>,
			AskOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
		),

		BidOrderAdded(
			BidOrderId<T::BlockNumber, T::Hash>,
			BidOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash>,
		),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The specified address has already been registered to another account
		AddressAlreadyRegistered,

		NonExistentAddress,

		NonExistentDealOrder,
		NonExistentAskOrder,
		NonExistentBidOrder,
		NonExistentOffer,
		NonExistentTransfer,

		TransferAlreadyRegistered,
		TransferMismatch,
		TransferAlreadyProcessed,

		InsufficientAuthority,

		NonExistentRepaymentOrder,

		DuplicateId,

		NotAddressOwner,

		OffchainSignedTxFailed,

		NoLocalAcctForSignedTx,

		RepaymentOrderNonZeroGain,

		AddressPlatformMismatch,

		AlreadyAuthority,

		DuplicateOffer,

		DealIncomplete,

		DealOrderAlreadyLocked,
		DealOrderExpired,

		NotFundraiser,

		MalformedDealOrder,

		NotInvestor,

		ScaleDecodeError,

		PendingTransferPoolFull,

		VerifyStringTooLong,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_block_number: T::BlockNumber) -> Weight {
			PendingTransfers::<T>::kill();
			0
		}
		fn offchain_worker(_block_number: T::BlockNumber) {
			if let Some(auth_id) = Self::authority_id() {
				let auth_id = T::FromAccountId::from(auth_id);
				for PendingTransfer { verify_string: _, transfer } in PendingTransfers::<T>::get() {
					log::debug!("verifying transfer");
					// TODO: actually hit gateway to verify given transaction
					if let Err(e) = Self::offchain_signed_tx(auth_id.clone(), |_| {
						Call::finalize_transfer { transfer: transfer.clone() }
					}) {
						log::error!("Failed to send finalize transfer transaction: {:?}", e);
					}
				}
			} else {
				log::trace!("Not authority, skipping off chain work");
			}
		}
		fn on_finalize(block_number: T::BlockNumber) {
			log::debug!("Cleaning up expired entries");
			AskOrders::<T>::remove_prefix(block_number, None);
			BidOrders::<T>::remove_prefix(block_number, None);
			DealOrders::<T>::remove_prefix(block_number, None);
			RepaymentOrders::<T>::remove_prefix(block_number, None);
			Offers::<T>::remove_prefix(block_number, None);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Registers an external address on `blockchain` and `network` with value `address`
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn register_address(
			origin: OriginFor<T>,
			blockchain: Blockchain,
			address: ExternalAddress,
			network: Network,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let address_id = AddressId::new::<T>(&blockchain, &address, &network);
			ensure!(
				!Addresses::<T>::contains_key(&address_id),
				Error::<T>::AddressAlreadyRegistered
			);

			let entry = Address { blockchain, value: address, network, sighash: who };
			Self::deposit_event(Event::<T>::AddressRegistered(address_id.clone(), entry.clone()));
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
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order_id = AskOrderId::new::<T>(Self::block_number() + expiration, &guid);
			ensure!(!AskOrders::<T>::contains_id(&ask_order_id), Error::<T>::DuplicateId);

			let address = Self::get_address(&address_id)?;
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

			Self::deposit_event(Event::<T>::AskOrderAdded(ask_order_id.clone(), ask_order.clone()));

			AskOrders::<T>::insert_id(ask_order_id, ask_order);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_bid_order(
			origin: OriginFor<T>,
			address_id: AddressId<T::Hash>,
			amount: ExternalAmount,
			interest: ExternalAmount,
			maturity: BlockNumberFor<T>,
			fee: BalanceFor<T>,
			expiration: BlockNumberFor<T>,
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bid_order_id = BidOrderId::new::<T>(Self::block_number() + expiration, &guid);
			ensure!(!BidOrders::<T>::contains_id(&bid_order_id), Error::<T>::DuplicateId);

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

				Self::deposit_event(Event::<T>::BidOrderAdded(
					bid_order_id.clone(),
					bid_order.clone(),
				));
				BidOrders::<T>::insert_id(bid_order_id, bid_order);
				Ok(())
			} else {
				Err(Error::<T>::NonExistentAddress.into())
			}
		}

		#[pallet::weight(10_000)]
		pub fn add_offer(
			origin: OriginFor<T>,
			ask_order_id: AskOrderId<T::BlockNumber, T::Hash>,
			bid_order_id: BidOrderId<T::BlockNumber, T::Hash>,
			expiration: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order = try_get_id!(AskOrders<T>, &ask_order_id, NonExistentAskOrder)?;

			let _bid_order = try_get_id!(BidOrders<T>, &bid_order_id, NonExistentBidOrder)?;

			let src_address = Self::get_address(&ask_order.address)?;

			// TODO: Do validation of addresses and parameters here

			let offer_id =
				OfferId::new::<T>(Self::block_number() + expiration, &ask_order_id, &bid_order_id);

			ensure!(!Offers::<T>::contains_id(&offer_id), Error::<T>::DuplicateOffer);

			let offer = Offer {
				ask_order: ask_order_id,
				bid_order: bid_order_id,
				block: Self::block_number(),
				blockchain: src_address.blockchain,
				expiration,
				sighash: who,
			};

			Offers::<T>::insert_id(offer_id, offer);

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_deal_order(
			origin: OriginFor<T>,
			offer_id: OfferId<T::BlockNumber, T::Hash>,
			expiration: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let offer = try_get_id!(Offers<T>, &offer_id, NonExistentOffer)?;
			let ask_order = try_get_id!(AskOrders<T>, &offer.ask_order, NonExistentAskOrder)?;

			let bid_order = try_get_id!(BidOrders<T>, &offer.bid_order, NonExistentBidOrder)?;

			// TODO: checks to make sure orders match up

			let deal_order_id = DealOrderId::new::<T>(Self::block_number() + expiration, &offer_id);
			let deal_order = DealOrder {
				blockchain: offer.blockchain,
				src_address: ask_order.address,
				dst_address: bid_order.address,
				amount: bid_order.amount,
				interest: bid_order.interest,
				maturity: bid_order.maturity,
				fee: bid_order.fee,
				expiration,
				block: Self::block_number(),
				sighash: who,
				loan_transfer: None,
				lock: None,
				repayment_transfer: None,
			};

			DealOrders::<T>::insert_id(deal_order_id, deal_order);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn lock_deal_order(
			origin: OriginFor<T>,
			deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			DealOrders::<T>::try_mutate(
				deal_order_id.expiration(),
				deal_order_id.hash(),
				|value| {
					if let Some(deal_order) = value {
						ensure!(deal_order.lock.is_none(), Error::<T>::DealOrderAlreadyLocked);
						ensure!(deal_order.loan_transfer.is_some(), Error::<T>::DealIncomplete);
						ensure!(deal_order.sighash == who, Error::<T>::NotFundraiser);
						deal_order.lock = Some(who);
						Ok(())
					} else {
						Err(Error::<T>::NonExistentDealOrder)
					}
				},
			)?;

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,2))]
		pub fn complete_deal_order(
			origin: OriginFor<T>,
			deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
			transfer_id: TransferId<T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			DealOrders::<T>::try_mutate(
				deal_order_id.expiration(),
				deal_order_id.hash(),
				|value| {
					if let Some(deal_order) = value {
						let src_address =
							try_get!(Addresses<T>, &deal_order.src_address, NonExistentAddress)?;

						ensure!(src_address.sighash == who, Error::<T>::NotInvestor);

						let head = Self::block_number();
						ensure!(head >= deal_order.block, Error::<T>::MalformedDealOrder);

						let elapsed = head - deal_order.block;
						ensure!(deal_order.expiration >= elapsed, Error::<T>::DealOrderExpired);

						Transfers::<T>::try_mutate(transfer_id, |transfer| {
							if let Some(transfer) = transfer {
								ensure!(
									transfer.order == OrderId::Deal(deal_order_id),
									Error::<T>::TransferMismatch
								);
								ensure!(
									transfer.amount == deal_order.amount,
									Error::<T>::TransferMismatch
								);
								ensure!(transfer.sighash == who, Error::<T>::TransferMismatch);
								ensure!(!transfer.processed, Error::<T>::TransferAlreadyProcessed);
								Ok(())
							} else {
								Err(Error::<T>::NonExistentTransfer)
							}
						})
					} else {
						Err(Error::<T>::NonExistentDealOrder)
					}
				},
			)?;
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1))]
		pub fn register_transfer(
			origin: OriginFor<T>,
			gain: ExternalAmount,
			order_id: OrderId<T::BlockNumber, T::Hash>,
			blockchain_tx_id: ExternalTxId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (src_address_id, dest_address_id, mut amount) = match &order_id {
				OrderId::Deal(deal_order_id) => {
					let order = try_get_id!(DealOrders<T>, &deal_order_id, NonExistentDealOrder)?;

					if gain == 0 {
						(order.src_address, order.dst_address, order.amount)
					} else {
						(order.dst_address, order.src_address, order.amount)
					}
				},
				OrderId::Repayment(repay_order_id) => {
					ensure!(gain == 0, Error::<T>::RepaymentOrderNonZeroGain);
					let order = try_get_id!(
						RepaymentOrders<T>,
						&repay_order_id,
						NonExistentRepaymentOrder
					)?;
					(order.src_address, order.dst_address, order.amount)
				},
			};

			let src_address = Self::get_address(&src_address_id)?;
			let dest_address = Self::get_address(&dest_address_id)?;

			ensure!(src_address.sighash == who, Error::<T>::NotAddressOwner);

			ensure!(
				src_address.blockchain == dest_address.blockchain &&
					src_address.network == dest_address.network,
				Error::<T>::AddressPlatformMismatch
			);

			let transfer_id = TransferId::new::<T>(
				&src_address.blockchain,
				&src_address.network,
				&blockchain_tx_id,
			);
			ensure!(
				!Transfers::<T>::contains_key(&transfer_id),
				Error::<T>::TransferAlreadyRegistered
			);

			if &*blockchain_tx_id == &*b"0" {
				amount = 0;
				let transfer = Transfer {
					blockchain: src_address.blockchain,
					network: src_address.network,
					amount,
					block: <frame_system::Pallet<T>>::block_number(),
					src_address: src_address_id,
					dst_address: dest_address_id,
					order: order_id,
					processed: false,
					sighash: who.clone(),
					tx: blockchain_tx_id,
				};
				Self::deposit_event(Event::<T>::TransferRegistered(
					transfer_id.clone(),
					transfer.clone(),
				));
				Transfers::<T>::insert(transfer_id, transfer);
			} else {
				amount += gain;
				let mut buf = [b'0'; lexical_core::BUFFER_SIZE];
				let amount_str = lexical_core::write(amount, &mut buf);
				let order_id_hex = order_id.to_hex();

				let verify_string = concatenate!(
					&*src_address.blockchain,
					b"verify",
					&*src_address.value,
					&*dest_address.value,
					&order_id_hex,
					&*amount_str,
					&*blockchain_tx_id;
					&*src_address.network;
					sep = b' '
				);
				let verify_string =
					verify_string.try_into().map_err(|_| Error::<T>::VerifyStringTooLong)?;
				let transfer = Transfer {
					blockchain: src_address.blockchain,
					network: src_address.network,
					amount,
					block: <frame_system::Pallet<T>>::block_number(),
					src_address: src_address_id,
					dst_address: dest_address_id,
					order: order_id,
					processed: false,
					sighash: who.clone(),
					tx: blockchain_tx_id,
				};

				Self::deposit_event(Event::<T>::TransferRegistered(
					transfer_id.clone(),
					transfer.clone(),
				));

				let pending = PendingTransfer { verify_string, transfer };
				PendingTransfers::<T>::try_mutate(|transfers| transfers.try_push(pending))
					.map_err(|()| Error::<T>::PendingTransferPoolFull)?;
			}

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn finalize_transfer(
			origin: OriginFor<T>,
			transfer: Transfer<T::AccountId, T::BlockNumber, T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Authorities::<T>::contains_key(&who), Error::<T>::InsufficientAuthority);

			let key = TransferId::new::<T>(&transfer.blockchain, &transfer.network, &transfer.tx);
			ensure!(!Transfers::<T>::contains_key(&key), Error::<T>::TransferAlreadyRegistered);
			let mut transfer = transfer;
			transfer.block = frame_system::Pallet::<T>::block_number();

			Self::deposit_event(Event::<T>::TransferFinalized(key.clone(), transfer.clone()));
			Transfers::<T>::insert(key, transfer);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn add_authority(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(!Authorities::<T>::contains_key(&who), Error::<T>::AlreadyAuthority);

			Authorities::<T>::insert(who, ());

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn block_number() -> BlockNumberFor<T> {
		<frame_system::Pallet<T>>::block_number()
	}
	pub fn get_address(address_id: &AddressId<T::Hash>) -> Result<Address<T::AccountId>, Error<T>> {
		Self::addresses(&address_id).ok_or(Error::<T>::NonExistentAddress)
	}

	fn authority_id() -> Option<T::AccountId> {
		let local_keys = crypto::Public::all()
			.into_iter()
			.map(|p| sp_core::sr25519::Public::from(p).into())
			.collect::<Vec<T::FromAccountId>>();

		log::trace!("{:?}", local_keys);

		Authorities::<T>::iter_keys().find_map(|auth| {
			let acct = auth.clone().into();
			local_keys.contains(&acct).then(|| auth)
		})
	}

	pub fn offchain_signed_tx(
		auth_id: T::FromAccountId,
		call: impl Fn(&Account<T>) -> Call<T>,
	) -> Result<(), Error<T>> {
		use sp_core::crypto::UncheckedFrom;
		let auth_bytes: &[u8; 32] = auth_id.as_ref();
		let public = T::InternalPublic::unchecked_from(*auth_bytes);
		let public: T::PublicSigning = public.into();
		let signer =
			Signer::<T, T::AuthorityId>::any_account().with_filter(sp_std::vec![public.into()]);
		let result = signer.send_signed_transaction(call);

		if let Some((acc, res)) = result {
			if res.is_err() {
				log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(Error::OffchainSignedTxFailed)
			} else {
				return Ok(())
			}
		}

		log::error!("No local account available");
		Err(Error::NoLocalAcctForSignedTx)
	}
}
