// address registration now verifies ownership, so removed existing addresses
use super::{v3, AccountIdOf, HashOf};
use super::{Migrate, PhantomData};
use frame_support::{
	dispatch::Weight,
	storage_alias,
	traits::{Get, StorageVersion},
	Blake2_128Concat,
};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::SaturatedConversion;

use crate::{Config, ExternalAddress};
pub use v3::*;

use crate::AddressId;
use v3::Blockchain as OldBlockchain;

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Address<AccountId> {
	pub blockchain: OldBlockchain,
	pub value: ExternalAddress,
	pub owner: AccountId,
}

#[storage_alias]
type Addresses<T: Config> =
	StorageMap<crate::Pallet<T>, Blake2_128Concat, AddressId<HashOf<T>>, Address<AccountIdOf<T>>>;

pub(crate) struct Migration<Runtime>(pub PhantomData<Runtime>);

impl<Runtime> Migration<Runtime> {
	pub(super) fn new() -> Self {
		Self(PhantomData::<Runtime>)
	}
}

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) {}

	fn migrate(&self) -> Weight {
		let sp_io::MultiRemovalResults { unique: count_removed, .. } =
			Addresses::<T>::clear(u32::MAX, None);

		T::DbWeight::get().writes(count_removed.saturated_into())
	}

	fn post_upgrade(&self) {
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
	use super::{Address, Addresses, OldBlockchain};
	use crate::{
		concatenate,
		mock::{AccountId, ExtBuilder, Test},
		AddressId,
	};
	use sp_core::H256;
	use sp_runtime::traits::Hash as HashT;
	use sp_std::convert::TryInto;

	#[extend::ext]
	impl<H> AddressId<H> {
		fn with_old_blockchain<T: frame_system::Config>(
			blockchain: &OldBlockchain,
			address: &[u8],
		) -> AddressId<H>
		where
			<T as frame_system::Config>::Hashing: HashT<Output = H>,
		{
			let key = concatenate!(blockchain.as_bytes(), address);
			AddressId::make(T::Hashing::hash(&key))
		}
	}

	#[test]
	fn migrate_works() {
		ExtBuilder::default().build_and_execute(|| {
			let mut ids = Vec::new();
			for i in 0u8..10u8 {
				let id = AddressId::<H256>::with_old_blockchain::<Test>(
					&OldBlockchain::Ethereum,
					&i.to_be_bytes(),
				);
				let address = Address {
					blockchain: OldBlockchain::Ethereum,
					value: i.to_be_bytes().to_vec().try_into().unwrap(),
					owner: AccountId::new([i; 32]),
				};
				Addresses::<Test>::insert(&id, address);
				ids.push(id);
			}

			super::Migration::<Test>::new().migrate();

			for id in ids {
				assert!(!Addresses::<Test>::contains_key(id));
			}
		});
	}
}
