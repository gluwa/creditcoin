use super::pallet;
use super::Config;

pub trait AuthorityController<AccountId> {
	fn insert_authority(authority: &AccountId);
	fn remove_authority(authority: &AccountId);
	fn is_authority(authority: &AccountId) -> bool;
}

impl<Runtime: Config> AuthorityController<Runtime::AccountId> for Runtime {
	fn insert_authority(authority: &Runtime::AccountId) {
		pallet::Authorities::<Runtime>::insert(authority, ());
	}
	fn remove_authority(authority: &Runtime::AccountId) {
		pallet::Authorities::<Runtime>::remove(authority);
	}
	fn is_authority(authority: &Runtime::AccountId) -> bool {
		pallet::Authorities::<Runtime>::contains_key(authority)
	}
}

#[cfg(test)]
mod tests {
	use crate::authority::AuthorityController;
	use crate::mock::{
		runtime::{AccountId, Runtime},
		ExtBuilder,
	};

	#[test]
	fn insert_check_and_remove() {
		ExtBuilder::default().build().execute_with(|| {
			let account: AccountId = AccountId::new([0; 32]);

			assert!(!Runtime::is_authority(&account));

			Runtime::insert_authority(&account);

			let value = crate::Pallet::<Runtime>::authorities(&account);
			assert_eq!(value, Some(()));

			assert!(Runtime::is_authority(&account));

			Runtime::remove_authority(&account);
			let value = crate::Pallet::<Runtime>::authorities(&account);
			assert_eq!(value, None)
		});
	}
}
