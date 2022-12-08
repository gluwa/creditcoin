#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::pallet::PendingTasks;
use crate::Pallet;
use frame_benchmarking::benchmarks;
use frame_support::traits::Hooks;
use frame_system::Config as SystemConfig;
use pallet_timestamp::Pallet as Timestamp;
use sp_core::Hasher;
use sp_runtime::codec::Encode;
use sp_runtime::traits::One;

pub trait TaskDefault<T: SystemConfig> {
	fn generate_from_seed(seed: u32) -> Self;
}

benchmarks! {
	where_clause {  where T::Task: TaskDefault<T> }
	on_initialize {
		//insert t transfers
		let t in 0..1024;

		<Timestamp<T>>::set_timestamp(1u32.into());

		let deadline = T::BlockNumber::one();

		for i in 0..t {
			let task: T::Task = TaskDefault::<T>::generate_from_seed(i);
			let id = T::Hashing::hash(&task.encode());
			PendingTasks::<T>::insert(&deadline, &id, task);
		}

	}: { Pallet::<T>::on_initialize(deadline)}
}
