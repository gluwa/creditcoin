use super::v5;
use super::{AccountIdOf, BlockNumberOf, HashOf, MomentOf};
use crate::Config;
use crate::EvmCurrencyType;
use crate::EvmInfo;
use crate::EvmSupportedTransferKinds;
use crate::EvmTransferKind;
use crate::Id;
use core::convert::TryFrom;
use frame_support::pallet_prelude::*;
use frame_support::storage_alias;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::prelude::*;

pub use v5::*;

pub use v5::Address as OldAddress;
pub use v5::AskOrder as OldAskOrder;
pub use v5::AskTerms as OldAskTerms;
pub use v5::BidOrder as OldBidOrder;
pub use v5::BidTerms as OldBidTerms;
pub use v5::Blockchain as OldBlockchain;
pub use v5::DealOrder as OldDealOrder;
pub use v5::LoanTerms as OldLoanTerms;
pub use v5::OrderId as OldOrderId;
pub use v5::Task as OldTask;
pub use v5::Transfer as OldTransfer;
pub use v5::TransferKind as OldTransferKind;
pub use v5::UnverifiedCollectedCoinsStruct as OldUnverifiedCollectedCoins;
pub use v5::UnverifiedTransfer as OldUnverifiedTransfer;

use crate::Address;
use crate::AddressId;
use crate::AskOrder;
use crate::AskOrderId;
use crate::AskTerms;
use crate::BidOrder;
use crate::BidOrderId;
use crate::BidTerms;
use crate::Blockchain;
use crate::Currency;
use crate::CurrencyId;
use crate::DealOrder;
use crate::LegacyTransferKind;
use crate::LoanTerms;
use crate::Task;
use crate::TaskId;
use crate::Transfer;
use crate::TransferId;
use crate::TransferKind;
use crate::UnverifiedCollectedCoins;
use crate::UnverifiedTransfer;

fn translate_blockchain(old: OldBlockchain) -> Option<Blockchain> {
	match old {
		OldBlockchain::Ethereum => Some(Blockchain::ETHEREUM),
		OldBlockchain::Rinkeby => Some(Blockchain::RINKEBY),
		// this assumes that Luniverse == mainnet luniverse, we may want to make the chain ID of the
		// old "Luniverse" variant on-chain-storage to make testnet work
		OldBlockchain::Luniverse => Some(Blockchain::LUNIVERSE),
		other => {
			log::warn!(
				"unexpected blockchain found on storage item: {:?}",
				core::str::from_utf8(other.as_bytes()).ok()
			);
			None
		},
	}
}

fn translate_loan_terms<T: Config>(
	old: OldLoanTerms,
	currency: CurrencyId<HashOf<T>>,
) -> LoanTerms<HashOf<T>> {
	LoanTerms {
		amount: old.amount,
		interest_rate: old.interest_rate,
		term_length: old.term_length,
		currency,
	}
}

fn translate_transfer_kind(old: OldTransferKind) -> Option<TransferKind> {
	Some(match old {
		OldTransferKind::Ethless(_) => TransferKind::Evm(EvmTransferKind::Ethless),
		OldTransferKind::Erc20(_) => TransferKind::Evm(EvmTransferKind::Erc20),
		other => {
			log::warn!("unexpected transfer kind found on storage item: {:?}", other);
			return None;
		},
	})
}

fn reconstruct_currency(blockchain: &OldBlockchain, kind: &OldTransferKind) -> Option<Currency> {
	let info = match blockchain {
		OldBlockchain::Ethereum => EvmInfo::ETHEREUM,
		OldBlockchain::Rinkeby => EvmInfo::RINKEBY,
		OldBlockchain::Luniverse => EvmInfo::LUNIVERSE,
		other => {
			log::warn!(
				"unexpected blockchain found on storage item: {:?}",
				core::str::from_utf8(other.as_bytes()).ok()
			);
			return None;
		},
	};
	let currency_type = match kind {
		OldTransferKind::Erc20(addr) => EvmCurrencyType::SmartContract(
			addr.clone(),
			EvmSupportedTransferKinds::try_from(vec![EvmTransferKind::Erc20])
				.expect("1 is less than the bound (2); qed"),
		),
		OldTransferKind::Ethless(addr) => EvmCurrencyType::SmartContract(
			addr.clone(),
			EvmSupportedTransferKinds::try_from(vec![EvmTransferKind::Ethless])
				.expect("1 is less than the bound (2); qed"),
		),
		other => {
			log::warn!("unexpected transfer kind found in storage: {:?}", other);
			return None;
		},
	};
	Some(Currency::Evm(currency_type, info))
}

