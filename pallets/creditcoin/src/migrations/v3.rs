use super::{v1, v2};
use core::convert::TryFrom;
use frame_support::dispatch::Weight;
use frame_support::{generate_storage_alias, traits::Get, Identity, Twox64Concat};

use crate::Config;

use v1::AskOrder as OldAskOrder;
use v1::AskTerms as OldAskTerms;
use v1::BidOrder as OldBidOrder;
use v1::BidTerms as OldBidTerms;
use v1::InterestRate as OldInterestRate;
use v1::LoanTerms as OldLoanTerms;
use v2::DealOrder as OldDealOrder;

use crate::AskOrder;
use crate::AskTerms;
use crate::BidOrder;
use crate::BidTerms;
use crate::DealOrder;
use crate::InterestRate;
use crate::LoanTerms;

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

impl From<OldAskTerms> for AskTerms {
	fn from(old: OldAskTerms) -> Self {
		Self::try_from(LoanTerms::from(old.0)).expect("existing ask terms must be valid")
	}
}

impl From<OldBidTerms> for BidTerms {
	fn from(old: OldBidTerms) -> Self {
		Self::try_from(LoanTerms::from(old.0)).expect("existing bid terms must be valid")
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
        }
    );

	weight
}
