#![cfg_attr(not(feature = "std"), no_std)]
pub mod impls;
pub mod pallet;
pub mod sampling;
pub mod types;

pub use pallet::{Config, Pallet};
use sp_arithmetic::traits::{Bounded, CheckedMul, One, Zero};

pub trait VotingPowerProvider {
	type Unit: Clone + Zero + One + PartialOrd + CheckedMul + Bounded;
	type Who;
	type ItemId;

	fn power(who: &Self::Who, task_id: &Self::ItemId) -> Self::Unit;
}