fn reconstruct_currency_from_deal<T: Config>(
	deal_order: &OldDealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
) -> Option<Currency> {
	let transfer_id = deal_order.funding_transfer_id.as_ref()?;
	let transfer = OldTransfers::<T>::get(transfer_id)?;
	let currency = reconstruct_currency(&deal_order.blockchain, &transfer.kind)?;
	Some(currency)
}

fn translate_transfer<T: Config>(
	transfer: OldTransfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
) -> Option<Transfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>> {
	Some(Transfer {
		amount: transfer.amount,
		from: transfer.from,
		to: transfer.to,
		tx_id: transfer.tx_id,
		block: transfer.block,
		is_processed: transfer.is_processed,
		account_id: transfer.account_id,
		timestamp: transfer.timestamp,
		deal_order_id: match transfer.order_id {
			OldOrderId::Deal(id) => id,
			OldOrderId::Repayment(id) => {
				log::warn!("Found unexpected repayment ID attached to a transfer: {:?}", id);
				return None;
			},
		},
		blockchain: translate_blockchain(transfer.blockchain)?,
		kind: translate_transfer_kind(transfer.kind)?,
	})
}

fn to_legacy_transfer_kind(transfer_kind: OldTransferKind) -> LegacyTransferKind {
	match transfer_kind {
		OldTransferKind::Erc20(addr) => LegacyTransferKind::Erc20(addr),
		OldTransferKind::Ethless(addr) => LegacyTransferKind::Ethless(addr),
		OldTransferKind::Native => LegacyTransferKind::Native,
		OldTransferKind::Other(other) => LegacyTransferKind::Other(other),
	}
}

#[storage_alias]
type Transfers<T: Config> = StorageMap<
	crate::Pallet<T>,
	Identity,
	TransferId<HashOf<T>>,
	Transfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

struct OldTransfersInstance;
impl frame_support::traits::StorageInstance for OldTransfersInstance {
	fn pallet_prefix() -> &'static str {
		"Creditcoin"
	}
	const STORAGE_PREFIX: &'static str = "Transfers";
}
#[allow(type_alias_bounds)]
type OldTransfers<T: Config> = frame_support::storage::types::StorageMap<
	OldTransfersInstance,
	Identity,
	TransferId<HashOf<T>>,
	OldTransfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

#[storage_alias]
type Addresses<T: Config> =
	StorageMap<crate::Pallet<T>, Blake2_128Concat, AddressId<HashOf<T>>, Address<AccountIdOf<T>>>;

#[storage_alias]
type PendingTasks<T: Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Identity,
	BlockNumberOf<T>,
	Identity,
	TaskId<HashOf<T>>,
	Task<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

#[frame_support::storage_alias]
type AskOrders<T: crate::Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Twox64Concat,
	BlockNumberOf<T>,
	Identity,
	HashOf<T>,
	AskOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>,
>;

#[frame_support::storage_alias]
type BidOrders<T: crate::Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Twox64Concat,
	BlockNumberOf<T>,
	Identity,
	HashOf<T>,
	BidOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>,
>;

