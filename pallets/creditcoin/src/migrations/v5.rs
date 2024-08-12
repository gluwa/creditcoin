// task storage moved from UnverifiedTransfers + UnverifiedCollectedCoins to PendingTasks
use super::{vec, Vec};
use super::{AccountIdOf, BlockNumberOf, HashOf, MomentOf};
use super::{Migrate, PhantomData};
use crate::{
	types::{ExternalAddress, ExternalTxId, TaskId},
	CollectedCoinsId, Config, TransferId, UnverifiedTransfer,
};
use frame_support::{pallet_prelude::*, storage_alias, Identity};
use parity_scale_codec::{Decode, Encode};

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct UnverifiedCollectedCoinsStruct {
	pub to: ExternalAddress,
	pub tx_id: ExternalTxId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub enum Task<AccountId, BlockNum, Hash, Moment> {
	VerifyTransfer(UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>),
	CollectCoins(UnverifiedCollectedCoinsStruct),
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>) -> Self {
		Task::VerifyTransfer(transfer)
	}
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedCollectedCoinsStruct>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(coins: UnverifiedCollectedCoinsStruct) -> Self {
		Task::CollectCoins(coins)
	}
}

#[storage_alias]
type UnverifiedTransfers<T: Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Identity,
	BlockNumberOf<T>,
	Identity,
	TransferId<HashOf<T>>,
	UnverifiedTransfer<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

#[storage_alias]
type UnverifiedCollectedCoins<T: Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Identity,
	BlockNumberOf<T>,
	Identity,
	CollectedCoinsId<HashOf<T>>,
	UnverifiedCollectedCoinsStruct,
>;

#[storage_alias]
pub type PendingTasks<T: Config> = StorageDoubleMap<
	crate::Pallet<T>,
	Identity,
	BlockNumberOf<T>,
	Identity,
	TaskId<HashOf<T>>,
	Task<AccountIdOf<T>, BlockNumberOf<T>, HashOf<T>, MomentOf<T>>,
>;

pub(super) struct Migration<Runtime>(PhantomData<Runtime>);

impl<Runtime> Migration<Runtime> {
	pub(super) fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T: Config> Migrate for Migration<T> {
	fn pre_upgrade(&self) -> Vec<u8> {
		vec![]
	}

	fn migrate(&self) -> Weight {
		let mut weight: Weight = Weight::zero();
		let weight_each = T::DbWeight::get().reads_writes(1, 1);

		for (deadline, id, transfer) in UnverifiedTransfers::<T>::iter() {
			weight = weight.saturating_add(weight_each);

			PendingTasks::<T>::insert(deadline, TaskId::from(id), Task::from(transfer));
		}

		let _results = UnverifiedTransfers::<T>::clear(u32::MAX, None);
		let _results = UnverifiedCollectedCoins::<T>::clear(u32::MAX, None);
		weight
	}

	fn post_upgrade(&self, _ctx: Vec<u8>) {
		assert_eq!(
			StorageVersion::get::<crate::Pallet<T>>(),
			5,
			"expected storage version to be 5 after migrations complete"
		);
	}
}

#[cfg(test)]
mod tests {
	use core::{convert::TryInto, ops::Not};

	use super::*;
	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		ExternalTxId, TransferId,
	};
	use sp_runtime::traits::Hash;

	#[test]
	fn unverified_transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let eth = crate::Blockchain::Ethereum;
			let deadline = 11;
			let tx_id: ExternalTxId = b"fafafafafafafa".to_vec().try_into().unwrap();
			let transfer = crate::UnverifiedTransfer {
				transfer: crate::Transfer {
					blockchain: eth.clone(),
					kind: crate::TransferKind::Native,
					from: crate::AddressId::new::<Test>(&eth, b"fromaddr"),
					to: crate::AddressId::new::<Test>(&eth, b"toaddr"),
					order_id: crate::OrderId::Deal(crate::DealOrderId::dummy()),
					amount: 1.into(),
					tx_id: tx_id.clone(),
					block: 1,
					is_processed: false,
					account_id: test_info.lender.account_id,
					timestamp: None,
				},
				from_external: b"baba".to_vec().try_into().unwrap(),
				to_external: b"abab".to_vec().try_into().unwrap(),
				deadline: 11,
			};
			let transfer_id = TransferId::new::<Test>(&eth, &tx_id);

			UnverifiedTransfers::<Test>::insert(deadline, &transfer_id, &transfer);
			assert!(UnverifiedTransfers::<Test>::contains_key(deadline, &transfer_id));

			super::Migration::<Test>::new().migrate();

			assert_eq!(
				PendingTasks::<Test>::get(
					deadline,
					crate::TaskId::VerifyTransfer(transfer_id.clone())
				),
				Some(Task::VerifyTransfer(transfer))
			);

			assert!(UnverifiedTransfers::<Test>::contains_key(deadline, transfer_id).not());
		});
	}

	#[test]
	fn unverified_collected_coins_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let deadline = 11;
			let tx_id: ExternalTxId = b"fafafafafafafa".to_vec().try_into().unwrap();
			let collect_coins = UnverifiedCollectedCoinsStruct {
				to: b"baba".to_vec().try_into().unwrap(),
				tx_id: tx_id.clone(),
			};

			let collect_coins_id = crate::CollectedCoinsId::from(
				<Test as frame_system::Config>::Hashing::hash(&tx_id),
			);

			UnverifiedCollectedCoins::<Test>::insert(deadline, &collect_coins_id, collect_coins);

			assert!(UnverifiedCollectedCoins::<Test>::contains_key(deadline, &collect_coins_id));

			super::Migration::<Test>::new().migrate();

			assert!(
				UnverifiedCollectedCoins::<Test>::contains_key(deadline, collect_coins_id).not()
			);
		});
	}
}
