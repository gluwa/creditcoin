use crate::Config;
use core::marker::PhantomData;
use frame_support::traits::Get;
use frame_system::Pallet as System;
use sp_core::{Decode, Encode};
use sp_runtime::offchain::storage::{StorageRetrievalError, StorageValueRef};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_runtime::Saturating;
use sp_std::vec::Vec;

pub trait OutputCache {
	type Id: Encode;
	type Output: Encode + Decode;

	fn set(id: &Self::Id, value: &Self::Output);

	fn get(id: &Self::Id) -> Result<Option<Self::Output>, StorageRetrievalError>;

	fn cache_key(id: &Self::Id) -> Vec<u8> {
		const DOMAIN_PREFIX: &[u8] = b"task-scheduler/task/cache/";
		id.using_encoded(|encoded_id| DOMAIN_PREFIX.iter().chain(encoded_id).copied().collect())
	}

	fn clear(id: &Self::Id) {
		let key = Self::cache_key(id);
		StorageValueRef::persistent(key.as_ref()).clear();
	}
}

pub struct NoCache<T>(PhantomData<T>);

impl<T: Config> OutputCache for NoCache<T> {
	type Id = T::Hash;
	type Output = T::TaskCall;

	fn set(_id: &Self::Id, _value: &Self::Output) {}

	fn get(_id: &Self::Id) -> Result<Option<Self::Output>, StorageRetrievalError> {
		Ok(None)
	}

	fn clear(_id: &Self::Id) {}
}

#[derive(Encode, Decode)]
pub struct BlockNumberCache<Item, Deadline, T> {
	deadline: Deadline,
	item: Item,
	_p: PhantomData<T>,
}

impl<T: Config> OutputCache for BlockNumberCache<T::TaskCall, T::BlockNumber, T> {
	type Id = T::Hash;
	type Output = T::TaskCall;

	fn set(id: &Self::Id, item: &Self::Output) {
		let key = Self::cache_key(id);
		let value_ref = StorageValueRef::persistent(key.as_ref());
		value_ref.set(&Self {
			deadline: T::UnverifiedTaskTimeout::get().saturating_sub(2u64.unique_saturated_into()),
			item: item.clone(),
			_p: PhantomData,
		});
	}

	fn get(id: &Self::Id) -> Result<Option<Self::Output>, StorageRetrievalError> {
		let key = Self::cache_key(id);
		let value_ref = StorageValueRef::persistent(key.as_ref());

		match value_ref.get::<Self>() {
			Ok(None) => Ok(None),
			Ok(Some(cache)) if cache.deadline < System::<T>::block_number() => Ok(None),
			Ok(Some(cache)) => Ok(Some(cache.item)),
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{
		generate_authority,
		runtime::{Runtime, TaskScheduler},
	};
	use crate::mocked_task::MockTask;
	use crate::ocw::lock_key;
	use crate::pallet::PendingTasks;
	use crate::tasks::task_lock;
	use crate::tasks::{TaskScheduler as TaskSchedulerT, TaskV2};
	use crate::Config;

	use runtime_utils::{
		pool::with_failing_submit_transaction, ExtBuilder, RollTo, Trivial, WithWorkerHook,
	};
	use sp_core::H256;

	#[test]
	fn cache_is_not_cleared_but_expires_at_task_expiration() {
		let mut ext_builder = ExtBuilder::default().with_keystore();
		generate_authority(&mut ext_builder, 0);
		let state = ext_builder.with_offchain();
		ext_builder.with_pool();
		ext_builder.build_sans_config().execute_with(|| {
			let mut height = 1;
			Trivial::<TaskScheduler, Runtime>::roll_to(height);

			let deadline = TaskScheduler::deadline();
			let task = MockTask::Remark(0);
			let id = TaskV2::<Runtime>::to_id(&task);

			assert_eq!(PendingTasks::<Runtime>::iter().count(), 0);
			assert_eq!(
				<Runtime as Config>::OutputCache::get(&id).unwrap(),
				None,
				"Unexpected cache item"
			);

			TaskScheduler::insert(&deadline, &id, task);
			assert_eq!(PendingTasks::<Runtime>::iter().count(), 1);

			height += 1;

			WithWorkerHook::<TaskScheduler, Runtime>::roll_to(height);

			<Runtime as Config>::OutputCache::get(&id)
				.unwrap()
				.expect("OCW cached the output");

			height += deadline + 1;

			WithWorkerHook::<TaskScheduler, Runtime>::roll_to(height);
			assert_eq!(PendingTasks::<Runtime>::iter().count(), 0);
			assert!(<Runtime as Config>::OutputCache::get(&id).unwrap().is_none(), "Cache Expires");
			let cache_key = <Runtime as Config>::OutputCache::cache_key(&id);
			state.read().persistent_storage.get(&cache_key).expect("cache");
		});
	}

	#[test]
	fn cache_is_cleared_after_failing_to_submit_txn() {
		let mut ext_builder = ExtBuilder::default().with_keystore();
		ext_builder.with_offchain();
		ext_builder.with_pool();
		generate_authority(&mut ext_builder, 0);
		ext_builder.build_sans_config().execute_with(|| {
			let mut height = 1;
			Trivial::<TaskScheduler, Runtime>::roll_to(height);

			let deadline = TaskScheduler::deadline();
			let task = MockTask::Remark(0);
			let id = TaskV2::<Runtime>::to_id(&task);

			TaskScheduler::insert(&deadline, &id, task);

			height += 1;

			with_failing_submit_transaction(|| {
				WithWorkerHook::<TaskScheduler, Runtime>::roll_to(height)
			});

			assert!(
				<Runtime as Config>::OutputCache::get(&id).unwrap().is_none(),
				"OCW submitted the call and cleared the cache"
			);
		});
	}

	#[test]
	fn task_lock_and_cache_expire_together_blockwise() {
		let mut ext_builder = ExtBuilder::default();
		let state = ext_builder.with_offchain();
		ext_builder.build_sans_config().execute_with(|| {
			let key = lock_key(&b"id");
			let mut lock = task_lock::<Runtime>(&key);
			lock.lock().forget();
			let mut lock = task_lock::<Runtime>(&key);
			let lock_deadline = lock.try_lock().map(|_| ()).expect_err("deadline").block_number;

			let key = H256::from_slice(&[0u8; 32]);
			<Runtime as Config>::OutputCache::set(
				&key,
				&frame_system::Call::remark { remark: ().encode() }.into(),
			);

			let cache_key = <Runtime as Config>::OutputCache::cache_key(&key);
			let input = state.read().persistent_storage.get(&cache_key).expect("Set");
			let cache_deadline = <Runtime as Config>::OutputCache::decode(&mut input.as_slice())
				.unwrap()
				.deadline;
			assert_eq!(cache_deadline, lock_deadline);
		});
	}
}
