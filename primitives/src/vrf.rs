use num::rational::BigRational;
use num::ToPrimitive;
use parity_scale_codec::{Decode, Encode};
use sp_arithmetic::per_things::{PerThing, Perbill, Perquintill};
use sp_runtime_interface::pass_by::PassByCodec;
use sp_runtime_interface::runtime_interface;

mod model {
	use super::*;

	/// S is a hyperparameter representing participation rate.
	/// The higher the value, the higher the chances of being sampled.
	/// R is the prover's relative stake. the output is proportional to the stake.
	pub fn model(s: Perquintill, r: Perquintill) -> Perquintill {
		use pdf::probability_density_function;
		probability_density_function(s.into(), r.into()).0
	}

	#[derive(Encode, Decode, PassByCodec)]
	pub struct Wrap(Perquintill);

	impl From<Wrap> for f64 {
		fn from(w: Wrap) -> Self {
			w.0.to_sub_1_float()
		}
	}

	impl From<Perquintill> for Wrap {
		fn from(p: Perquintill) -> Self {
			Wrap(p)
		}
	}

	#[runtime_interface]
	trait Pdf {
		fn probability_density_function(sample: Wrap, weight: Wrap) -> Wrap {
			let complement = 1f64 - sample.0.to_sub_1_float();
			let r: f64 = weight.into();
			let f: f64 = complement.powf(r);
			Perquintill::from_float(1f64 - f).into()
		}
	}

	// Convert fixed point to f64, Accuracy depends on [PerThing::Inner]
	pub(super) trait ToFloat: PerThing {
		fn to_sub_1_float(&self) -> f64
		where
			<Self as PerThing>::Inner: Into<u64>,
		{
			let c = self.deconstruct();
			c.into() as f64 / Self::ACCURACY.into() as f64
		}
	}

	impl ToFloat for Perquintill {}
	impl ToFloat for Perbill {}

	#[cfg(test)]
	mod tests {
		use super::*;
		use proptest::prelude::*;

		#[test]
		fn model_should_return_expected_distribution() {
			let r = Perquintill::from_float(0.000_001);
			let c = Perquintill::from_float(0.5);

			assert!((model(c, r).to_sub_1_float() - 6.931_469_403_334_939E-7).abs() < f64::EPSILON);
		}

		proptest! {
			#[test]
			fn model_does_not_crash(
				sf in 0.0..=1.0,
				rf in 0.0..=1.0
			) {
				let s = Perquintill::from_float(sf);
				let r = Perquintill::from_float(rf);

				let result = model(s, r).to_sub_1_float();
				assert!(0.0 <= result);
			}
		}

		#[test]
		fn model_result_should_increase_when_argument_s_increases() {
			let r = Perquintill::from_float(0.000_001);
			let mut previous = -1.0;

			for i in 0..=1_000_000 {
				let sf = f64::from(i) * 0.000_001;
				let s = Perquintill::from_float(sf);
				let result = model(s, r).to_sub_1_float();

				assert!(
					previous < result,
					"previous={previous:?}, result={result:?}, s={s:?}, r={r:?}"
				);
				previous = result;
			}
		}

		#[test]
		fn model_result_should_increase_when_argument_r_increases() {
			let s = Perquintill::from_float(0.5);
			let mut previous = -1.0;

			for i in 0..=1_000_000 {
				let rf = f64::from(i) * 0.000_001;
				let r = Perquintill::from_float(rf);
				let result = model(s, r).to_sub_1_float();

				assert!(
					previous < result,
					"previous={previous:?}, result={result:?}, s={s:?}, r={r:?}"
				);
				previous = result;
			}
		}

		#[test]
		fn model_result_should_increase_when_both_arguments_increases() {
			let mut previous = -1.0;

			for i in 0..=1_000_000 {
				let xf = f64::from(i) * 0.000_001;
				let s = Perquintill::from_float(xf);
				let r = Perquintill::from_float(xf);
				let result = model(s, r).to_sub_1_float();

				assert!(
					previous < result,
					"previous={previous:?}, result={result:?}, s={s:?}, r={r:?}"
				);
				previous = result;
			}
		}

		#[test]
		fn converting_floats_forth_and_back_stays_within_epsilon_precision() {
			use rand::Rng;
			use sp_arithmetic::Perbill;
			let mut rng = rand::thread_rng();
			for _ in 0..1000 {
				let r: f64 = rng.gen();

				let x = Perquintill::from_float(r).to_sub_1_float();
				assert!((r - x).abs() < f64::EPSILON, "|{r} - {x}| < ε");

				let x = Perbill::from_float(r).to_sub_1_float();
				assert!((r - x).abs() < f32::EPSILON.into(), "|{r} - {x}| < ε");
			}
		}
	}
}

pub mod sortition {
	use super::*;
	use model::ToFloat;

	pub fn threshold(sample_size: Perquintill, stake: u128, total_stake: u128) -> u128 {
		let ratio = Perquintill::from_rational(stake, total_stake);
		let level = model::model(sample_size, ratio).to_sub_1_float();
		let level = BigRational::from_float(level).expect("Model codomain [0,1)");
		let threshold = u128::MAX / level.denom() * level.numer();
		threshold.to_u128().expect("n * X, X ~ [0,1), n: u128; qed.")
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn threshold_works() {
			let x = sortition::threshold(Perquintill::from_float(0.5), 1, 1);
			let y = u128::MAX / 2;
			assert_eq!(x, y)
		}
	}
}
