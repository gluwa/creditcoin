use super::pallet::{Entries, Error, Index};
use super::Config;
use super::Pallet;
use pallet_offchain_task_scheduler::ocw::disputing::Disputable;
use sp_runtime::DispatchError;

impl<T: Config> Disputable for Pallet<T> {
	type Item = T::Item;
	type ItemId = T::ItemId;
	type Who = T::Who;

	fn disputable(id: &Self::ItemId) -> bool {
		Entries::<T>::get(id).is_some()
	}

	fn disagree(id: &Self::ItemId, item: &Self::Item) -> bool {
		Entries::<T>::get(id).map_or(false, |e| e.runners.keys().any(|i| i != item))
	}

	fn vote_on(
		who: &Self::Who,
		id: &Self::ItemId,
		item: &Self::Item,
	) -> Result<Option<Self::Item>, DispatchError> {
		todo!()
	}

	fn clear(id: &Self::ItemId) {
		Entries::<T>::remove(id);
		Index::<T>::remove(id);
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn disputable_returns_true_after_voting() {
		todo!()
	}

	#[test]
	fn disagree_returns_true_if_any_is_not_eq() {
		todo!()
	}
}
