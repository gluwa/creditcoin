use super::v5;
use crate::Config;
use crate::EvmCurrencyType;
use crate::EvmInfo;
use crate::EvmSupportedTransferKinds;
use crate::EvmTransferKind;
use crate::Id;
use core::convert::TryFrom;
use frame_support::generate_storage_alias;
use frame_support::pallet_prelude::*;
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
	currency: CurrencyId<T::Hash>,
) -> LoanTerms<T::Hash> {
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
	deal_order: &OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
) -> Option<Currency> {
	let transfer_id = deal_order.funding_transfer_id.as_ref()?;
	let transfer = OldTransfers::<T>::get(transfer_id)?;
	let currency = reconstruct_currency(&deal_order.blockchain, &transfer.kind)?;
	Some(currency)
}

fn translate_transfer<T: Config>(
	transfer: OldTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
) -> Option<Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>> {
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

generate_storage_alias!(
	Creditcoin,
	Transfers<T: Config> => Map<(Identity, TransferId<T::Hash>), Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

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
	TransferId<T::Hash>,
	OldTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
>;

generate_storage_alias!(
	Creditcoin,
	Addresses<T: Config> => Map<(Blake2_128Concat, AddressId<T::Hash>), Address<T::AccountId>>
);

generate_storage_alias!(
	Creditcoin,
	AskOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), AskOrder<T::AccountId, T::BlockNumber, T::Hash>>
);

generate_storage_alias!(
	Creditcoin,
	BidOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), BidOrder<T::AccountId, T::BlockNumber, T::Hash>>
);

generate_storage_alias!(
	Creditcoin,
	DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

generate_storage_alias!(
	Creditcoin,
	PendingTasks<T: Config> => DoubleMap<(Identity, T::BlockNumber), (Identity, TaskId<T::Hash>), Task<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);
	let write = T::DbWeight::get().writes(1);
	let read = T::DbWeight::get().reads(1);

	let mut reconstructed_currency_ask = BTreeMap::new();
	let mut reconstructed_currency_bid = BTreeMap::new();
	let mut currencies = BTreeSet::new();

	DealOrders::<T>::translate::<OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_exp, _hash, deal_order| {
			weight = weight.saturating_add(weight_each);

			let currency = reconstruct_currency_from_deal::<T>(&deal_order);
			let currency_id = if let Some(currency) = currency.as_ref() {
				currency.to_id::<T>()
			} else {
				CurrencyId::placeholder()
			};

			weight = weight.saturating_add(read);
			let offer = if let Some(offer) = crate::Offers::<T>::get(
				deal_order.offer_id.expiration(),
				deal_order.offer_id.hash(),
			) {
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
		},
	);

	AskOrders::<T>::translate::<OldAskOrder<T::AccountId, T::BlockNumber, T::Hash>, _>(
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

	BidOrders::<T>::translate::<OldBidOrder<T::AccountId, T::BlockNumber, T::Hash>, _>(
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

	Transfers::<T>::translate::<OldTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_id, transfer| {
			weight = weight.saturating_add(weight_each);
			translate_transfer::<T>(transfer)
		},
	);

	PendingTasks::<T>::translate::<OldTask<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_exp, _id, task| {
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
				OldTask::CollectCoins(collect_coins) => Task::CollectCoins(collect_coins),
			})
		},
	);

	Addresses::<T>::translate::<OldAddress<T::AccountId>, _>(|_id, address| {
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
