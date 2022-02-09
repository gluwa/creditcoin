#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_system::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::{KeyTypeId, RuntimeAppPublic};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod ocw;
mod types;

pub use types::*;

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
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_timestamp::Config
		+ CreateSignedTransaction<Call<Self>>
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

		type UnverifiedTransferLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn authorities)]
	pub type Authorities<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	#[pallet::storage]
	#[pallet::getter(fn pending_transfers)]
	pub type UnverifiedTransfers<T: Config> = StorageValue<
		_,
		BoundedVec<
			UnverifiedTransfer<T::AccountId, T::BlockNumber, T::Hash>,
			T::UnverifiedTransferLimit,
		>,
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
		DealOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
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
		AskOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn bid_orders)]
	pub type BidOrders<T: Config> = StorageDoubleMap<
		_,
		Twox128,
		T::BlockNumber,
		Identity,
		T::Hash,
		BidOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
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

		TransferVerified(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),

		AskOrderAdded(
			AskOrderId<T::BlockNumber, T::Hash>,
			AskOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
		),

		BidOrderAdded(
			BidOrderId<T::BlockNumber, T::Hash>,
			BidOrder<T::AccountId, T::Balance, T::BlockNumber, T::Hash, T::Moment>,
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

		UnsupportedTransferKind,

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

		UnverifiedTransferPoolFull,

		VerifyStringTooLong,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { authorities: Vec::new() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for authority in self.authorities.iter() {
				Authorities::<T>::insert(authority.clone(), ());
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_block_number: T::BlockNumber) -> Weight {
			UnverifiedTransfers::<T>::kill();
			0
		}
		fn offchain_worker(_block_number: T::BlockNumber) {
			if let Some(auth_id) = Self::authority_id() {
				let auth_id = T::FromAccountId::from(auth_id);
				for pending in UnverifiedTransfers::<T>::get() {
					log::debug!("verifying transfer");
					let transfer_validity = Self::verify_transfer_ocw(&pending);
					log::debug!("verify_transfer result: {:?}", transfer_validity);
					match transfer_validity {
						Ok(()) =>
							if let Err(e) = Self::offchain_signed_tx(auth_id.clone(), |_| {
								Call::verify_transfer { transfer: pending.transfer.clone() }
							}) {
								log::error!(
									"Failed to send finalize transfer transaction: {:?}",
									e
								);
							},
						Err(err) => {
							log::warn!(
								"failed to verify pending transfer {:?}: {:?}",
								pending,
								err
							);
						},
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
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let address_id = AddressId::new::<T>(&blockchain, &address);
			ensure!(
				!Addresses::<T>::contains_key(&address_id),
				Error::<T>::AddressAlreadyRegistered
			);

			let entry = Address { blockchain, value: address, owner: who };
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
			maturity: T::Moment,
			fee: BalanceFor<T>,
			expiration: BlockNumberFor<T>,
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order_id = AskOrderId::new::<T>(Self::block_number() + expiration, &guid);
			ensure!(!AskOrders::<T>::contains_id(&ask_order_id), Error::<T>::DuplicateId);

			let address = Self::get_address(&address_id)?;
			ensure!(address.owner == who, Error::<T>::NotAddressOwner);
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
			maturity: T::Moment,
			fee: BalanceFor<T>,
			expiration: BlockNumberFor<T>,
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bid_order_id = BidOrderId::new::<T>(Self::block_number() + expiration, &guid);
			ensure!(!BidOrders::<T>::contains_id(&bid_order_id), Error::<T>::DuplicateId);

			let address = Self::addresses(&address_id);
			if let Some(address) = address {
				ensure!(address.owner == who, Error::<T>::NotAddressOwner);
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

			let lender_address = Self::get_address(&ask_order.address)?;

			// TODO: Do validation of addresses and parameters here

			let offer_id =
				OfferId::new::<T>(Self::block_number() + expiration, &ask_order_id, &bid_order_id);

			ensure!(!Offers::<T>::contains_id(&offer_id), Error::<T>::DuplicateOffer);

			let offer = Offer {
				ask_order: ask_order_id,
				bid_order: bid_order_id,
				block: Self::block_number(),
				blockchain: lender_address.blockchain,
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
				lender: ask_order.address,
				borrower: bid_order.address,
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
							try_get!(Addresses<T>, &deal_order.lender, NonExistentAddress)?;

						ensure!(src_address.owner == who, Error::<T>::NotInvestor);

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
			transfer_kind: TransferKind,
			gain: ExternalAmount,
			order_id: OrderId<T::BlockNumber, T::Hash>,
			blockchain_tx_id: ExternalTxId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (from_id, to_id, mut amount) = match &order_id {
				OrderId::Deal(deal_order_id) => {
					let order = try_get_id!(DealOrders<T>, &deal_order_id, NonExistentDealOrder)?;

					if gain.is_zero() {
						// transfer for initial loan
						(order.lender, order.borrower, order.amount)
					} else {
						// transfer to repay loan
						(order.borrower, order.lender, order.amount)
					}
				},
				OrderId::Repayment(repay_order_id) => {
					ensure!(gain.is_zero(), Error::<T>::RepaymentOrderNonZeroGain);
					let order = try_get_id!(
						RepaymentOrders<T>,
						&repay_order_id,
						NonExistentRepaymentOrder
					)?;
					(order.src_address, order.dst_address, order.amount)
				},
			};

			let from = Self::get_address(&from_id)?;
			let to = Self::get_address(&to_id)?;

			ensure!(from.owner == who, Error::<T>::NotAddressOwner);

			ensure!(from.blockchain == to.blockchain, Error::<T>::AddressPlatformMismatch);

			ensure!(from.blockchain.supports(&transfer_kind), Error::<T>::UnsupportedTransferKind);

			let transfer_id = TransferId::new::<T>(&from.blockchain, &blockchain_tx_id);
			ensure!(
				!Transfers::<T>::contains_key(&transfer_id),
				Error::<T>::TransferAlreadyRegistered
			);

			if &*blockchain_tx_id == &*b"0" {
				// this transfer is an exemption, no need to verify it
				amount = ExternalAmount::zero();
				let transfer = Transfer {
					blockchain: from.blockchain,
					kind: transfer_kind,
					amount,
					block: <frame_system::Pallet<T>>::block_number(),
					from: from_id,
					to: to_id,
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
				let transfer = Transfer {
					blockchain: from.blockchain,
					kind: transfer_kind,
					amount,
					block: <frame_system::Pallet<T>>::block_number(),
					from: from_id,
					to: to_id,
					order: order_id,
					processed: false,
					sighash: who.clone(),
					tx: blockchain_tx_id,
				};

				Self::deposit_event(Event::<T>::TransferRegistered(
					transfer_id.clone(),
					transfer.clone(),
				));

				let pending = UnverifiedTransfer {
					from_external: from.value.clone(),
					to_external: to.value.clone(),
					transfer,
				};
				UnverifiedTransfers::<T>::try_mutate(|transfers| transfers.try_push(pending))
					.map_err(|()| Error::<T>::UnverifiedTransferPoolFull)?;
			}

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn verify_transfer(
			origin: OriginFor<T>,
			transfer: Transfer<T::AccountId, T::BlockNumber, T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Authorities::<T>::contains_key(&who), Error::<T>::InsufficientAuthority);

			let key = TransferId::new::<T>(&transfer.blockchain, &transfer.tx);
			ensure!(!Transfers::<T>::contains_key(&key), Error::<T>::TransferAlreadyRegistered);
			let mut transfer = transfer;
			transfer.block = frame_system::Pallet::<T>::block_number();

			Self::deposit_event(Event::<T>::TransferVerified(key.clone(), transfer.clone()));
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

	// fn create_bid_order(
	// 	who: &T::AccountId,
	// 	bid_order_id: &BidOrderId<T::BlockNumber, T::Hash>,
	// 	address_id: &AddressId<T::Hash>,
	// ) -> DispatchResult {
	// 	ensure!(!BidOrders::<T>::contains_id(&bid_order_id), Error::<T>::DuplicateId);

	// 	let address = Self::get_address(&address_id)?;
	// 	ensure!(address.owner == who, Error::<T>::NotAddressOwner);
	// 	let bid_order = BidOrder {
	// 		blockchain: address.blockchain,
	// 		address: address_id,
	// 		amount,
	// 		interest,
	// 		maturity,
	// 		fee,
	// 		expiration,
	// 		block: <frame_system::Pallet<T>>::block_number(),
	// 		sighash: who,
	// 	};
	// }
}
