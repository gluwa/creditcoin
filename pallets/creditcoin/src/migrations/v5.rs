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

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	for (deadline, id, transfer) in UnverifiedTransfers::<T>::iter() {
		weight = weight.saturating_add(weight_each);

		crate::PendingTasks::<T>::insert(
			deadline,
			crate::TaskId::from(id),
			crate::Task::from(transfer),
		);
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

	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		ExternalTxId, TransferId,
	};

	use super::*;

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

			super::migrate::<Test>();

			assert_eq!(
				crate::PendingTasks::<Test>::get(
					deadline,
					crate::TaskId::VerifyTransfer(transfer_id.clone())
				),
				Some(crate::Task::VerifyTransfer(transfer))
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
				crate::PendingTasks::<Test>::get(
					deadline,
					crate::TaskId::CollectCoins(collect_coins_id.clone())
				),
				Some(crate::Task::CollectCoins(new_collect_coins))
			);

			assert!(
				UnverifiedCollectedCoins::<Test>::contains_key(deadline, collect_coins_id).not()
			);
		});
	}
}
