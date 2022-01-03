#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

use frame_system::{
	offchain::{Account, SendSignedTransaction, Signer},
	pallet_prelude::*,
};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	traits::Hash,
	KeyTypeId, RuntimeAppPublic, RuntimeDebug,
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
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
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
	pub network: Vec<u8>,
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
pub struct PendingTransfer<AccountId, BlockNum, Hash> {
	verify_string: Vec<u8>,
	transfer: Transfer<AccountId, BlockNum, Hash>,
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

fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
	const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
	let mut hex = Vec::with_capacity(bytes.len() * 2);
	for byte in bytes {
		hex.push(HEX_CHARS_LOWER[(byte >> 4) as usize]);
		hex.push(HEX_CHARS_LOWER[(byte & 0x0F) as usize]);
	}
	hex
}

macro_rules! impl_to_hex {
	($id: ident) => {
		impl<H> $id<H>
		where
			H: AsRef<[u8]>,
		{
			pub fn to_hex(&self) -> Vec<u8> {
				bytes_to_hex(self.0.as_ref())
			}
		}
	};
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

impl_to_hex!(AskOrderId);
impl_to_hex!(BidOrderId);

impl<H> OrderId<H>
where
	H: AsRef<[u8]>,
{
	pub fn to_hex(&self) -> Vec<u8> {
		let bytes = match self {
			OrderId::Deal(deal) => deal.0.as_ref(),
			OrderId::Repayment(repay) => repay.0.as_ref(),
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

impl<H> AskOrderId<H> {
	pub fn new<Config>(guid: &[u8]) -> AskOrderId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		AskOrderId(Config::Hashing::hash(guid))
	}
}

impl<H> BidOrderId<H> {
	pub fn new<Config>(guid: &[u8]) -> BidOrderId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		BidOrderId(Config::Hashing::hash(guid))
	}
}

impl<H> RepaymentOrderId<H> {
	pub fn new<Config>(guid: &[u8]) -> RepaymentOrderId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		RepaymentOrderId(Config::Hashing::hash(guid))
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

impl<H> OfferId<H> {
	pub fn new<Config>(ask_order_id: &AskOrderId<H>, bid_order_id: &BidOrderId<H>) -> Self
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		let ask_bytes = ask_order_id.0.as_ref();
		let bid_bytes = bid_order_id.0.as_ref();
		let key = concatenate!(ask_bytes, bid_bytes);
		OfferId(Config::Hashing::hash(&key))
	}
}

impl<H> DealOrderId<H> {
	pub fn new<Config>(offer_id: &OfferId<H>) -> Self
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		DealOrderId(Config::Hashing::hash(offer_id.0.as_ref()))
	}
}

pub type BalanceFor<T> = <T as pallet_balances::Config>::Balance;

macro_rules! try_get {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		crate::pallet::$storage::<$t>::try_get($key).map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, LockableCurrency, Randomness},
		Blake2_128Concat,
	};
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

		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		type Currency: Currency<Self::AccountId> + LockableCurrency<Self::AccountId>;

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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn authorities)]
	pub type Authorities<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	#[pallet::storage]
	#[pallet::getter(fn pending_transfers)]
	pub type PendingTransfers<T: Config> =
		StorageValue<_, Vec<PendingTransfer<T::AccountId, T::BlockNumber, T::Hash>>, ValueQuery>;

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
		AddressRegistered(AddressId<T::Hash>, Address<T::AccountId>),

		TransferRegistered(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),

		TransferFinalized(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),
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

		TransferAlreadyRegistered,

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
				log::debug!("Do thing");
				for PendingTransfer { verify_string, transfer } in PendingTransfers::<T>::get() {
					log::debug!("verifying transfer");
					// TODO: actually hit gateway to verify given transaction
					if let Err(e) = Self::offchain_signed_tx(auth_id.clone(), |_| {
						Call::finalize_transfer { transfer: transfer.clone() }
					}) {
						log::error!("Failed to send finalize transfer transaction: {:?}", e);
					}
				}
			} else {
				log::debug!("Not authority, skipping off chain work");
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_address(
			origin: OriginFor<T>,
			blockchain: Vec<u8>,
			address: Vec<u8>,
			network: Vec<u8>,
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
			guid: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order_id = AskOrderId::new::<T>(&guid);
			ensure!(!AskOrders::<T>::contains_key(&ask_order_id), Error::<T>::DuplicateId);

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

			AskOrders::<T>::insert(ask_order_id, ask_order);
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
			guid: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bid_order_id = BidOrderId::new::<T>(&guid);
			ensure!(!BidOrders::<T>::contains_key(&bid_order_id), Error::<T>::DuplicateId);

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

		#[pallet::weight(10_000)]
		pub fn add_offer(
			origin: OriginFor<T>,
			ask_order_id: AskOrderId<T::Hash>,
			bid_order_id: BidOrderId<T::Hash>,
			expiration: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order = try_get!(AskOrders<T>, &ask_order_id, NonExistentAskOrder)?;

			let bid_order = try_get!(BidOrders<T>, &bid_order_id, NonExistentBidOrder)?;

			let src_address = Self::get_address(&ask_order.address)?;

			// TODO: Do validation of addresses and parameters here

			let offer_id = OfferId::new::<T>(&ask_order_id, &bid_order_id);

			ensure!(!Offers::<T>::contains_key(&offer_id), Error::<T>::DuplicateOffer);

			let offer = Offer {
				ask_order: ask_order_id,
				bid_order: bid_order_id,
				block: Self::block_number(),
				blockchain: src_address.blockchain,
				expiration,
				sighash: who,
			};

			Offers::<T>::insert(offer_id, offer);

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_deal_order(
			origin: OriginFor<T>,
			offer_id: OfferId<T::Hash>,
			expiration: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let offer = try_get!(Offers<T>, &offer_id, NonExistentOffer)?;
			let ask_order = try_get!(AskOrders<T>, &offer.ask_order, NonExistentAskOrder)?;

			let bid_order = try_get!(BidOrders<T>, &offer.bid_order, NonExistentBidOrder)?;

			// TODO: checks to make sure orders match up

			let deal_order_id = DealOrderId::new::<T>(&offer_id);
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

			DealOrders::<T>::insert(deal_order_id, deal_order);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1))]
		pub fn register_transfer(
			origin: OriginFor<T>,
			gain: ExternalAmount,
			order_id: OrderId<T::Hash>,
			blockchain_tx_id: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (src_address_id, dest_address_id, mut amount) = match &order_id {
				OrderId::Deal(deal_order_id) => {
					let order = try_get!(DealOrders<T>, &deal_order_id, NonExistentDealOrder)?;

					if gain == 0 {
						(order.src_address, order.dst_address, order.amount)
					} else {
						(order.dst_address, order.src_address, order.amount)
					}
				},
				OrderId::Repayment(repay_order_id) => {
					ensure!(gain == 0, Error::<T>::RepaymentOrderNonZeroGain);
					let order =
						try_get!(RepaymentOrders<T>, &repay_order_id, NonExistentRepaymentOrder)?;
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

			if blockchain_tx_id == &*b"0" {
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
				Self::deposit_event(Event::<T>::TransferFinalized(
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
					&src_address.blockchain,
					b"verify",
					&src_address.value,
					&dest_address.value,
					&order_id.to_hex(),
					&*amount_str,
					&blockchain_tx_id;
					&src_address.network;
					sep = b' '
				);
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

				PendingTransfers::<T>::mutate(|transfers| transfers.push(pending));
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
		let signer = Signer::<T, T::AuthorityId>::any_account().with_filter(vec![public.into()]);
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
