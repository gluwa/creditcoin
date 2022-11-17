use core::ops::Deref;

use crate::CurrencyId;

use super::ExternalAmount;
use frame_support::RuntimeDebug;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::convert::TryFrom;

pub type RatePerPeriod = u64;
pub type Decimals = u64;
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Duration {
	secs: u64,
	nanos: u32,
}

const MILLIS_PER_SEC: u64 = 1_000;
const NANOS_PER_MILLI: u32 = 1_000_000;

impl Duration {
	pub const fn new(secs: u64, nanos: u32) -> Self {
		Self { secs, nanos }
	}
	pub const fn from_millis(millis: u64) -> Self {
		Self {
			secs: millis / MILLIS_PER_SEC,
			nanos: ((millis % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI,
		}
	}

	pub const fn is_zero(&self) -> bool {
		self.secs == 0 && self.nanos == 0
	}
}

#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum InterestType {
	Simple,
	Compound,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct InterestRate {
	pub rate_per_period: RatePerPeriod,
	pub decimals: Decimals,
	pub period: Duration,
	pub interest_type: InterestType,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LoanTerms<Hash> {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length: Duration,
	pub currency: CurrencyId<Hash>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskTerms<Hash>(LoanTerms<Hash>);

impl<Hash> Deref for AskTerms<Hash> {
	type Target = LoanTerms<Hash>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Clone, Copy, RuntimeDebug)]
pub struct InvalidTermLengthError;

impl<T: crate::Config> From<InvalidTermLengthError> for crate::Error<T> {
	fn from(_: InvalidTermLengthError) -> Self {
		Self::InvalidTermLength
	}
}

impl<Hash> TryFrom<LoanTerms<Hash>> for AskTerms<Hash> {
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms<Hash>) -> Result<Self, Self::Error> {
		if terms.term_length.is_zero() {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl<Hash> AskTerms<Hash> {
	pub fn match_with(&self, bid_terms: &BidTerms<Hash>) -> bool {
		self.amount == bid_terms.amount
			&& self.interest_rate == bid_terms.interest_rate
			&& self.term_length == bid_terms.term_length
	}

	pub fn agreed_terms(&self, bid_terms: BidTerms<Hash>) -> Option<LoanTerms<Hash>> {
		self.match_with(&bid_terms).then_some(bid_terms.0)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidTerms<Hash>(LoanTerms<Hash>);

impl<Hash> Deref for BidTerms<Hash> {
	type Target = LoanTerms<Hash>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Hash> TryFrom<LoanTerms<Hash>> for BidTerms<Hash> {
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms<Hash>) -> Result<Self, Self::Error> {
		if terms.term_length.is_zero() {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl<Hash> BidTerms<Hash> {
	pub fn match_with(&self, ask_terms: &AskTerms<Hash>) -> bool {
		ask_terms.match_with(self)
	}

	pub fn agreed_terms(self, ask_terms: &AskTerms<Hash>) -> Option<LoanTerms<Hash>> {
		ask_terms.agreed_terms(self)
	}
}

#[cfg(test)]
impl Default for LoanTerms<sp_core::H256> {
	fn default() -> Self {
		Self {
			amount: Default::default(),
			interest_rate: InterestRate::default(),
			term_length: Duration::from_millis(100_000),
			currency: CurrencyId::new::<crate::mock::Test>(&crate::Currency::default()),
		}
	}
}
#[cfg(test)]
impl Default for InterestRate {
	fn default() -> Self {
		Self {
			rate_per_period: 0,
			decimals: 1,
			period: Duration::from_millis(100_000),
			interest_type: InterestType::Simple,
		}
	}
}
