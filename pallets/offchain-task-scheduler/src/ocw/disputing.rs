use sp_runtime::DispatchError;

use crate::Config;
use core::marker::PhantomData;

//implemented by task?? No, swap the behavior by switching AT.
pub trait Disputable {
	type Item: Clone;
	type ItemId;
	type Who;

	fn disputable(id: &Self::ItemId) -> bool;
	fn disagree(id: &Self::ItemId, item: &Self::Item) -> bool;
	fn vote_on(
		who: &Self::Who,
		id: &Self::ItemId,
		item: &Self::Item,
	) -> Result<Option<Self::Item>, DispatchError>;
	fn clear(id: &Self::ItemId);
}

pub struct NeverDisputable<T>(PhantomData<T>);
impl<T: Config> Disputable for NeverDisputable<T> {
	type Who = T::AccountId;
	type ItemId = T::Hash;
	type Item = T::TaskCall;

	fn disputable(_task_id: &T::Hash) -> bool {
		false
	}
	fn disagree(_task_id: &T::Hash, _task_output: &T::TaskCall) -> bool {
		false
	}
	fn vote_on(
		_who: &T::AccountId,
		_id: &T::Hash,
		item: &T::TaskCall,
	) -> Result<Option<T::TaskCall>, DispatchError> {
		Ok(Some(item.clone()))
	}
	fn clear(_id: &Self::ItemId) {}
}
