use alloc::format;
use frame_system::pallet_prelude::BlockNumberFor;
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;

use crate::types::DoubleMapExt;
use crate::types::{AddressId, AskOrderId, AskTerms, BidOrderId, BidTerms, Blockchain, OfferId};
use crate::{Config, Duration, InterestRate, InterestType, LoanTerms};

pub(crate) fn fake_address_id<T: Config>(seed: u32) -> AddressId<T::Hash> {
	let address = format!("somefakeaddress{seed}");
	crate::AddressId::new::<T>(&Blockchain::Ethereum, address.as_bytes())
}

pub(crate) fn fake_ask_id<T: Config>(
	seed: u32,
	expiration_block: BlockNumberFor<T>,
) -> AskOrderId<T::BlockNumber, T::Hash> {
	let guid = format!("somefakeaskguid{seed}");
	crate::AskOrderId::new::<T>(expiration_block, guid.as_bytes())
}

pub(crate) fn insert_fake_ask<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	seed: u32,
) {
	let address_id = fake_address_id::<T>(seed);
	let ask_id = fake_ask_id::<T>(seed, expiration_block);
	let ask = crate::AskOrder {
		block: System::<T>::block_number(),
		blockchain: Blockchain::Ethereum,
		expiration_block,
		lender: who.clone(),
		lender_address_id: address_id,
		terms: AskTerms::try_from(fake_loan_terms()).unwrap(),
	};

	crate::AskOrders::<T>::insert_id(ask_id, ask);
}

pub(crate) fn fake_bid_id<T: SystemConfig>(
	seed: u32,
	expiration_block: BlockNumberFor<T>,
) -> BidOrderId<T::BlockNumber, T::Hash> {
	let guid = format!("somefakebidguid{seed}");
	crate::BidOrderId::new::<T>(expiration_block, guid.as_bytes())
}

pub(crate) fn insert_fake_bid<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	seed: u32,
) {
	let address_id = fake_address_id::<T>(seed);
	let bid_id = fake_bid_id::<T>(seed, expiration_block);
	let bid = crate::BidOrder {
		block: System::<T>::block_number(),
		blockchain: Blockchain::Ethereum,
		expiration_block,
		borrower: who.clone(),
		borrower_address_id: address_id,
		terms: BidTerms::try_from(fake_loan_terms()).unwrap(),
	};

	crate::BidOrders::<T>::insert_id(bid_id, bid);
}

pub(crate) fn fake_offer_id<T: SystemConfig>(
	expiration_block: BlockNumberFor<T>,
	ask_id: &AskOrderId<T::BlockNumber, T::Hash>,
	bid_id: &BidOrderId<T::BlockNumber, T::Hash>,
) -> OfferId<T::BlockNumber, T::Hash> {
	OfferId::new::<T>(expiration_block, ask_id, bid_id)
}

pub(crate) fn insert_fake_offer<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	seed: u32,
) {
	let ask_id = fake_ask_id::<T>(seed, expiration_block);
	let bid_id = fake_bid_id::<T>(seed, expiration_block);

	let offer_id = fake_offer_id::<T>(expiration_block, &ask_id, &bid_id);
	let offer = crate::Offer {
		ask_id,
		bid_id,
		block: frame_system::Pallet::<T>::block_number(),
		blockchain: Blockchain::Ethereum,
		expiration_block,
		lender: who.clone(),
	};

	crate::Offers::<T>::insert_id(offer_id, offer);
}

pub(crate) fn fake_loan_terms() -> LoanTerms {
	LoanTerms {
		amount: 10u64.into(),
		interest_rate: InterestRate {
			rate_per_period: 1,
			decimals: 1,
			period: Duration::from_millis(100),
			interest_type: InterestType::Simple,
		},
		term_length: Duration::new(1u64, 0u32),
	}
}
