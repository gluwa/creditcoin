#![cfg(feature = "runtime-benchmarks")]
//! Benchmarking setup for pallet-template

// use super::*;

#[allow(unused)]
use crate::{BlockAuthor, Config, Pallet as Rewards, Pallet};

use frame_benchmarking::{account, benchmarks};
use frame_support::traits::{OnFinalize, OnInitialize};
use sp_runtime::traits::{Bounded, One};

benchmarks! {
	on_initialize{}:{Rewards::<T>::on_initialize(T::BlockNumber::max_value());}

	on_finalize { //put author and whitelist
		let author:T::AccountId = account::<T::AccountId>("caller",1,1);

		<BlockAuthor<T>>::put(author);
		let state_key = crate::BlockAuthor::<T>::hashed_key().to_vec();
		frame_benchmarking::benchmarking::add_to_whitelist(state_key.into());

	}:{ Rewards::<T>::on_finalize(T::BlockNumber::one()); }
}

//impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
