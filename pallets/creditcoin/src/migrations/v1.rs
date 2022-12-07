// First `LoanTerms` rework. `maturity` is replaced with `term_length`,
// and `InterestRate` changed from a type alias = u64 to a new struct `InterestRate`

use super::{AccountIdOf, BlockNumberOf, HashOf, MomentOf};
use crate::{
	loan_terms::{Decimals, Duration},
	AddressId, Config, ExternalAmount, OfferId, RatePerPeriod, TransferId,
};
use frame_support::pallet_prelude::*;
use frame_support::{Identity, Twox64Concat};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::traits::{Saturating, UniqueSaturatedInto};

type OldInterestRate = u64;

const OLD_INTEREST_RATE_DECIMALS: u64 = 4;

#[derive(Encode, Decode)]
struct OldLoanTerms<Moment> {
	amount: ExternalAmount,
	interest_rate: OldInterestRate,
	maturity: Moment,
}

#[derive(Encode, Decode)]
struct OldAskTerms<Moment>(OldLoanTerms<Moment>);

#[derive(Encode, Decode)]
struct OldBidTerms<Moment>(OldLoanTerms<Moment>);

#[derive(Encode, Decode)]
struct OldAskOrder<AccountId, BlockNum, Hash, Moment> {
	blockchain: Blockchain,
	lender_address_id: AddressId<Hash>,
	terms: OldAskTerms<Moment>,
	expiration_block: BlockNum,
	block: BlockNum,
	lender: AccountId,
}

#[derive(Encode, Decode)]
struct OldBidOrder<AccountId, BlockNum, Hash, Moment> {
	blockchain: Blockchain,
	borrower_address_id: AddressId<Hash>,
	terms: OldBidTerms<Moment>,
	expiration_block: BlockNum,
	block: BlockNum,
	borrower: AccountId,
}

#[derive(Encode, Decode)]
struct OldDealOrder<AccountId, BlockNum, Hash, Moment> {
	blockchain: Blockchain,
	offer_id: OfferId<BlockNum, Hash>,
	lender_address_id: AddressId<Hash>,
	borrower_address_id: AddressId<Hash>,
	terms: OldLoanTerms<Moment>,
	expiration_block: BlockNum,
	timestamp: Moment,
	funding_transfer_id: Option<TransferId<Hash>>,
	repayment_transfer_id: Option<TransferId<Hash>>,
	lock: Option<AccountId>,
	borrower: AccountId,
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
	pub funding_transfer_id: Option<TransferId<Hash>>,
	pub repayment_transfer_id: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub borrower: AccountId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct InterestRate {
	pub rate_per_period: RatePerPeriod,
	pub decimals: Decimals,
	pub period: Duration,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct LoanTerms {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length: Duration,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct AskTerms(pub LoanTerms);

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct BidTerms(pub LoanTerms);

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

type OtherChainLen = ConstU32<256>;
pub type OtherChain = BoundedVec<u8, OtherChainLen>;

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq, Clone))]
pub enum Blockchain {
	Ethereum,
	Rinkeby,
	Luniverse,
	Bitcoin,
	Other(OtherChain),
}

