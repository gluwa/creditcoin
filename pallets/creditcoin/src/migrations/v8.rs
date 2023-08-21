use super::Migrate;
use super::{vec, Vec};
use super::{AccountIdOf, BlockNumberOf, HashOf, MomentOf};

use crate::mock::{Balance, ExtBuilder};
use crate::types::{CollectedCoinsStruct, DeployedContract};

use crate::{
	loan_terms::{Decimals, Duration},
	AddressId, Blockchain, Config, ExternalAmount, OfferId, RatePerPeriod, TransferId,
};
use crate::{CollectedCoins, CollectedCoinsId, ExternalTxId};
use frame_support::weights::Weight;
use frame_support::{pallet_prelude::*, traits::Get};
use frame_support::{storage_alias, Identity, Twox64Concat};
use frame_system::Pallet;
use parity_scale_codec::{Decode, Encode};
use sp_runtime::traits::{Saturating, UniqueSaturatedInto};

#[derive(Clone, Encode, Decode)]
pub struct OldCollectedCoinsStruct<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	pub tx_id: ExternalTxId,
}

pub(super) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime: Config> Migration<Runtime> {
	pub(super) fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn post_upgrade(&self, _blob: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			9,
			"expected storage version to be 1 after migrations complete"
		);
	}
	fn migrate(&self) -> Weight {
		let mut weight: Weight = Weight::zero();

		let weight_each = T::DbWeight::get().reads_writes(1, 1);

		crate::CollectedCoins::<T>::translate::<OldCollectedCoinsStruct<T::Hash, T::Balance>, _>(
			|k: CollectedCoinsId<T::Hash>, y: OldCollectedCoinsStruct<T::Hash, T::Balance>| {
				weight = weight.saturating_add(weight_each);

				Some(CollectedCoinsStruct {
					to: y.to,
					amount: y.amount,
					tx_id: y.tx_id,
					contract_type: crate::ContractType::GCRE,
				})
			},
		);

		weight
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{self, Test};

	#[frame_support::storage_alias]
	type CollectedCoins<T: crate::Config> = StorageMap<
		crate::Pallet<T>,
		Identity,
		CollectedCoinsId<<mock::Test as frame_system::Config>::Hash>,
		OldCollectedCoinsStruct<
			<mock::Test as frame_system::Config>::Hash,
			<mock::Test as pallet_balances::Config>::Balance,
		>,
	>;

	type OldCollectedCoinsStorage = CollectedCoins<Test>;

	#[test]
	fn test_migrate_collected_coins_struct() {
		ExtBuilder::default().build_and_execute(|| {
			let tx_id = BoundedVec::default();

			let storage_id =
				crate::CollectedCoinsId::new::<crate::mock::Test>(&Blockchain::Bitcoin, &tx_id);

			let address: [u8; 20] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

			let to = AddressId::new::<crate::mock::Test>(&Blockchain::Ethereum, &address);
			let old = OldCollectedCoinsStruct { to, amount: 100, tx_id };

			OldCollectedCoinsStorage::insert(&storage_id, &old);

			super::Migration::<Test>::new().migrate();

			let new = super::CollectedCoins::<Test>::try_get(&storage_id).unwrap();

			assert_eq!(old.to, new.to);
			assert_eq!(old.amount, new.amount);
			assert_eq!(old.tx_id, new.tx_id);
			assert_eq!(new.contract_type, crate::ContractType::GCRE);
		})
	}
}
