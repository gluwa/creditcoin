// `interest_type` added to `LoanTerms`

use super::v2;
use frame_support::dispatch::Weight;
use frame_support::{generate_storage_alias, traits::Get, Identity, Twox64Concat};
use parity_scale_codec::{Decode, Encode};

use crate::{AddressId, Config, Duration, ExternalAmount, OfferId, TransferId};

pub use v2::AskOrder as OldAskOrder;
pub use v2::AskTerms as OldAskTerms;
pub use v2::BidOrder as OldBidOrder;
pub use v2::BidTerms as OldBidTerms;
pub use v2::Blockchain;
pub use v2::DealOrder as OldDealOrder;
pub use v2::InterestRate as OldInterestRate;
pub use v2::LoanTerms as OldLoanTerms;
pub use v2::{OrderId, Transfer, TransferKind};

use crate::InterestRate;

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct LoanTerms {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length: Duration,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct AskTerms(pub(super) LoanTerms);
#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct BidTerms(pub(super) LoanTerms);

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct AskOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub lender_address_id: AddressId<Hash>,
	pub terms: AskTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct BidOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: BidTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub borrower: AccountId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct DealOrder<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub offer_id: OfferId<BlockNum, Hash>,
	pub lender_address_id: AddressId<Hash>,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: LoanTerms,
	pub expiration_block: BlockNum,
	pub timestamp: Moment,
	pub block: Option<BlockNum>,
	pub funding_transfer_id: Option<TransferId<Hash>>,
	pub repayment_transfer_id: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub borrower: AccountId,
}

impl From<OldInterestRate> for InterestRate {
	fn from(old: OldInterestRate) -> Self {
		Self {
			decimals: old.decimals,
			rate_per_period: old.rate_per_period,
			period: old.period,
			interest_type: crate::InterestType::Simple,
		}
	}
}

impl From<OldLoanTerms> for LoanTerms {
	fn from(old: OldLoanTerms) -> Self {
		Self {
			amount: old.amount,
			interest_rate: InterestRate::from(old.interest_rate),
			term_length: old.term_length,
		}
	}
}

impl From<LoanTerms> for AskTerms {
	fn from(terms: LoanTerms) -> Self {
		Self(terms)
	}
}

impl From<LoanTerms> for BidTerms {
	fn from(terms: LoanTerms) -> Self {
		Self(terms)
	}
}

impl From<OldAskTerms> for AskTerms {
	fn from(old: OldAskTerms) -> Self {
		AskTerms(LoanTerms::from(old.0))
	}
}

impl From<OldBidTerms> for BidTerms {
	fn from(old: OldBidTerms) -> Self {
		BidTerms(LoanTerms::from(old.0))
	}
}

generate_storage_alias!(
	Creditcoin,
	DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

generate_storage_alias!(
	Creditcoin,
	AskOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), AskOrder<T::AccountId, T::BlockNumber, T::Hash>>
);

generate_storage_alias!(
	Creditcoin,
	BidOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), BidOrder<T::AccountId, T::BlockNumber, T::Hash>>
);

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	DealOrders::<T>::translate::<OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_exp, _hash, old_deal| {
			weight = weight.saturating_add(weight_each);
			Some(DealOrder {
				blockchain: old_deal.blockchain,
				offer_id: old_deal.offer_id,
				lender_address_id: old_deal.lender_address_id,
				borrower_address_id: old_deal.borrower_address_id,
				terms: LoanTerms::from(old_deal.terms),
				expiration_block: old_deal.expiration_block,
				timestamp: old_deal.timestamp,
				block: old_deal.block,
				funding_transfer_id: old_deal.funding_transfer_id,
				repayment_transfer_id: old_deal.repayment_transfer_id,
				lock: old_deal.lock,
				borrower: old_deal.borrower,
			})
		},
	);

	AskOrders::<T>::translate::<OldAskOrder<T::AccountId, T::BlockNumber, T::Hash>, _>(
		|_exp, _hash, old_ask| {
			weight = weight.saturating_add(weight_each);
			Some(AskOrder {
				blockchain: old_ask.blockchain,
				lender_address_id: old_ask.lender_address_id,
				terms: AskTerms::from(old_ask.terms),
				expiration_block: old_ask.expiration_block,
				block: old_ask.block,
				lender: old_ask.lender,
			})
		},
	);

	BidOrders::<T>::translate::<OldBidOrder<T::AccountId, T::BlockNumber, T::Hash>, _>(
		|_exp, _hash, old_bid| {
			weight = weight.saturating_add(weight_each);
			Some(BidOrder {
				blockchain: old_bid.blockchain,
				borrower_address_id: old_bid.borrower_address_id,
				terms: BidTerms::from(old_bid.terms),
				expiration_block: old_bid.expiration_block,
				block: old_bid.block,
				borrower: old_bid.borrower,
			})
		},
	);

	weight
}

