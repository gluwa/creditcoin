// task storage moved from UnverifiedTransfers + UnverifiedCollectedCoins to PendingTasks
use crate::{
	types::{ExternalAddress, ExternalTxId},
	CollectedCoinsId, Config, TransferId,
};
use parity_scale_codec::{Decode, Encode};

use frame_support::{generate_storage_alias, migration, pallet_prelude::*, Identity};

pub use super::v4::Transfer;
pub use super::v4::*;

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct UnverifiedCollectedCoinsStruct {
	pub to: ExternalAddress,
	pub tx_id: ExternalTxId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> {
	pub transfer: Transfer<AccountId, BlockNum, Hash, Moment>,
	pub from_external: ExternalAddress,
	pub to_external: ExternalAddress,
	pub deadline: BlockNum,
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

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub enum TaskId<Hash> {
	VerifyTransfer(TransferId<Hash>),
	CollectCoins(CollectedCoinsId<Hash>),
}

impl<Hash> From<TransferId<Hash>> for TaskId<Hash> {
	fn from(id: TransferId<Hash>) -> Self {
		TaskId::VerifyTransfer(id)
	}
}

impl<Hash> From<CollectedCoinsId<Hash>> for TaskId<Hash> {
	fn from(id: CollectedCoinsId<Hash>) -> Self {
		TaskId::CollectCoins(id)
	}
}

generate_storage_alias!(
	Creditcoin,
	UnverifiedTransfers<T: Config> => DoubleMap<
		(Identity, T::BlockNumber),
		(Identity, TransferId<T::Hash>),
		UnverifiedTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>
	>
);

generate_storage_alias!(
	Creditcoin,
	UnverifiedCollectedCoins<T: Config> => DoubleMap<
		(Identity, T::BlockNumber),
		(Identity, CollectedCoinsId<T::Hash>),
		UnverifiedCollectedCoinsStruct
	>
);

generate_storage_alias!(
	Creditcoin,
	PendingTasks<T: Config> => DoubleMap<
		(Identity, T::BlockNumber),
		(Identity, TaskId<T::Hash>),
		Task<T::AccountId, T::BlockNumber, T::Hash, T::Moment>
	>
);

/*
#[pallet::storage]
#[pallet::getter(fn pending_tasks)]
pub type PendingTasks<T: Config> = StorageDoubleMap<
	_,
	Identity,
	T::BlockNumber,
	Identity,
	TaskId<T::Hash>,
	Task<T::AccountId, T::BlockNumber, T::Hash, T::Moment>,
>; */

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	for (deadline, id, transfer) in UnverifiedTransfers::<T>::iter() {
		weight = weight.saturating_add(weight_each);

		PendingTasks::<T>::insert(deadline, TaskId::from(id), Task::from(transfer));
	}

	for (deadline, id, collect_coins) in UnverifiedCollectedCoins::<T>::iter() {
		weight = weight.saturating_add(weight_each);

		PendingTasks::<T>::insert(deadline, TaskId::from(id), Task::from(collect_coins));
	}

	let module = crate::Pallet::<T>::name().as_bytes();
	migration::remove_storage_prefix(module, b"UnverifiedTransfers", b"");
	migration::remove_storage_prefix(module, b"UnverifiedCollectedCoins", b"");

	weight
}

#[cfg(test)]
mod tests {
	use core::{convert::TryInto, ops::Not};

	use ethereum_types::H256;

	use sp_runtime::traits::Hash;

	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		ExternalTxId, TransferId,
	};

	use super::{
		Blockchain, OrderId, PendingTasks, Task, TaskId, Transfer, TransferKind,
		UnverifiedCollectedCoins, UnverifiedCollectedCoinsStruct, UnverifiedTransfer,
		UnverifiedTransfers,
	};

	#[test]
	fn unverified_transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let eth = Blockchain::Ethereum;
			let deadline = 11;
			let tx_id: ExternalTxId = b"fafafafafafafa".to_vec().try_into().unwrap();
			let transfer = UnverifiedTransfer {
				transfer: Transfer {
					blockchain: eth.clone(),
					kind: TransferKind::Native,
					from: crate::AddressId::make(H256::random()),
					to: crate::AddressId::make(H256::random()),
					order_id: OrderId::Deal(crate::DealOrderId::dummy()),
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
			let transfer_id = TransferId::from_old_blockchain::<Test>(&eth, &tx_id);

			UnverifiedTransfers::<Test>::insert(deadline, &transfer_id, &transfer);
			assert!(UnverifiedTransfers::<Test>::contains_key(deadline, &transfer_id));

			super::migrate::<Test>();

			assert_eq!(
				PendingTasks::<Test>::get(deadline, TaskId::VerifyTransfer(transfer_id.clone())),
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

			let collect_coins_id = crate::CollectedCoinsId::make(
				<Test as frame_system::Config>::Hashing::hash(&tx_id),
			);

			UnverifiedCollectedCoins::<Test>::insert(deadline, &collect_coins_id, &collect_coins);

			assert!(UnverifiedCollectedCoins::<Test>::contains_key(deadline, &collect_coins_id));

			super::migrate::<Test>();

			assert_eq!(
				PendingTasks::<Test>::get(deadline, TaskId::CollectCoins(collect_coins_id.clone())),
				Some(Task::CollectCoins(collect_coins))
			);

			assert!(
				UnverifiedCollectedCoins::<Test>::contains_key(deadline, collect_coins_id).not()
			);
		});
	}
}
