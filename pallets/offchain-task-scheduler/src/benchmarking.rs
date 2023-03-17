#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::authority::AuthorityController;
use crate::pallet::PendingTasks;
use crate::tasks::TaskScheduler;
use crate::Pallet;
use frame_benchmarking::{account, benchmarks};
use frame_support::traits::Hooks;
use frame_system::Call as SystemCall;
use frame_system::Config as SystemConfig;
use frame_system::RawOrigin;
use ocw::RuntimePublicOf;
use pallet_timestamp::Pallet as Timestamp;
use sp_core::sr25519::Public;
use sp_core::Hasher;
use sp_runtime::codec::Encode;
use sp_runtime::traits::One;
use sp_std::boxed::Box;
use sp_std::vec;

pub trait TaskDefault<T: SystemConfig> {
	fn generate_from_seed(seed: u32) -> Self;
}

benchmarks! {
	where_clause { where
		T::Task: TaskDefault<T>,
		RuntimePublicOf<T>: Into<T::Public> + AsRef<Public> + sp_std::fmt::Debug + Clone,
		T::TaskCall: From<SystemCall<T>>
	 }
	on_initialize {
		//insert t transfers
		let t in 0..1024;

		<Timestamp<T>>::set_timestamp(1u32.into());

		let deadline = T::BlockNumber::one();

		for i in 0..t {
			let task: T::Task = TaskDefault::<T>::generate_from_seed(i);
			let id = T::Hashing::hash(&task.encode());
			PendingTasks::<T>::insert(deadline, id, task);
		}

	}: { Pallet::<T>::on_initialize(deadline)}
	submit_output {

		<Timestamp<T>>::set_timestamp(1u32.into());

		let task: T::Task = TaskDefault::<T>::generate_from_seed(0);
		let deadline = Pallet::<T>::deadline();
		let id = T::Hashing::hash(&task.encode());

		let acc = account("dummy",0,0);
		Pallet::<T>::insert_authority(&acc);

	}:_(RawOrigin::Signed(acc),deadline,id,Box::new(SystemCall::<T>::remark{remark:vec![]}.into()))
}