#[frame_support::storage_alias]
type DealOrders<T: crate::Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Twox64Concat,
	BlockNumberOf<T>,
	Identity,
	HashOf<T>,
	DealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = Weight::zero();
	let weight_each = T::DbWeight::get().reads_writes(1, 1);
	let write = T::DbWeight::get().writes(1);
	let read = T::DbWeight::get().reads(1);

	let mut reconstructed_currency_ask = BTreeMap::new();
	let mut reconstructed_currency_bid = BTreeMap::new();
	let mut currencies = BTreeSet::new();

	DealOrders::<T>::translate::<
		OldDealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_exp, _hash, deal_order| {
		weight = weight.saturating_add(weight_each);

		let currency = reconstruct_currency_from_deal::<T>(&deal_order);
		let currency_id = if let Some(currency) = currency.as_ref() {
			currency.to_id::<T>()
		} else {
			CurrencyId::placeholder()
		};

		weight = weight.saturating_add(read);
		let offer = if let Some(offer) =
			crate::Offers::<T>::get(deal_order.offer_id.expiration(), deal_order.offer_id.hash())
		{
			offer
		} else {
			log::warn!("deal order has a non-existent offer: {:?}", deal_order.offer_id);
			return None;
		};

		if let Some(currency) = currency {
			reconstructed_currency_ask.insert(offer.ask_id, currency_id.clone());
			reconstructed_currency_bid.insert(offer.bid_id, currency_id.clone());
			currencies.insert((currency_id.clone(), currency));
		}

		Some(DealOrder {
			offer_id: deal_order.offer_id,
			lender_address_id: deal_order.lender_address_id,
			borrower_address_id: deal_order.borrower_address_id,
			terms: translate_loan_terms::<T>(deal_order.terms, currency_id),
			expiration_block: deal_order.expiration_block,
			timestamp: deal_order.timestamp,
			block: deal_order.block,
			funding_transfer_id: deal_order.funding_transfer_id,
			repayment_transfer_id: deal_order.repayment_transfer_id,
			lock: deal_order.lock,
			borrower: deal_order.borrower,
		})
	});

	AskOrders::<T>::translate::<OldAskOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>, _>(
		|exp, hash, ask_order| {
			weight = weight.saturating_add(weight_each);
			let ask_id = AskOrderId::with_expiration_hash::<T>(exp, hash);
			let currency = reconstructed_currency_ask
				.remove(&ask_id)
				.unwrap_or_else(CurrencyId::placeholder);
			Some(AskOrder {
				lender_address_id: ask_order.lender_address_id,
				terms: AskTerms::try_from(translate_loan_terms::<T>(
					ask_order.terms.0,
					currency,
				)).expect("terms are checked for validity on creation so they must be valid on an existing ask order; qed"),
				expiration_block: ask_order.expiration_block,
				block: ask_order.block,
				lender: ask_order.lender,
			})
		},
	);

	BidOrders::<T>::translate::<OldBidOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>, _>(
		|exp, hash, bid_order| {
			weight = weight.saturating_add(weight_each);
			let bid_id = BidOrderId::with_expiration_hash::<T>(exp, hash);
			let currency = reconstructed_currency_bid
				.remove(&bid_id)
				.unwrap_or_else(CurrencyId::placeholder);
			Some(BidOrder {
				borrower_address_id: bid_order.borrower_address_id,
				terms: BidTerms::try_from(translate_loan_terms::<T>(bid_order.terms.0, currency)).expect("terms are checked on creation so they must be valid on existing bid order; qed"),
				expiration_block: bid_order.expiration_block,
				block: bid_order.block,
				borrower: bid_order.borrower,
			})
		},
	);

	Transfers::<T>::translate::<
		OldTransfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_id, transfer| {
		weight = weight.saturating_add(weight_each);
		translate_transfer::<T>(transfer)
	});

	PendingTasks::<T>::translate::<
		OldTask<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_exp, _id, task| {
		weight = weight.saturating_add(weight_each);
		Some(match task {
			OldTask::VerifyTransfer(unverified_transfer) => {
				let kind = unverified_transfer.transfer.kind.clone();
				Task::VerifyTransfer(UnverifiedTransfer {
					transfer: translate_transfer::<T>(unverified_transfer.transfer)?,
					from_external: unverified_transfer.from_external,
					to_external: unverified_transfer.to_external,
					deadline: unverified_transfer.deadline,
					currency_to_check: crate::CurrencyOrLegacyTransferKind::TransferKind(
						to_legacy_transfer_kind(kind),
					),
				})
			},
			OldTask::CollectCoins(collect_coins) => Task::CollectCoins(UnverifiedCollectedCoins {
				to: collect_coins.to,
				tx_id: collect_coins.tx_id,
				contract: Default::default(),
			}),
		})
	});

	Addresses::<T>::translate::<OldAddress<AccountIdOf<T>>, _>(|_id, address| {
		weight = weight.saturating_add(weight_each);
		Some(Address {
			blockchain: translate_blockchain(address.blockchain)?,
			value: address.value,
			owner: address.owner,
		})
	});

	for (currency_id, currency) in currencies {
		weight = weight.saturating_add(write);
		crate::Currencies::<T>::insert(currency_id, currency);
	}

	weight
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		concatenate,
		helpers::HexToAddress,
		mock::{ExtBuilder, Test},
		tests::{IntoBounded, TestInfo},
		Duration, InterestRate,
	};
	use frame_support::Blake2_128Concat;
	use sp_runtime::traits::Hash as _;

	type OldAddresses = Addresses<Test>;

	#[frame_support::storage_alias]
	type DealOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		super::OldDealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
	>;

	type OldDealOrders = DealOrders<Test>;

	#[frame_support::storage_alias]
	type AskOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		super::OldAskOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>,
	>;
	type OldAskOrders = AskOrders<Test>;

	#[frame_support::storage_alias]
	type BidOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		super::OldBidOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>>,
	>;
	type OldBidOrders = BidOrders<Test>;

	#[storage_alias]
	type Addresses<T: Config> = StorageMap<
		crate::Pallet<T>,
		Blake2_128Concat,
		AddressId<HashOf<T>>,
		super::OldAddress<AccountIdOf<T>>,
	>;

	#[storage_alias]
	type PendingTasks<T: Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Identity,
		BlockNumberOf<T>,
		Identity,
		TaskId<HashOf<T>>,
		super::OldTask<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
	>;

	type OldPendingTasks = PendingTasks<Test>;

	type OldTransfers = super::OldTransfers<Test>;

	fn hash(val: &[u8]) -> <Test as frame_system::Config>::Hash {
		<Test as frame_system::Config>::Hashing::hash(val)
	}

	type AccountId = <Test as frame_system::Config>::AccountId;
	type BlockNumber = <Test as frame_system::Config>::BlockNumber;
	type Hash = <Test as frame_system::Config>::Hash;
	type Moment = <Test as pallet_timestamp::Config>::Moment;

	type DealOrderId = crate::DealOrderId<BlockNumber, Hash>;
	type AskOrderId = crate::AskOrderId<BlockNumber, Hash>;
	type BidOrderId = crate::BidOrderId<BlockNumber, Hash>;
	type OfferId = crate::OfferId<BlockNumber, Hash>;

	type OldDealOrder = super::OldDealOrder<AccountId, BlockNumber, Hash, Moment>;
	type OldAskOrder = super::OldAskOrder<AccountId, BlockNumber, Hash>;
	type OldBidOrder = super::OldBidOrder<AccountId, BlockNumber, Hash>;
	type OldTransfer = super::OldTransfer<AccountId, BlockNumber, Hash, Moment>;
	type Offer = crate::Offer<AccountId, BlockNumber, Hash>;

	fn old_transfer(
		test_info: &TestInfo,
		deal_id: DealOrderId,
		kind: OldTransferKind,
	) -> (TransferId<Hash>, OldTransfer) {
		let blockchain = OldBlockchain::Rinkeby;
		let transfer = OldTransfer {
			blockchain: blockchain.clone(),
			kind,
			from: test_info.lender.address_id.clone(),
			to: test_info.borrower.address_id.clone(),
			order_id: OldOrderId::Deal(deal_id),
			amount: 1.into(),
			tx_id: "0xdeadbeef".hex_to_address(),
			block: 50,
			is_processed: false,
			account_id: test_info.lender.account_id.clone(),
			timestamp: Some(10000),
		};

		let transfer_id = crate::TransferId::make({
			let key = concatenate!(blockchain.as_bytes(), &*transfer.tx_id);
			hash(&key)
		});

		(transfer_id, transfer)
	}

	fn attach_transfer(transfer_id: TransferId<Hash>, deal: &mut OldDealOrder) {
		deal.funding_transfer_id = Some(transfer_id);
	}

	fn old_ask_bid_offer(
		test_info: &TestInfo,
	) -> ((AskOrderId, OldAskOrder), (BidOrderId, OldBidOrder), (OfferId, Offer)) {
		let expiration = 10000;
		let ask = OldAskOrder {
			blockchain: OldBlockchain::Rinkeby,
			lender_address_id: test_info.lender.address_id.clone(),
			terms: OldAskTerms(old_loan_terms()),
			expiration_block: expiration,
			block: 10,
			lender: test_info.lender.account_id.clone(),
		};

		let bid = OldBidOrder {
			blockchain: OldBlockchain::Rinkeby,
			borrower_address_id: test_info.borrower.address_id.clone(),
			terms: OldBidTerms(old_loan_terms()),
			expiration_block: expiration,
			block: 11,
			borrower: test_info.borrower.account_id.clone(),
		};

		let ask_id = AskOrderId::new::<Test>(expiration, &[1, 1, 1, 1]);
		let bid_id = BidOrderId::new::<Test>(expiration, &[2, 2, 2, 2]);

		let offer = Offer {
			ask_id: ask_id.clone(),
			bid_id: bid_id.clone(),
			expiration_block: expiration,
			block: 12,
			lender: test_info.lender.account_id.clone(),
		};
		let offer_id = OfferId::new::<Test>(expiration, &ask_id, &bid_id);

		((ask_id, ask), (bid_id, bid), (offer_id, offer))
	}

	fn old_loan_terms() -> OldLoanTerms {
		OldLoanTerms {
			amount: 100u64.into(),
			interest_rate: InterestRate {
				rate_per_period: 100,
				decimals: 4,
				period: Duration::from_millis(2000),
				interest_type: crate::InterestType::Simple,
			},
			term_length: Duration::from_millis(10000),
		}
	}

	fn old_deal_order(
		test_info: &TestInfo,
		offer: Option<(Offer, OfferId)>,
	) -> (DealOrderId, OldDealOrder) {
		let (offer_id, _offer) = match offer {
			Some((off, id)) => (id, off),
			None => test_info.create_offer(),
		};
		let expiration_block = 10000;

		let deal_id = DealOrderId::with_expiration_hash::<Test>(
			expiration_block,
			hash(offer_id.hash().as_ref()),
		);
		let blockchain = OldBlockchain::Rinkeby;

		(
			deal_id,
			OldDealOrder {
				blockchain,
				offer_id,
				lender_address_id: test_info.lender.address_id.clone(),
				borrower_address_id: test_info.borrower.address_id.clone(),
				terms: old_loan_terms(),
				expiration_block,
				timestamp: 100000,
				block: Some(100),
				funding_transfer_id: None,
				repayment_transfer_id: None,
				lock: None,
				borrower: test_info.borrower.account_id.clone(),
			},
		)
	}

	fn old_to_new_terms(terms: OldLoanTerms, currency: Option<Currency>) -> super::LoanTerms<Hash> {
		super::LoanTerms {
			amount: terms.amount,
			interest_rate: terms.interest_rate,
			term_length: terms.term_length,
			currency: currency.map_or_else(CurrencyId::placeholder, |c| c.to_id::<Test>()),
		}
	}

	fn old_to_new_deal(
		deal: OldDealOrder,
		currency: Option<Currency>,
	) -> super::DealOrder<AccountId, BlockNumber, Hash, Moment> {
		super::DealOrder {
			offer_id: deal.offer_id,
			lender_address_id: deal.lender_address_id,
			borrower_address_id: deal.borrower_address_id,
			terms: old_to_new_terms(deal.terms, currency),
			expiration_block: deal.expiration_block,
			timestamp: deal.timestamp,
			block: deal.block,
			funding_transfer_id: deal.funding_transfer_id,
			repayment_transfer_id: deal.repayment_transfer_id,
			lock: deal.lock,
			borrower: deal.borrower,
		}
	}

	fn old_to_new_ask(
		ask: OldAskOrder,
		currency: Option<Currency>,
	) -> super::AskOrder<AccountId, BlockNumber, Hash> {
		super::AskOrder {
			lender_address_id: ask.lender_address_id,
			terms: crate::AskTerms::try_from(old_to_new_terms(ask.terms.0, currency)).unwrap(),
			expiration_block: ask.expiration_block,
			block: ask.block,
			lender: ask.lender,
		}
	}

	fn old_to_new_bid(
		bid: OldBidOrder,
		currency: Option<Currency>,
	) -> super::BidOrder<AccountId, BlockNumber, Hash> {
		super::BidOrder {
			borrower_address_id: bid.borrower_address_id,
			terms: crate::BidTerms::try_from(old_to_new_terms(bid.terms.0, currency)).unwrap(),
			expiration_block: bid.expiration_block,
			block: bid.block,
			borrower: bid.borrower,
		}
	}

	fn ethless_currency(contract: &str) -> Currency {
		Currency::Evm(
			EvmCurrencyType::SmartContract(
				contract.hex_to_address(),
				vec![EvmTransferKind::Ethless].into_bounded(),
			),
			EvmInfo::RINKEBY,
		)
	}

	fn insert_deal(id: &DealOrderId, deal: &OldDealOrder) {
		OldDealOrders::insert(id.expiration(), id.hash(), deal);
	}

	fn insert_transfer(id: &TransferId<Hash>, transfer: &OldTransfer) {
		OldTransfers::insert(id, transfer);
	}

	const CONTRACT: &str = "0xaaaa";

	#[test]
	fn deal_order_with_transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();

			let (deal_id, mut deal) = old_deal_order(&test_info, None);

			let (transfer_id, transfer) = old_transfer(
				&test_info,
				deal_id.clone(),
				OldTransferKind::Ethless(CONTRACT.hex_to_address()),
			);
			insert_transfer(&transfer_id, &transfer);

			attach_transfer(transfer_id, &mut deal);
			insert_deal(&deal_id, &deal);

			migrate::<Test>();

			let migrated_deal =
				super::DealOrders::<Test>::get(deal_id.expiration(), deal_id.hash()).unwrap();

			assert_eq!(migrated_deal, old_to_new_deal(deal, Some(ethless_currency(CONTRACT))));
		});
	}

	#[test]
	fn deal_order_without_transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::default();

			let (deal_id, deal) = old_deal_order(&test_info, None);
			insert_deal(&deal_id, &deal);

			migrate::<Test>();

			let migrated_deal =
				super::DealOrders::<Test>::get(deal_id.expiration(), deal_id.hash()).unwrap();

			assert_eq!(migrated_deal, old_to_new_deal(deal, None));
		});
	}

	#[test]
	fn transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::default();

			let (deal_id, _) = old_deal_order(&test_info, None);
			let (transfer_id, transfer) = old_transfer(
				&test_info,
				deal_id.clone(),
				OldTransferKind::Ethless(CONTRACT.hex_to_address()),
			);

			insert_transfer(&transfer_id, &transfer);

			migrate::<Test>();

			let migrated_transfer = super::Transfers::<Test>::get(&transfer_id).unwrap();

			assert_eq!(
				migrated_transfer,
				super::Transfer {
					blockchain: super::Blockchain::RINKEBY,
					kind: super::TransferKind::Evm(EvmTransferKind::Ethless),
					from: test_info.lender.address_id,
					to: test_info.borrower.address_id,
					deal_order_id: deal_id,
					amount: transfer.amount,
					tx_id: transfer.tx_id,
					block: transfer.block,
					is_processed: transfer.is_processed,
					account_id: transfer.account_id,
					timestamp: transfer.timestamp
				}
			)
		})
	}

	#[test]
	fn address_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::default();

			let old_address = OldAddress {
				blockchain: OldBlockchain::Rinkeby,
				owner: test_info.lender.account_id,
				value: "0xaaaabbbbccccdddd".hex_to_address(),
			};
			let address_id = super::AddressId::make(hash(&concatenate!(
				old_address.blockchain.as_bytes(),
				&*old_address.value
			)));

			OldAddresses::insert(&address_id, &old_address);

			migrate::<Test>();

			let migrated_address = super::Addresses::<Test>::get(&address_id).unwrap();

			assert_eq!(
				migrated_address,
				super::Address {
					blockchain: super::Blockchain::RINKEBY,
					value: old_address.value,
					owner: old_address.owner
				}
			);
		});
	}

	#[test]
	fn ask_bid_orders_with_transfer_migrate() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::default();

			let ((ask_id, ask), (bid_id, bid), (offer_id, offer)) = old_ask_bid_offer(&test_info);

			OldAskOrders::insert(ask_id.expiration(), ask_id.hash(), &ask);
			OldBidOrders::insert(bid_id.expiration(), bid_id.hash(), &bid);
			crate::Offers::<Test>::insert(offer_id.expiration(), offer_id.hash(), &offer);

			let (deal_id, mut deal) = old_deal_order(&test_info, Some((offer, offer_id)));

			let (transfer_id, transfer) = old_transfer(
				&test_info,
				deal_id.clone(),
				OldTransferKind::Ethless(CONTRACT.hex_to_address()),
			);

			insert_transfer(&transfer_id, &transfer);
			attach_transfer(transfer_id, &mut deal);

			insert_deal(&deal_id, &deal);

			migrate::<Test>();

			let migrated_ask =
				super::AskOrders::<Test>::get(ask_id.expiration(), ask_id.hash()).unwrap();

			let migrated_bid =
				super::BidOrders::<Test>::get(bid_id.expiration(), bid_id.hash()).unwrap();

			let currency = ethless_currency(CONTRACT);

			assert_eq!(migrated_ask, old_to_new_ask(ask, Some(currency.clone())));

			assert_eq!(migrated_bid, old_to_new_bid(bid, Some(currency)));
		});
	}

	#[test]
	fn ask_bid_orders_without_transfer_migrate() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::default();

			let ((ask_id, ask), (bid_id, bid), (offer_id, offer)) = old_ask_bid_offer(&test_info);

			OldAskOrders::insert(ask_id.expiration(), ask_id.hash(), &ask);
			OldBidOrders::insert(bid_id.expiration(), bid_id.hash(), &bid);
			crate::Offers::<Test>::insert(offer_id.expiration(), offer_id.hash(), &offer);

			let (deal_id, deal) = old_deal_order(&test_info, Some((offer, offer_id)));

			insert_deal(&deal_id, &deal);

			migrate::<Test>();

			let migrated_ask =
				super::AskOrders::<Test>::get(ask_id.expiration(), ask_id.hash()).unwrap();

			let migrated_bid =
				super::BidOrders::<Test>::get(bid_id.expiration(), bid_id.hash()).unwrap();

			assert_eq!(migrated_ask, old_to_new_ask(ask, None));

			assert_eq!(migrated_bid, old_to_new_bid(bid, None));
		});
	}

	#[test]
	fn unverified_collected_coins_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let tx_id = "0xfafafafafafafa".hex_to_address();

			let old_collect_coins = OldUnverifiedCollectedCoins {
				to: b"baba".to_vec().try_into().unwrap(),
				tx_id: tx_id.clone(),
			};

			let deadline = 100;

			let id = TaskId::from(crate::CollectedCoinsId::make(hash(&tx_id)));

			let new_collect_coins = UnverifiedCollectedCoins {
				to: b"baba".to_vec().try_into().unwrap(),
				tx_id,
				contract: Default::default(),
			};

			OldPendingTasks::insert(deadline, &id, OldTask::from(old_collect_coins));

			migrate::<Test>();

			assert_eq!(
				super::PendingTasks::<Test>::get(deadline, &id).unwrap(),
				Task::CollectCoins(new_collect_coins)
			);
		});
	}
}