impl Blockchain {
	#[allow(dead_code)]
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			Blockchain::Ethereum => b"ethereum",
			Blockchain::Rinkeby => b"rinkeby",
			Blockchain::Luniverse => b"luniverse",
			Blockchain::Bitcoin => b"bitcoin",
			Blockchain::Other(chain) => chain.as_slice(),
		}
	}
}

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
	AskOrders::<T>::translate::<
		OldAskOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_expiration, _hash, ask| {
		weight = weight.saturating_add(weight_each);
		Some(AskOrder {
			block: ask.block,
			blockchain: ask.blockchain,
			expiration_block: ask.expiration_block,
			lender: ask.lender,
			lender_address_id: ask.lender_address_id,
			terms: AskTerms(LoanTerms {
				amount: ask.terms.0.amount,
				interest_rate: InterestRate {
					rate_per_period: ask.terms.0.interest_rate,
					decimals: OLD_INTEREST_RATE_DECIMALS,
					period: Duration::from_millis(ask.terms.0.maturity.unique_saturated_into()),
				},
				term_length: Duration::from_millis(ask.terms.0.maturity.unique_saturated_into()),
			}),
		})
	});

	BidOrders::<T>::translate::<
		OldBidOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_expiration, _hash, bid| {
		weight = weight.saturating_add(weight_each);
		Some(BidOrder {
			block: bid.block,
			blockchain: bid.blockchain,
			expiration_block: bid.expiration_block,
			borrower: bid.borrower,
			borrower_address_id: bid.borrower_address_id,
			terms: BidTerms(LoanTerms {
				amount: bid.terms.0.amount,
				interest_rate: InterestRate {
					rate_per_period: bid.terms.0.interest_rate,
					decimals: OLD_INTEREST_RATE_DECIMALS,
					period: Duration::from_millis(bid.terms.0.maturity.unique_saturated_into()),
				},
				term_length: Duration::from_millis(bid.terms.0.maturity.unique_saturated_into()),
			}),
		})
	});

	DealOrders::<T>::translate::<
		OldDealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
		_,
	>(|_expiration, _hash, deal| {
		weight = weight.saturating_add(weight_each);
		Some(DealOrder {
			blockchain: deal.blockchain,
			offer_id: deal.offer_id,
			lender_address_id: deal.lender_address_id,
			borrower_address_id: deal.borrower_address_id,
			terms: LoanTerms {
				amount: deal.terms.amount,
				interest_rate: InterestRate {
					rate_per_period: deal.terms.interest_rate,
					decimals: OLD_INTEREST_RATE_DECIMALS,
					period: Duration::from_millis(deal.terms.maturity.unique_saturated_into()),
				},
				term_length: Duration::from_millis(
					deal.terms.maturity.saturating_sub(deal.timestamp).unique_saturated_into(),
				),
			},
			expiration_block: deal.expiration_block,
			timestamp: deal.timestamp,
			funding_transfer_id: deal.funding_transfer_id,
			repayment_transfer_id: deal.repayment_transfer_id,
			lock: deal.lock,
			borrower: deal.borrower,
		})
	});
	weight
}

