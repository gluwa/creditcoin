// address registration now verifies ownership, so removed existing addresses
use super::v3;
use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::Blake2_128Concat;
use frame_support::{dispatch::Weight, generate_storage_alias};
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

generate_storage_alias!(
	Creditcoin,
	Addresses<T: Config> => Map<(Blake2_128Concat, AddressId<T::Hash>), Address<T::AccountId>>
);

pub(crate) fn migrate<T: Config>() -> Weight {
	let count_removed = match Addresses::<T>::remove_all(None) {
		sp_io::KillStorageResult::AllRemoved(count) => count,
		sp_io::KillStorageResult::SomeRemaining(count) => count,
	};

	T::DbWeight::get().writes(count_removed.saturated_into())
}

#[cfg(test)]
mod tests {
	use super::{Address, Addresses, Blockchain};
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
		fn new_old<T: frame_system::Config>(blockchain: &Blockchain, address: &[u8]) -> AddressId<H>
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
				let id =
					AddressId::<H256>::new_old::<Test>(&Blockchain::Ethereum, &i.to_be_bytes());
				let address = Address {
					blockchain: Blockchain::Ethereum,
					value: i.to_be_bytes().to_vec().try_into().unwrap(),
					owner: AccountId::new([i; 32]),
				};
				Addresses::<Test>::insert(&id, address);
				ids.push(id);
			}

			super::migrate::<Test>();

			for id in ids {
				assert!(!Addresses::<Test>::contains_key(id));
			}
		});
	}
}
