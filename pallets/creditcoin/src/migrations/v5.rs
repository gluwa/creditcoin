// task storage moved from UnverifiedTransfers + UnverifiedCollectedCoins to PendingTasks
use crate::{
	types::{self, ExternalAddress, ExternalTxId},
	CollectedCoinsId, Config, TransferId, UnverifiedTransfer,
};
use codec::{Decode, Encode};

use frame_support::{generate_storage_alias, migration, pallet_prelude::*, Identity};

pub use super::v4::*;

mod old_type {
	use super::*;

	#[derive(Encode, Decode)]
	pub struct UnverifiedCollectedCoins {
		pub to: ExternalAddress,
		pub tx_id: ExternalTxId,
	}
}

pub use super::v4::Transfer;

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
	CollectCoins(UnverifiedCc),
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>) -> Self {
		Task::VerifyTransfer(transfer)
	}
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedCc>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(coins: UnverifiedCc) -> Self {
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
		old_type::UnverifiedCollectedCoins
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

		let old_type::UnverifiedCollectedCoins { to, tx_id } = collect_coins;
		let new_item = types::UnverifiedCollectedCoins { contract: Default::default(), to, tx_id };

		crate::PendingTasks::<T>::insert(
			deadline,
			crate::TaskId::from(id),
			crate::Task::from(new_item),
		);
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

	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		ExternalTxId, TransferId,
	};

	use super::{
		Blockchain, OrderId, PendingTasks, Task, TaskId, Transfer, TransferKind,
		UnverifiedCollectedCoins, UnverifiedTransfer, UnverifiedTransfers,
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
			let old_collect_coins = old_type::UnverifiedCollectedCoins {
				to: b"baba".to_vec().try_into().unwrap(),
				tx_id: tx_id.clone(),
			};

			let new_collect_coins = types::UnverifiedCollectedCoins {
				to: b"baba".to_vec().try_into().unwrap(),
				tx_id: tx_id.clone(),
				contract: Default::default(),
			};

			let collect_coins_id =
				crate::CollectedCoinsId::new::<Test>(&new_collect_coins.contract.chain, &tx_id);

			UnverifiedCollectedCoins::<Test>::insert(
				deadline,
				&collect_coins_id,
				&old_collect_coins,
			);

			assert!(UnverifiedCollectedCoins::<Test>::contains_key(deadline, &collect_coins_id));

			super::migrate::<Test>();

			assert_eq!(
				PendingTasks::<Test>::get(deadline, TaskId::CollectCoins(collect_coins_id.clone())),
				Some(Task::CollectCoins(new_collect_coins))
			);

			assert!(
				UnverifiedCollectedCoins::<Test>::contains_key(deadline, collect_coins_id).not()
			);
		});
	}
}