#[cfg(test)]
mod tests {
	use sp_core::U256;

	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		AskOrderId, BidOrderId, DealOrderId, DoubleMapExt, OfferId,
	};

	use super::{
		AccountIdOf, AskOrder, AskTerms, BidOrder, BidTerms, BlockNumberOf, Blockchain, Duration,
		HashOf, Identity, InterestRate, LoanTerms, MomentOf, OldAskOrder, OldAskTerms, OldBidOrder,
		OldBidTerms, OldDealOrder, OldLoanTerms, Twox64Concat, OLD_INTEREST_RATE_DECIMALS,
	};

	#[frame_support::storage_alias]
	type DealOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		OldDealOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
	>;

	type OldDealOrders = DealOrders<Test>;

	#[frame_support::storage_alias]
	type AskOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		OldAskOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
	>;
	type OldAskOrders = AskOrders<Test>;

	#[frame_support::storage_alias]
	type BidOrders<T: crate::Config> = StorageDoubleMap<
		crate::Pallet<T>,
		Twox64Concat,
		BlockNumberOf<T>,
		Identity,
		HashOf<T>,
		OldBidOrder<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
	>;

	type OldBidOrders = BidOrders<Test>;

	#[test]
	fn ask_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let ask_id = AskOrderId::new::<Test>(100, "asdf".as_bytes());
			let test_info = TestInfo::new_defaults();
			let old_ask = OldAskOrder {
				blockchain: Blockchain::Ethereum,
				lender_address_id: test_info.lender.address_id.clone(),
				terms: OldAskTerms(OldLoanTerms {
					amount: 1000u64.into(),
					interest_rate: 1000,
					maturity: 2000,
				}),
				expiration_block: 100,
				block: 0,
				lender: test_info.lender.account_id,
			};
			OldAskOrders::insert_id(&ask_id, &old_ask);

			super::migrate::<Test>();

			let ask = super::AskOrders::<Test>::try_get_id(&ask_id).unwrap();

			assert_eq!(
				ask,
				AskOrder {
					blockchain: old_ask.blockchain,
					lender_address_id: old_ask.lender_address_id,
					terms: AskTerms(LoanTerms {
						amount: old_ask.terms.0.amount,
						interest_rate: InterestRate {
							rate_per_period: old_ask.terms.0.interest_rate,
							decimals: OLD_INTEREST_RATE_DECIMALS,
							period: Duration::from_millis(old_ask.terms.0.maturity)
						},
						term_length: Duration::from_millis(old_ask.terms.0.maturity)
					}),
					expiration_block: old_ask.expiration_block,
					block: old_ask.block,
					lender: old_ask.lender,
				}
			)
		});
	}

	#[test]
	fn bid_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let bid_id = BidOrderId::new::<Test>(100, "asdf".as_bytes());
			let test_info = TestInfo::new_defaults();
			let address_id = test_info.borrower.address_id.clone();
			let expiration_block = 100;
			let block = 0;
			let amount: U256 = 1000u64.into();

			let old_bid = OldBidOrder {
				blockchain: Blockchain::Ethereum,
				borrower_address_id: address_id,
				terms: OldBidTerms(OldLoanTerms { amount, interest_rate: 1000, maturity: 2000 }),
				expiration_block,
				block,
				borrower: test_info.borrower.account_id.clone(),
			};
			OldBidOrders::insert_id(&bid_id, &old_bid);

			super::migrate::<Test>();

			let bid = super::BidOrders::<Test>::try_get_id(&bid_id).unwrap();

			assert_eq!(
				bid,
				BidOrder {
					blockchain: old_bid.blockchain,
					borrower_address_id: old_bid.borrower_address_id,
					terms: BidTerms(LoanTerms {
						amount,
						interest_rate: InterestRate {
							rate_per_period: old_bid.terms.0.interest_rate,
							decimals: OLD_INTEREST_RATE_DECIMALS,
							period: Duration::from_millis(old_bid.terms.0.maturity)
						},
						term_length: Duration::from_millis(old_bid.terms.0.maturity)
					}),
					expiration_block,
					block,
					borrower: test_info.borrower.account_id,
				}
			)
		})
	}

	#[test]
	fn deal_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let offer_id = OfferId::with_expiration_hash::<Test>(100, [1; 32].into());
			let deal_id = DealOrderId::with_expiration_hash::<Test>(100, [0; 32].into());
			let test_info = TestInfo::new_defaults();
			let old_deal = OldDealOrder {
				blockchain: Blockchain::Ethereum,
				lender_address_id: test_info.lender.address_id.clone(),
				terms: OldLoanTerms { amount: 1000u64.into(), interest_rate: 1000, maturity: 2000 },
				expiration_block: 100,
				offer_id,
				borrower_address_id: test_info.borrower.address_id.clone(),
				timestamp: 0,
				funding_transfer_id: None,
				repayment_transfer_id: None,
				lock: None,
				borrower: test_info.borrower.account_id,
			};

			OldDealOrders::insert_id(&deal_id, &old_deal);

			super::migrate::<Test>();

			let deal = super::DealOrders::<Test>::try_get_id(&deal_id).unwrap();

			assert_eq!(
				deal,
				super::DealOrder {
					blockchain: old_deal.blockchain,
					lender_address_id: old_deal.lender_address_id,
					terms: LoanTerms {
						amount: old_deal.terms.amount,
						interest_rate: InterestRate {
							rate_per_period: old_deal.terms.interest_rate,
							decimals: OLD_INTEREST_RATE_DECIMALS,
							period: Duration::from_millis(old_deal.terms.maturity)
						},
						term_length: Duration::from_millis(
							old_deal.terms.maturity.saturating_sub(old_deal.timestamp)
						)
					},
					offer_id: old_deal.offer_id,
					borrower_address_id: old_deal.borrower_address_id,
					expiration_block: old_deal.expiration_block,
					timestamp: old_deal.timestamp,
					funding_transfer_id: old_deal.funding_transfer_id,
					repayment_transfer_id: old_deal.repayment_transfer_id,
					lock: old_deal.lock,
					borrower: old_deal.borrower
				}
			);
		});
	}
}
