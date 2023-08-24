use super::pallet::{Authorities, Pallet};
use super::Config;

pub trait AuthorityController {
	type AccountId;
	fn insert_authority(authority: &Self::AccountId);
	fn remove_authority(authority: &Self::AccountId);
	fn is_authority(authority: &Self::AccountId) -> bool;
}

impl<Runtime: Config> AuthorityController for Pallet<Runtime> {
	type AccountId = Runtime::AccountId;

	fn insert_authority(authority: &Self::AccountId) {
		Authorities::<Runtime>::insert(authority, ());
	}
	fn remove_authority(authority: &Self::AccountId) {
		Authorities::<Runtime>::remove(authority);
	}
	fn is_authority(authority: &Self::AccountId) -> bool {
		Authorities::<Runtime>::contains_key(authority)
	}
}

#[cfg(test)]
mod tests {
	use crate::authority::AuthorityController;
	use crate::mock::runtime::{AccountId, Runtime, TaskScheduler};
	use runtime_utils::ExtBuilder;

	#[test]
	fn insert_check_and_remove() {
		ExtBuilder::default().build::<Runtime>().execute_with(|| {
			let account: AccountId = AccountId::new([0; 32]);

			assert!(!TaskScheduler::is_authority(&account));

			TaskScheduler::insert_authority(&account);

			let value = TaskScheduler::authorities(&account);
			assert_eq!(value, Some(()));

			assert!(TaskScheduler::is_authority(&account));

			TaskScheduler::remove_authority(&account);
			let value = TaskScheduler::authorities(&account);
			assert_eq!(value, None)
		});
	}
}
