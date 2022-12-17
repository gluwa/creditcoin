#![cfg(feature = "runtime-benchmarks")]
// use super::*;

#[allow(unused)]
use crate::{BlockAuthor, Config, Pallet as Rewards, Pallet};

use frame_benchmarking::{account, benchmarks};
use frame_support::traits::{OnFinalize, OnInitialize};
use sp_runtime::traits::{Bounded, One};

benchmarks! {}

//impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
