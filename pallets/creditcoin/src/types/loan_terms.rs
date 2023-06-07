use core::ops::Deref;

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
pub struct LoanTerms {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length: Duration,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskTerms(LoanTerms);

impl Deref for AskTerms {
	type Target = LoanTerms;

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

impl TryFrom<LoanTerms> for AskTerms {
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms) -> Result<Self, Self::Error> {
		if terms.term_length.is_zero() {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl AskTerms {
	pub fn match_with(&self, bid_terms: &BidTerms) -> bool {
		self.amount == bid_terms.amount
			&& self.interest_rate == bid_terms.interest_rate
			&& self.term_length == bid_terms.term_length
	}

	pub fn agreed_terms(&self, bid_terms: BidTerms) -> Option<LoanTerms> {
		self.match_with(&bid_terms).then_some(bid_terms.0)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidTerms(LoanTerms);

impl Deref for BidTerms {
	type Target = LoanTerms;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl TryFrom<LoanTerms> for BidTerms {
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms) -> Result<Self, Self::Error> {
		if terms.term_length.is_zero() {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl BidTerms {
	pub fn match_with(&self, ask_terms: &AskTerms) -> bool {
		ask_terms.match_with(self)
	}

	pub fn agreed_terms(self, ask_terms: &AskTerms) -> Option<LoanTerms> {
		ask_terms.agreed_terms(self)
	}
}

#[cfg(test)]
impl Default for LoanTerms {
	fn default() -> Self {
		Self {
			amount: Default::default(),
			interest_rate: InterestRate::default(),
			term_length: Duration::from_millis(100_000),
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
