#![cfg(feature = "runtime-benchmarks")]
//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Difficulty;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_arithmetic::traits::UniqueSaturatedFrom;

benchmarks! {
	set_target_block_time{
	}: _(RawOrigin::Root, <T as Config>::Moment::unique_saturated_from(100u64))
	verify {
		assert_eq!(TargetBlockTime::<T>::get(), <T as Config>::Moment::unique_saturated_from(100u64));
	}

	set_adjustment_period{
		let caller: T::AccountId = whitelisted_caller();
	}:_(RawOrigin::Root,100i64)
	verify{
		assert_eq!(DifficultyAdjustmentPeriod::<T>::get(),100i64);
	}

}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
