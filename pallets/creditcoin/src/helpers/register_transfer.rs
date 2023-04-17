use crate::OrderId;
use crate::{
	pallet::*, types::AddressId, Error, ExternalAmount, ExternalTxId, Task, Transfer, TransferId,
	TransferKind, UnverifiedTransfer,
};
use frame_support::ensure;
use frame_system::pallet_prelude::*;
use frame_system::Config as SystemConfig;
use pallet_offchain_task_scheduler::tasks::TaskScheduler;
use pallet_offchain_task_scheduler::tasks::TaskV2;
use pallet_timestamp::Config as TimestampConfig;
use sp_std::prelude::*;

type UnverifiedTransferFor<T> = UnverifiedTransfer<
	<T as SystemConfig>::AccountId,
	BlockNumberFor<T>,
	<T as SystemConfig>::Hash,
	<T as TimestampConfig>::Moment,
>;

type TransferFor<T> = Transfer<
	<T as SystemConfig>::AccountId,
	BlockNumberFor<T>,
	<T as SystemConfig>::Hash,
	<T as TimestampConfig>::Moment,
>;

impl<T: Config> Pallet<T> {
	pub fn generate_transfer(
		who: T::AccountId,
		from_id: AddressId<T::Hash>,
		to_id: AddressId<T::Hash>,
		transfer_kind: TransferKind,
		amount: ExternalAmount,
		order_id: OrderId<T::BlockNumber, T::Hash>,
		blockchain_tx_id: ExternalTxId,
	) -> Result<(TransferFor<T>, UnverifiedTransferFor<T>), crate::Error<T>> {
		let from = Self::get_address(&from_id)?;
		let to = Self::get_address(&to_id)?;

		ensure!(from.owner == who, Error::<T>::NotAddressOwner);

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressBlockchainMismatch);

		let block = Self::block_number();

		let transfer = Transfer {
			blockchain: from.blockchain,
			kind: transfer_kind,
			amount,
			block,
			from: from_id,
			to: to_id,
			order_id,
			is_processed: false,
			account_id: who,
			tx_id: blockchain_tx_id,
			timestamp: None,
		};

		let deadline = T::TaskScheduler::deadline();

		Ok((
			transfer.clone(),
			UnverifiedTransfer {
				from_external: from.value,
				to_external: to.value,
				transfer,
				deadline,
			},
		))
	}

	pub fn register_transfer_internal(
		who: T::AccountId,
		from_id: AddressId<T::Hash>,
		to_id: AddressId<T::Hash>,
		transfer_kind: TransferKind,
		amount: ExternalAmount,
		order_id: OrderId<T::BlockNumber, T::Hash>,
		blockchain_tx_id: ExternalTxId,
	) -> Result<
		(TransferId<T::Hash>, Transfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>),
		crate::Error<T>,
	> {
		let from = Self::get_address(&from_id)?;
		let to = Self::get_address(&to_id)?;

		ensure!(from.owner == who, Error::<T>::NotAddressOwner);

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressBlockchainMismatch);

		ensure!(from.blockchain.supports(&transfer_kind), Error::<T>::UnsupportedTransferKind);

		let transfer = Transfer {
			blockchain: from.blockchain,
			kind: transfer_kind,
			amount,
			block: Self::block_number(),
			from: from_id,
			to: to_id,
			order_id,
			is_processed: false,
			account_id: who,
			tx_id: blockchain_tx_id,
			timestamp: None,
		};

		let deadline = T::TaskScheduler::deadline();

		let pending = UnverifiedTransferFor::<T> {
			from_external: from.value,
			to_external: to.value,
			transfer: transfer.clone(),
			deadline,
		};

		let inner_id = TaskV2::<T>::to_id(&pending);

		Self::check_and_submit_transfer_as_task(&inner_id, pending)?;

		Ok((inner_id.into(), transfer))
	}

	#[inline]
	fn check_and_submit_transfer_as_task(
		task_id: &T::Hash,
		pending_transfer: UnverifiedTransferFor<T>,
	) -> Result<(), crate::Error<T>> {
		let deadline = pending_transfer.deadline;

		ensure!(
			!<UnverifiedTransfer<_, _, _, _> as TaskV2::<T>>::is_persisted(task_id),
			Error::<T>::TransferAlreadyRegistered
		);
		ensure!(
			!T::TaskScheduler::is_scheduled(&deadline, task_id),
			Error::<T>::TransferAlreadyRegistered
		);
		let pending_transfer = Task::from(pending_transfer);
		T::TaskScheduler::insert(&deadline, task_id, pending_transfer);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::helpers::extensions::IntoBounded;
	use crate::mock::{ExtBuilder, Test};
	use crate::pallet::Pallet as Creditcoin;
	use crate::tests::{RegisteredAddress, TestInfo};
	use crate::Blockchain;
	use frame_support::BoundedVec;

	#[test]
	fn register_transfer_internal_should_error_with_non_existent_lender_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";
			let bogus_address =
				AddressId::new::<Test>(&Blockchain::Rinkeby, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				bogus_address,
				deal_order.borrower_address_id,
				TransferKind::Native,
				deal_order.terms.amount,
				OrderId::Deal(deal_order_id),
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NonExistentAddress);
		})
	}

	#[test]
	fn register_transfer_internal_should_error_when_signer_doesnt_own_from_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.borrower_address_id, // should match 1st argument
				deal_order.lender_address_id,
				TransferKind::Native,
				deal_order.terms.amount,
				OrderId::Deal(deal_order_id),
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NotAddressOwner);
		})
	}
	#[test]
	fn register_transfer_internal_should_error_when_addresses_are_not_on_the_same_blockchain() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let second_borrower = RegisteredAddress::new("borrower2", Blockchain::Luniverse);
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				second_borrower.address_id,
				TransferKind::Native,
				deal_order.terms.amount,
				deal_order_id.into(),
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::AddressBlockchainMismatch);
		})
	}

	#[test]
	fn register_transfer_internal_should_error_when_transfer_kind_is_not_supported() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				deal_order.borrower_address_id,
				// not supported on Blockchain::RINKEBY
				TransferKind::Other(BoundedVec::try_from(b"other".to_vec()).unwrap()),
				deal_order.terms.amount,
				deal_order_id.into(),
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::UnsupportedTransferKind);
		})
	}

	#[test]
	fn register_transfer_internal_should_error_when_transfer_is_already_registered() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let (_transfer_id, transfer) = test_info.create_funding_transfer(&deal_order_id);

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				deal_order.borrower_address_id,
				TransferKind::Ethless(b"foo".into_bounded()),
				deal_order.terms.amount,
				deal_order_id.into(),
				transfer.tx_id,
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::TransferAlreadyRegistered);
		})
	}
}
