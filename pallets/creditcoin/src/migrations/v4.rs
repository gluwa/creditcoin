// address registration now verifies ownership, so removed existing addresses

use frame_support::dispatch::Weight;
use frame_support::traits::Get;
use sp_runtime::SaturatedConversion;

pub use super::v3::*;

use crate::Config;

pub(crate) fn migrate<T: Config>() -> Weight {
	let count_removed = match crate::Addresses::<T>::remove_all(None) {
		sp_io::KillStorageResult::AllRemoved(count) => count,
		sp_io::KillStorageResult::SomeRemaining(count) => count,
	};

	T::DbWeight::get().writes(count_removed.saturated_into())
}

#[cfg(test)]
mod tests {
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

			super::migrate::<Test>();

			for id in ids {
				assert!(!crate::Addresses::<Test>::contains_key(id));
			}
		});
	}
}
