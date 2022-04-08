use core::ops::Deref;

use super::ExternalAmount;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::convert::TryFrom;

pub type InterestRate = u64;

pub const INTEREST_RATE_PRECISION: u64 = 10_000;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct LoanTerms<Moment> {
	pub amount: ExternalAmount,
	pub interest_rate: InterestRate,
	pub duration: Moment,
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
pub struct InvalidDurationError;

impl<T: crate::Config> From<InvalidDurationError> for crate::Error<T> {
	fn from(_: InvalidDurationError) -> Self {
		Self::InvalidDuration
	}
}

impl<Moment> TryFrom<LoanTerms<Moment>> for AskTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	type Error = InvalidDurationError;
	fn try_from(terms: LoanTerms<Moment>) -> Result<Self, Self::Error> {
		if terms.duration.unique_saturated_into() == 0 {
			return Err(InvalidDurationError);
		}

		Ok(Self(terms))
	}
}

impl<Moment> AskTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	pub fn match_with(&self, bid_terms: &BidTerms<Moment>) -> bool {
		self.amount == bid_terms.amount
			&& (self.interest_rate / self.maturity.unique_saturated_into())
				>= (bid_terms.interest_rate / bid_terms.maturity.unique_saturated_into())
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
	type Error = InvalidDurationError;
	fn try_from(terms: LoanTerms<Moment>) -> Result<Self, Self::Error> {
		if terms.duration.unique_saturated_into() == 0 {
			return Err(InvalidDurationError);
		}

		Ok(Self(terms))
	}
}

impl<Moment> BidTerms<Moment>
where
	Moment: UniqueSaturatedInto<u64> + Copy,
{
	pub fn match_with(&self, ask_terms: &AskTerms<Moment>) -> bool {
		ask_terms.match_with(self)
	}

	pub fn agreed_terms(self, ask_terms: &AskTerms<Moment>) -> Option<LoanTerms<Moment>> {
		ask_terms.agreed_terms(self)
	}
}

#[cfg(test)]
mod tests {
	use super::calc_interest;
	use crate::ExternalAmount;
	use ethereum_types::U256;

	#[test]
	pub fn test_calc_interest() {
		let principal_amount = ExternalAmount::from(100_000u64);
		let interest_rate_bps = 1_000;
		let interest = calc_interest(&principal_amount, interest_rate_bps);
		assert_eq!(interest, U256::from(10_000u64));
	}
}
