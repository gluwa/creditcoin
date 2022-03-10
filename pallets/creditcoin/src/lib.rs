#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;
use sp_runtime::KeyTypeId;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[macro_use]
mod helpers;
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

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{tokens::ExistenceRequirement, Currency},
		weights::PostDispatchInfo,
	};
	use frame_system::{
		ensure_signed,
		offchain::{AppCrypto, CreateSignedTransaction},
		pallet_prelude::*,
	};
	use sp_runtime::traits::{IdentifyAccount, UniqueSaturatedInto, Verify};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_timestamp::Config
		+ CreateSignedTransaction<Call<Self>>
	where
		<Self as frame_system::Config>::BlockNumber: UniqueSaturatedInto<u64>,
		<Self as pallet_timestamp::Config>::Moment: UniqueSaturatedInto<u64>,
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Call: From<Call<Self>>;

		type AuthorityId: AppCrypto<
			Self::Public,
			<Self as frame_system::offchain::SigningTypes>::Signature,
		>;

		type Signer: From<sp_core::ecdsa::Public>
			+ IdentifyAccount<AccountId = <Self as frame_system::Config>::AccountId>;

		type FromAccountId: From<sp_core::sr25519::Public>
			+ IsType<Self::AccountId>
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
	pub type LegacyWallets<T: Config> = StorageMap<_, Twox128, LegacySighash, T::Balance>;

	#[pallet::storage]
	pub type LegacyBalanceKeeper<T: Config> = StorageValue<_, T::AccountId>;

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
		Twox64Concat,
		T::BlockNumber,
		Identity,
		T::Hash,
		DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn addresses)]
	pub type Addresses<T: Config> =
		StorageMap<_, Blake2_128Concat, AddressId<T::Hash>, Address<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn used_guids)]
	pub type UsedGuids<T: Config> = StorageMap<_, Blake2_128Concat, Guid, ()>;

	#[pallet::storage]
	#[pallet::getter(fn ask_orders)]
	pub type AskOrders<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Identity,
		T::Hash,
		AskOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn bid_orders)]
	pub type BidOrders<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Identity,
		T::Hash,
		BidOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers)]
	pub type Offers<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
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
		TransferProcessed(TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash>),

		AskOrderAdded(
			AskOrderId<T::BlockNumber, T::Hash>,
			AskOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
		),

		BidOrderAdded(
			BidOrderId<T::BlockNumber, T::Hash>,
			BidOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
		),

		OfferAdded(OfferId<T::BlockNumber, T::Hash>, Offer<T::AccountId, T::BlockNumber, T::Hash>),

		DealOrderAdded(
			DealOrderId<T::BlockNumber, T::Hash>,
			DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
		),
		DealOrderFunded(
			DealOrderId<T::BlockNumber, T::Hash>,
			DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
		),
		DealOrderClosed(
			DealOrderId<T::BlockNumber, T::Hash>,
			DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
		),

		LoanExempted(DealOrderId<T::BlockNumber, T::Hash>, TransferId<T::Hash>),

		LegacyWalletClaimed(T::AccountId, LegacySighash, T::Balance),
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
		TransferAmountInsufficient,
		MalformedTransfer,

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

		DealNotFunded,

		DealOrderAlreadyFunded,
		DealOrderAlreadyClosed,
		DealOrderAlreadyLocked,
		DealOrderMustBeLocked,
		DuplicateDealOrder,
		DealOrderExpired,

		AskOrderExpired,
		BidOrderExpired,
		OfferExpired,
		AskBidMismatch,

		SameOwner,
		InvalidSignature,

		NotBorrower,

		MalformedDealOrder,

		NotLender,

		ScaleDecodeError,

		UnverifiedTransferPoolFull,

		RepaymentOrderUnsupported,

		NotLegacyWalletOwner,
		LegacySighashMalformed,
		LegacyWalletNotFound,
		LegacyBalanceKeeperMissing,

		VerifyStringTooLong,

		GuidAlreadyUsed,

		InvalidMaturity,
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::AccountId>,
		pub legacy_wallets: Vec<(LegacySighash, T::Balance)>,
		pub legacy_balance_keeper: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				authorities: Vec::new(),
				legacy_wallets: Vec::new(),
				legacy_balance_keeper: None,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for authority in &self.authorities {
				Authorities::<T>::insert(authority.clone(), ());
			}
			for (sighash, balance) in &self.legacy_wallets {
				LegacyWallets::<T>::insert(sighash, balance);
			}
			if let Some(acct) = &self.legacy_balance_keeper {
				LegacyBalanceKeeper::<T>::put(acct.clone());
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
						Ok(()) => {
							if let Err(e) = Self::offchain_signed_tx(auth_id.clone(), |_| {
								Call::verify_transfer { transfer: pending.transfer.clone() }
							}) {
								log::error!(
									"Failed to send finalize transfer transaction: {:?}",
									e
								);
							}
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
			Offers::<T>::remove_prefix(block_number, None);
			let deals_to_keep: Vec<_> = DealOrders::<T>::drain_prefix(block_number)
				.filter_map(|(hash, deal)| {
					if deal.funding_transfer_id.is_some() {
						Some((DealOrderId::with_expiration_hash::<T>(block_number, hash), deal))
					} else {
						None
					}
				})
				.collect();

			for (key, deal) in deals_to_keep {
				DealOrders::<T>::insert_id(key, deal);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claims legacy wallet and transfers the balance to the sender's account.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn claim_legacy_wallet(
			origin: OriginFor<T>,
			public_key: sp_core::ecdsa::Public,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let account_id_of_key = T::Signer::from(public_key.clone()).into_account();
			ensure!(account_id_of_key == who, Error::<T>::NotLegacyWalletOwner);

			let sighash = LegacySighash::from(&public_key);

			let legacy_balance =
				LegacyWallets::<T>::get(&sighash).ok_or(Error::<T>::LegacyWalletNotFound)?;

			let legacy_keeper =
				LegacyBalanceKeeper::<T>::get().ok_or(Error::<T>::LegacyBalanceKeeperMissing)?;

			<pallet_balances::Pallet<T> as Currency<T::AccountId>>::transfer(
				&legacy_keeper,
				&who,
				legacy_balance,
				ExistenceRequirement::AllowDeath,
			)?;
			LegacyWallets::<T>::remove(&sighash);
			Self::deposit_event(Event::<T>::LegacyWalletClaimed(who, sighash, legacy_balance));

			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::No })
		}

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

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,2))]
		pub fn add_ask_order(
			origin: OriginFor<T>,
			address_id: AddressId<T::Hash>,
			terms: LoanTerms<T::Moment>,
			expiration_block: BlockNumberFor<T>,
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order_id = AskOrderId::new::<T>(expiration_block, &guid);
			ensure!(!AskOrders::<T>::contains_id(&ask_order_id), Error::<T>::DuplicateId);

			let address = Self::get_address(&address_id)?;
			ensure!(address.owner == who, Error::<T>::NotAddressOwner);

			let ask_order = AskOrder {
				blockchain: address.blockchain,
				lender_address_id: address_id,
				terms: terms.try_into().map_err(Error::<T>::from)?,
				expiration_block,
				block: <frame_system::Pallet<T>>::block_number(),
				lender: who,
			};

			Self::use_guid(&guid)?;
			sp_io::offchain_index::set(&guid, &ask_order.encode());
			Self::deposit_event(Event::<T>::AskOrderAdded(ask_order_id.clone(), ask_order.clone()));
			AskOrders::<T>::insert_id(ask_order_id, ask_order);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,2))]
		pub fn add_bid_order(
			origin: OriginFor<T>,
			address_id: AddressId<T::Hash>,
			terms: LoanTerms<T::Moment>,
			expiration_block: BlockNumberFor<T>,
			guid: Guid,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bid_order_id = BidOrderId::new::<T>(expiration_block, &guid);
			ensure!(!BidOrders::<T>::contains_id(&bid_order_id), Error::<T>::DuplicateId);

			let address = Self::get_address(&address_id)?;
			ensure!(address.owner == who, Error::<T>::NotAddressOwner);
			let bid_order = BidOrder {
				blockchain: address.blockchain,
				borrower_address_id: address_id,
				terms: terms.try_into().map_err(Error::<T>::from)?,
				expiration_block,
				block: <frame_system::Pallet<T>>::block_number(),
				borrower: who,
			};

			Self::use_guid(&guid)?;
			sp_io::offchain_index::set(&guid, &bid_order.encode());

			Self::deposit_event(Event::<T>::BidOrderAdded(bid_order_id.clone(), bid_order.clone()));
			BidOrders::<T>::insert_id(bid_order_id, bid_order);
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
		pub fn add_offer(
			origin: OriginFor<T>,
			ask_order_id: AskOrderId<T::BlockNumber, T::Hash>,
			bid_order_id: BidOrderId<T::BlockNumber, T::Hash>,
			expiration_block: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let ask_order = try_get_id!(AskOrders<T>, &ask_order_id, NonExistentAskOrder)?;

			ensure!(ask_order.lender == who, Error::<T>::NotLender);

			let head = Self::block_number();

			ensure!(ask_order.expiration_block >= head, Error::<T>::AskOrderExpired);

			let bid_order = try_get_id!(BidOrders<T>, &bid_order_id, NonExistentBidOrder)?;

			ensure!(bid_order.borrower != who, Error::<T>::SameOwner);

			ensure!(bid_order.expiration_block >= head, Error::<T>::BidOrderExpired);

			ensure!(
				ask_order.blockchain == bid_order.blockchain,
				Error::<T>::AddressPlatformMismatch
			);

			ensure!(ask_order.terms.match_with(&bid_order.terms), Error::<T>::AskBidMismatch);

			let offer_id = OfferId::new::<T>(expiration_block, &ask_order_id, &bid_order_id);

			ensure!(!Offers::<T>::contains_id(&offer_id), Error::<T>::DuplicateOffer);

			let offer = Offer {
				ask_id: ask_order_id,
				bid_id: bid_order_id,
				block: Self::block_number(),
				blockchain: ask_order.blockchain,
				expiration_block,
				lender: who,
			};

			Self::deposit_event(Event::<T>::OfferAdded(offer_id.clone(), offer.clone()));
			Offers::<T>::insert_id(offer_id, offer);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
		pub fn add_deal_order(
			origin: OriginFor<T>,
			offer_id: OfferId<T::BlockNumber, T::Hash>,
			expiration_block: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let deal_order_id = DealOrderId::new::<T>(expiration_block, &offer_id);
			ensure!(!DealOrders::<T>::contains_id(&deal_order_id), Error::<T>::DuplicateDealOrder);

			let offer = try_get_id!(Offers<T>, &offer_id, NonExistentOffer)?;

			let head = Self::block_number();

			ensure!(offer.expiration_block >= head, Error::<T>::OfferExpired);

			let ask_order = try_get_id!(AskOrders<T>, &offer.ask_id, NonExistentAskOrder)?;

			let bid_order = try_get_id!(BidOrders<T>, &offer.bid_id, NonExistentBidOrder)?;

			ensure!(bid_order.borrower == who, Error::<T>::NotBorrower);

			let agreed_terms = ask_order
				.terms
				.agreed_terms(bid_order.terms)
				.ok_or(Error::<T>::AskBidMismatch)?;

			let deal_order = DealOrder {
				blockchain: offer.blockchain,
				offer_id,
				lender_address_id: ask_order.lender_address_id,
				borrower_address_id: bid_order.borrower_address_id,
				terms: agreed_terms,
				expiration_block,
				timestamp: Self::timestamp(),
				borrower: who,
				funding_transfer_id: None,
				lock: None,
				repayment_transfer_id: None,
			};

			Self::deposit_event(Event::<T>::DealOrderAdded(
				deal_order_id.clone(),
				deal_order.clone(),
			));
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
				|value| -> DispatchResult {
					let deal_order = value.as_mut().ok_or(Error::<T>::NonExistentDealOrder)?;

					ensure!(deal_order.lock.is_none(), Error::<T>::DealOrderAlreadyLocked);
					ensure!(deal_order.funding_transfer_id.is_some(), Error::<T>::DealNotFunded);
					ensure!(deal_order.borrower == who, Error::<T>::NotBorrower);

					deal_order.lock = Some(who);
					Ok(())
				},
			)?;

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,2))]
		pub fn fund_deal_order(
			origin: OriginFor<T>,
			deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
			transfer_id: TransferId<T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::try_mutate_deal_order_and_transfer(
				&deal_order_id,
				&transfer_id,
				|deal_order| {
					let lender =
						try_get!(Addresses<T>, &deal_order.lender_address_id, NonExistentAddress)?;

					ensure!(lender.owner == who, Error::<T>::NotLender);

					let now = Self::timestamp();
					ensure!(now >= deal_order.timestamp, Error::<T>::MalformedDealOrder);

					ensure!(
						deal_order.funding_transfer_id.is_none(),
						Error::<T>::DealOrderAlreadyFunded
					);
					let head = Self::block_number();
					ensure!(deal_order.expiration_block >= head, Error::<T>::DealOrderExpired);

					deal_order.funding_transfer_id = Some(transfer_id.clone());
					deal_order.timestamp = now;

					Ok(Some(Event::<T>::DealOrderFunded(deal_order_id.clone(), deal_order.clone())))
				},
				|transfer, deal_order| {
					ensure!(
						transfer.order_id == OrderId::Deal(deal_order_id.clone()),
						Error::<T>::TransferMismatch
					);
					ensure!(
						transfer.amount == deal_order.terms.amount,
						Error::<T>::TransferMismatch
					);
					ensure!(transfer.sighash == who, Error::<T>::TransferMismatch);
					ensure!(!transfer.processed, Error::<T>::TransferAlreadyProcessed);

					transfer.processed = true;
					Ok(Some(Event::<T>::TransferProcessed(transfer_id.clone(), transfer.clone())))
				},
			)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn register_deal_order(
			origin: OriginFor<T>,
			lender_address_id: AddressId<T::Hash>,
			borrower_address_id: AddressId<T::Hash>,
			terms: LoanTerms<T::Moment>,
			expiration_block: BlockNumberFor<T>,
			ask_guid: Guid,
			bid_guid: Guid,
			borrower_key: sp_core::ecdsa::Public,
			borrower_signature: sp_core::ecdsa::Signature,
		) -> DispatchResult {
			let lender_account = ensure_signed(origin)?;
			let borrower_account = T::Signer::from(borrower_key.clone()).into_account();

			let message =
				Self::register_deal_order_message(expiration_block, &ask_guid, &bid_guid, &terms);

			ensure!(
				borrower_signature.verify(message.as_slice(), &borrower_key),
				Error::<T>::InvalidSignature
			);

			let borrower = Self::get_address(&borrower_address_id)?;
			ensure!(borrower.owner == borrower_account, Error::<T>::NotAddressOwner);

			let lender = Self::get_address(&lender_address_id)?;
			ensure!(lender.owner == lender_account, Error::<T>::NotAddressOwner);

			ensure!(lender.matches_chain_of(&borrower), Error::<T>::AddressPlatformMismatch);

			let ask_order_id = AskOrderId::new::<T>(expiration_block, &ask_guid);
			ensure!(!AskOrders::<T>::contains_id(&ask_order_id), Error::<T>::DuplicateId);

			let bid_order_id = BidOrderId::new::<T>(expiration_block, &bid_guid);
			ensure!(!BidOrders::<T>::contains_id(&bid_order_id), Error::<T>::DuplicateId);

			let offer_id = OfferId::new::<T>(expiration_block, &ask_order_id, &bid_order_id);
			ensure!(!Offers::<T>::contains_id(&offer_id), Error::<T>::DuplicateOffer);

			let deal_order_id = DealOrderId::new::<T>(expiration_block, &offer_id);
			ensure!(!DealOrders::<T>::contains_id(&deal_order_id), Error::<T>::DuplicateDealOrder);

			let current_block = Self::block_number();

			let ask_order = AskOrder {
				blockchain: lender.blockchain.clone(),
				lender_address_id: lender_address_id.clone(),
				terms: terms.clone().try_into().map_err(Error::<T>::from)?,
				expiration_block,
				block: current_block,
				lender: lender_account.clone(),
			};

			let bid_order = BidOrder {
				blockchain: lender.blockchain.clone(),
				borrower_address_id: borrower_address_id.clone(),
				terms: terms.clone().try_into().map_err(Error::<T>::from)?,
				expiration_block,
				block: current_block,
				borrower: borrower_account.clone(),
			};

			let offer = Offer {
				ask_id: ask_order_id.clone(),
				bid_id: bid_order_id.clone(),
				block: current_block,
				blockchain: lender.blockchain.clone(),
				expiration_block,
				lender: lender_account,
			};

			let deal_order = DealOrder {
				blockchain: lender.blockchain,
				offer_id: offer_id.clone(),
				lender_address_id,
				borrower_address_id,
				terms,
				expiration_block,
				timestamp: Self::timestamp(),
				borrower: borrower_account,
				funding_transfer_id: None,
				lock: None,
				repayment_transfer_id: None,
			};

			AskOrders::<T>::insert_id(ask_order_id.clone(), ask_order.clone());
			Self::deposit_event(Event::<T>::AskOrderAdded(ask_order_id, ask_order));

			BidOrders::<T>::insert_id(bid_order_id.clone(), bid_order.clone());
			Self::deposit_event(Event::<T>::BidOrderAdded(bid_order_id, bid_order));

			Offers::<T>::insert_id(offer_id.clone(), offer.clone());
			Self::deposit_event(Event::<T>::OfferAdded(offer_id, offer));

			DealOrders::<T>::insert_id(deal_order_id.clone(), deal_order.clone());
			Self::deposit_event(Event::<T>::DealOrderAdded(deal_order_id, deal_order));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(4,2))]
		pub fn close_deal_order(
			origin: OriginFor<T>,
			deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
			transfer_id: TransferId<T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::try_mutate_deal_order_and_transfer(
				&deal_order_id,
				&transfer_id,
				|deal_order| {
					let borrower = try_get!(
						Addresses<T>,
						&deal_order.borrower_address_id,
						NonExistentAddress
					)?;

					ensure!(borrower.owner == who, Error::<T>::NotBorrower);

					let now = Self::timestamp();
					ensure!(now >= deal_order.timestamp, Error::<T>::MalformedDealOrder);

					ensure!(
						deal_order.repayment_transfer_id.is_none(),
						Error::<T>::DealOrderAlreadyClosed
					);

					ensure!(deal_order.lock.is_some(), Error::<T>::DealOrderMustBeLocked);

					deal_order.repayment_transfer_id = Some(transfer_id.clone());

					Ok(Some(Event::<T>::DealOrderClosed(deal_order_id.clone(), deal_order.clone())))
				},
				|transfer, deal_order| {
					ensure!(
						transfer.order_id == OrderId::Deal(deal_order_id.clone()),
						Error::<T>::TransferMismatch
					);

					ensure!(transfer.block <= Self::block_number(), Error::<T>::MalformedTransfer);
					ensure!(transfer.sighash == who, Error::<T>::TransferMismatch);
					ensure!(!transfer.processed, Error::<T>::TransferAlreadyProcessed);

					//TODO: add compound interest formula
					let expected_interest = deal_order.terms.calc_interest();

					ensure!(
						transfer.amount >= expected_interest + deal_order.terms.amount,
						Error::<T>::TransferAmountInsufficient
					);

					transfer.processed = true;
					Ok(Some(Event::<T>::TransferProcessed(transfer_id.clone(), transfer.clone())))
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
						(order.lender_address_id, order.borrower_address_id, order.terms.amount)
					} else {
						// transfer to repay loan
						(order.borrower_address_id, order.lender_address_id, order.terms.amount)
					}
				},
				OrderId::Repayment(_) => {
					ensure!(gain.is_zero(), Error::<T>::RepaymentOrderNonZeroGain);
					return Err(Error::<T>::RepaymentOrderUnsupported.into());
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

			if *blockchain_tx_id == *b"0" {
				// this transfer is an exemption, no need to verify it
				amount = ExternalAmount::zero();
				let transfer = Transfer {
					blockchain: from.blockchain,
					kind: transfer_kind,
					amount,
					block: <frame_system::Pallet<T>>::block_number(),
					from: from_id,
					to: to_id,
					order_id,
					processed: false,
					sighash: who,
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
					order_id,
					processed: false,
					sighash: who,
					tx: blockchain_tx_id,
				};

				Self::deposit_event(Event::<T>::TransferRegistered(transfer_id, transfer.clone()));

				let pending = UnverifiedTransfer {
					from_external: from.value,
					to_external: to.value,
					transfer,
				};
				UnverifiedTransfers::<T>::try_mutate(|transfers| transfers.try_push(pending))
					.map_err(|()| Error::<T>::UnverifiedTransferPoolFull)?;
			}

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 2))]
		pub fn exempt(
			origin: OriginFor<T>,
			deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
			transfer_id: TransferId<T::Hash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::try_mutate_deal_order_and_transfer(
				&deal_order_id,
				&transfer_id,
				|deal_order| {
					ensure!(
						deal_order.repayment_transfer_id.is_some(),
						Error::<T>::DealOrderAlreadyClosed
					);

					let lender = Self::get_address(&deal_order.lender_address_id)?;
					ensure!(who == lender.owner, Error::<T>::NotLender);

					deal_order.repayment_transfer_id = Some(transfer_id.clone());

					Ok(None)
				},
				|transfer, _| {
					ensure!(
						transfer.order_id == OrderId::Deal(deal_order_id.clone()),
						Error::<T>::TransferMismatch
					);

					ensure!(!transfer.processed, Error::<T>::TransferAlreadyProcessed);

					transfer.processed = true;

					Ok(None)
				},
			)?;

			Self::deposit_event(Event::<T>::LoanExempted(deal_order_id, transfer_id));
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
