use crate::{
	loan_terms::{Decimals, Duration},
	AddressId, Blockchain, Config, ExternalAmount, OfferId, RatePerPeriod, TransferId,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::weights::Weight;
use frame_support::{generate_storage_alias, traits::Get};
use frame_support::{Identity, Twox64Concat};
use scale_info::TypeInfo;
use sp_runtime::traits::{Saturating, UniqueSaturatedInto};

type OldInterestRate = u64;

const OLD_INTEREST_RATE_DECIMALS: u64 = 4;

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
struct OldLoanTerms<Moment> {
	amount: ExternalAmount,
	interest_rate: OldInterestRate,
	maturity: Moment,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
struct OldAskTerms<Moment>(OldLoanTerms<Moment>);

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
struct OldBidTerms<Moment>(OldLoanTerms<Moment>);

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
struct OldAskOrder<AccountId, BlockNum, Hash, Moment> {
	blockchain: Blockchain,
	lender_address_id: AddressId<Hash>,
	terms: OldAskTerms<Moment>,
	expiration_block: BlockNum,
	block: BlockNum,
	lender: AccountId,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
struct OldBidOrder<AccountId, BlockNum, Hash, Moment> {
	blockchain: Blockchain,
	borrower_address_id: AddressId<Hash>,
	terms: OldBidTerms<Moment>,
	expiration_block: BlockNum,
	block: BlockNum,
	borrower: AccountId,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
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

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
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

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct InterestRate {
	pub rate_per_period: RatePerPeriod,
	pub decimals: Decimals,
	pub period: Duration,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct LoanTerms {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length: Duration,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct AskTerms(pub LoanTerms);

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct BidTerms(pub LoanTerms);

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct AskOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub lender_address_id: AddressId<Hash>,
	pub terms: AskTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct BidOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: BidTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub borrower: AccountId,
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
	AskOrders::<T>::translate::<OldAskOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_expiration, _hash, ask| {
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
					term_length: Duration::from_millis(
						ask.terms.0.maturity.unique_saturated_into(),
					),
				}),
			})
		},
	);

	BidOrders::<T>::translate::<OldBidOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_expiration, _hash, bid| {
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
					term_length: Duration::from_millis(
						bid.terms.0.maturity.unique_saturated_into(),
					),
				}),
			})
		},
	);

	DealOrders::<T>::translate::<OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_expiration, _hash, deal| {
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
		},
	);
	weight
}
