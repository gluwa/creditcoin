// address registration now verifies ownership, so removed existing addresses
use super::{vec, Vec};
use super::{Migrate, PhantomData};
use crate::Config;
use frame_support::{
	dispatch::Weight,
	traits::{Get, StorageVersion},
};
use sp_runtime::SaturatedConversion;

impl<Runtime> Migration<Runtime> {
	pub(super) fn new() -> Self {
		Self(PhantomData)
	}
}

pub(super) struct Migration<Runtime>(PhantomData<Runtime>);

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn migrate(&self) -> Weight {
		let sp_io::MultiRemovalResults { unique: count_removed, .. } =
			crate::Addresses::<T>::clear(u32::MAX, None);

		T::DbWeight::get().writes(count_removed.saturated_into())
	}

	fn post_upgrade(&self, _ctx: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			4,
			"expected storage version to be 4 after migrations complete"
		);
	}
}

#[cfg(test)]
mod tests {
	use super::Migrate;
	use crate::{
		mock::{AccountId, ExtBuilder, Test},
		Address, AddressId,
	};
	use sp_core::H256;
	use sp_std::convert::TryInto;

	#[test]
	fn migrate_works() {
		ExtBuilder::default().build_and_execute(|| {
			let mut ids = Vec::new();
			for i in 0u8..10u8 {
				let id =
					AddressId::<H256>::new::<Test>(&crate::Blockchain::Ethereum, &i.to_be_bytes());
				let address = Address {
					blockchain: crate::Blockchain::Ethereum,
					value: i.to_be_bytes().to_vec().try_into().unwrap(),
					owner: AccountId::new([i; 32]),
				};
				crate::Addresses::<Test>::insert(&id, address);
				ids.push(id);
			}

			super::Migration::<Test>::new().migrate();

			for id in ids {
				assert!(!crate::Addresses::<Test>::contains_key(id));
			}
		});
	}
}