#[cfg(test)]
mod tests {
	use core::convert::TryFrom;

	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		AskOrderId, BidOrderId, DealOrderId, DoubleMapExt, Duration, InterestRate, OfferId,
	};

	use super::{
		generate_storage_alias, AskOrder, AskTerms, BidOrder, BidTerms, Blockchain, Config,
		DealOrder, Identity, LoanTerms, OldAskOrder, OldAskTerms, OldBidOrder, OldBidTerms,
		OldDealOrder, OldInterestRate, OldLoanTerms, Twox64Concat,
	};

	generate_storage_alias!(
		Creditcoin,
		DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
	);

	type OldDealOrders = DealOrders<Test>;

	generate_storage_alias!(
		Creditcoin,
		AskOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), OldAskOrder<T::AccountId, T::BlockNumber, T::Hash>>
	);

	type OldAskOrders = AskOrders<Test>;

	generate_storage_alias!(
		Creditcoin,
		BidOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), OldBidOrder<T::AccountId, T::BlockNumber, T::Hash>>
	);

	type OldBidOrders = BidOrders<Test>;

	#[test]
	fn ask_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let ask_order_id = AskOrderId::new::<Test>(100, "asdf".as_bytes());
			let test_info = TestInfo::new_defaults();

			let old_ask_order = OldAskOrder {
				blockchain: Blockchain::Ethereum,
				lender_address_id: test_info.lender.address_id,
				terms: OldAskTerms(OldLoanTerms {
					amount: 100u64.into(),
					interest_rate: OldInterestRate {
						rate_per_period: 100,
						decimals: 4,
						period: Duration::from_millis(1000),
					},
					term_length: Duration::from_millis(2000),
				}),
				expiration_block: 100,
				block: 1,
				lender: test_info.lender.account_id,
			};

			OldAskOrders::insert_id(&ask_order_id, &old_ask_order);

			super::migrate::<Test>();

			let ask_order = super::AskOrders::<Test>::try_get_id(&ask_order_id).unwrap();

			assert_eq!(
				ask_order,
				AskOrder {
					blockchain: old_ask_order.blockchain,
					lender_address_id: old_ask_order.lender_address_id,
					terms: AskTerms::try_from(LoanTerms {
						amount: old_ask_order.terms.0.amount,
						interest_rate: InterestRate {
							rate_per_period: old_ask_order.terms.0.interest_rate.rate_per_period,
							decimals: old_ask_order.terms.0.interest_rate.decimals,
							period: old_ask_order.terms.0.interest_rate.period,
							interest_type: crate::InterestType::Simple,
						},
						term_length: old_ask_order.terms.0.term_length,
					})
					.unwrap(),
					expiration_block: old_ask_order.expiration_block,
					block: old_ask_order.block,
					lender: old_ask_order.lender,
				}
			);
		});
	}

	#[test]
	fn bid_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let bid_order_id = BidOrderId::new::<Test>(100, "asdf".as_bytes());
			let test_info = TestInfo::new_defaults();

			let old_bid_order = OldBidOrder {
				blockchain: Blockchain::Ethereum,
				borrower_address_id: test_info.borrower.address_id,
				terms: OldBidTerms(OldLoanTerms {
					amount: 100u64.into(),
					interest_rate: OldInterestRate {
						rate_per_period: 100,
						decimals: 4,
						period: Duration::from_millis(1000),
					},
					term_length: Duration::from_millis(2000),
				}),
				expiration_block: 100,
				block: 1,
				borrower: test_info.borrower.account_id,
			};

			OldBidOrders::insert_id(&bid_order_id, &old_bid_order);

			super::migrate::<Test>();

			let bid_order = super::BidOrders::<Test>::try_get_id(&bid_order_id).unwrap();

			assert_eq!(
				bid_order,
				BidOrder {
					blockchain: old_bid_order.blockchain,
					borrower_address_id: old_bid_order.borrower_address_id,
					terms: BidTerms::try_from(LoanTerms {
						amount: old_bid_order.terms.0.amount,
						interest_rate: InterestRate {
							rate_per_period: old_bid_order.terms.0.interest_rate.rate_per_period,
							decimals: old_bid_order.terms.0.interest_rate.decimals,
							period: old_bid_order.terms.0.interest_rate.period,
							interest_type: crate::InterestType::Simple,
						},
						term_length: old_bid_order.terms.0.term_length,
					})
					.unwrap(),
					expiration_block: old_bid_order.expiration_block,
					block: old_bid_order.block,
					borrower: old_bid_order.borrower,
				}
			);
		});
	}

	#[test]
	fn deal_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();

			let deal_id = DealOrderId::with_expiration_hash::<Test>(100, [0u8; 32].into());
			let offer_id = OfferId::with_expiration_hash::<Test>(100, [1u8; 32].into());

			let old_deal = OldDealOrder {
				blockchain: Blockchain::Ethereum,
				offer_id,
				lender_address_id: test_info.lender.address_id,
				borrower_address_id: test_info.borrower.address_id,
				terms: OldLoanTerms {
					amount: 100u64.into(),
					interest_rate: OldInterestRate {
						rate_per_period: 100,
						decimals: 4,
						period: Duration::from_millis(2000),
					},
					term_length: Duration::from_millis(10000),
				},
				expiration_block: 100,
				timestamp: 0,
				funding_transfer_id: None,
				repayment_transfer_id: None,
				lock: None,
				block: None,
				borrower: test_info.borrower.account_id,
			};

			OldDealOrders::insert_id(&deal_id, &old_deal);

			super::migrate::<Test>();

			let deal = super::DealOrders::<Test>::try_get_id(&deal_id).unwrap();

			assert_eq!(
				deal,
				DealOrder {
					blockchain: old_deal.blockchain,
					offer_id: old_deal.offer_id,
					lender_address_id: old_deal.lender_address_id,
					borrower_address_id: old_deal.borrower_address_id,
					terms: LoanTerms {
						amount: old_deal.terms.amount,
						interest_rate: InterestRate {
							rate_per_period: old_deal.terms.interest_rate.rate_per_period,
							decimals: old_deal.terms.interest_rate.decimals,
							period: old_deal.terms.interest_rate.period,
							interest_type: crate::InterestType::Simple
						},
						term_length: old_deal.terms.term_length,
					},
					expiration_block: old_deal.expiration_block,
					timestamp: old_deal.timestamp,
					funding_transfer_id: old_deal.funding_transfer_id,
					repayment_transfer_id: old_deal.repayment_transfer_id,
					lock: old_deal.lock,
					borrower: old_deal.borrower,
					block: old_deal.block,
				}
			);
		});
	}
}
