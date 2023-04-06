// task storage moved from UnverifiedTransfers + UnverifiedCollectedCoins to PendingTasks
pub use super::v4::Transfer;
pub use super::v4::*;
use super::{vec, Vec};
use super::{AccountIdOf, BlockNumberOf, HashOf, MomentOf};
use super::{Migrate, PhantomData};
use crate::{
	types::{self, ExternalAddress, ExternalTxId},
	CollectedCoinsId, Config, TransferId, UnverifiedTransfer,
};
use frame_support::{pallet_prelude::*, storage_alias, Identity};
use parity_scale_codec::{Decode, Encode};

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

		for (deadline, id, collect_coins) in UnverifiedCollectedCoins::<T>::iter() {
			weight = weight.saturating_add(weight_each);

			PendingTasks::<T>::insert(deadline, TaskId::from(id), Task::from(collect_coins));
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

			super::Migration::<Test>::new().migrate();

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

			let collect_coins_id = crate::CollectedCoinsId::from(
				<Test as frame_system::Config>::Hashing::hash(&tx_id),
			);

			UnverifiedCollectedCoins::<Test>::insert(
				deadline,
				&collect_coins_id,
				&old_collect_coins,
			);

			assert!(UnverifiedCollectedCoins::<Test>::contains_key(deadline, &collect_coins_id));

			super::Migration::<Test>::new().migrate();

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
