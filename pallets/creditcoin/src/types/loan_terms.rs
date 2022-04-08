use core::ops::Deref;

use super::ExternalAmount;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::convert::TryFrom;

pub type RatePerPeriod = u64;
pub type Decimals = u64;
pub type Period = u64;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct InterestRate {
	pub rate_per_period: RatePerPeriod,
	pub decimals: Decimals,
	pub period_ms: Period,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct LoanTerms<Moment> {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub term_length_ms: Moment,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AskTerms<Moment>(LoanTerms<Moment>);

impl<Moment> Deref for AskTerms<Moment> {
	type Target = LoanTerms<Moment>;

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

impl<Moment> TryFrom<LoanTerms<Moment>> for AskTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms<Moment>) -> Result<Self, Self::Error> {
		if terms.term_length_ms.unique_saturated_into() == 0 {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl<Moment> AskTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy + PartialEq,
{
	pub fn match_with(&self, bid_terms: &BidTerms<Moment>) -> bool {
		self.amount == bid_terms.amount
			&& self.interest_rate == bid_terms.interest_rate
			&& self.term_length_ms == bid_terms.term_length_ms
	}

	pub fn agreed_terms(&self, bid_terms: BidTerms<Moment>) -> Option<LoanTerms<Moment>> {
		self.match_with(&bid_terms).then(|| bid_terms.0)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct BidTerms<Moment>(LoanTerms<Moment>);

impl<Moment> Deref for BidTerms<Moment> {
	type Target = LoanTerms<Moment>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<Moment> TryFrom<LoanTerms<Moment>> for BidTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	type Error = InvalidTermLengthError;
	fn try_from(terms: LoanTerms<Moment>) -> Result<Self, Self::Error> {
		if terms.term_length_ms.unique_saturated_into() == 0 {
			return Err(InvalidTermLengthError);
		}

		Ok(Self(terms))
	}
}

impl<Moment> BidTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy + PartialEq,
{
	pub fn match_with(&self, ask_terms: &AskTerms<Moment>) -> bool {
		ask_terms.match_with(self)
	}

	pub fn agreed_terms(self, ask_terms: &AskTerms<Moment>) -> Option<LoanTerms<Moment>> {
		ask_terms.agreed_terms(self)
	}
}

#[cfg(test)]
impl<Moment> Default for LoanTerms<Moment>
where
	Moment: sp_runtime::traits::One,
{
	fn default() -> Self {
		Self {
			amount: Default::default(),
			interest_rate: InterestRate { rate_per_period: 0, decimals: 1, period_ms: 1 },
			term_length_ms: Moment::one(),
		}
	}
}
#[cfg(test)]
impl Default for InterestRate {
	fn default() -> Self {
		Self { rate_per_period: 0, decimals: 1, period_ms: 1_000_000 }
	}
}
