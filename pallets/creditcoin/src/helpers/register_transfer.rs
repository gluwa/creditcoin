use crate::{
	pallet::*, types::AddressId, Blockchain, Currency, CurrencyId, DealOrderId, Error,
	EvmCurrencyType, EvmSupportedTransferKinds, EvmTransferKind, ExternalAmount, ExternalTxId, Id,
	LegacyTransferKind, Task, Transfer, TransferId, TransferKind, UnverifiedTransfer,
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
		deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
		blockchain_tx_id: ExternalTxId,
		currency: &CurrencyId<T::Hash>,
	) -> Result<(TransferFor<T>, UnverifiedTransferFor<T>), crate::Error<T>> {
		let from = Self::get_address(&from_id)?;
		let to = Self::get_address(&to_id)?;

		ensure!(from.owner == who, Error::<T>::NotAddressOwner);

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressBlockchainMismatch);

		let currency = Currencies::<T>::get(&currency).ok_or(Error::<T>::CurrencyNotRegistered)?;

		ensure!(currency.supports(&transfer_kind), Error::<T>::UnsupportedTransferKind);

		let block = Self::block_number();

		let transfer = Transfer {
			blockchain: from.blockchain,
			kind: transfer_kind,
			amount,
			block,
			from: from_id,
			to: to_id,
			deal_order_id,
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
				currency_to_check: crate::CurrencyOrLegacyTransferKind::Currency(currency),
			},
		))
	}

	pub fn register_transfer_internal(
		who: T::AccountId,
		from_id: AddressId<T::Hash>,
		to_id: AddressId<T::Hash>,
		transfer_kind: TransferKind,
		amount: ExternalAmount,
		deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
		blockchain_tx_id: ExternalTxId,
		currency: &CurrencyId<T::Hash>,
	) -> Result<
		(TransferId<T::Hash>, Transfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>),
		crate::Error<T>,
	> {
		let (transfer, pending) = Self::generate_transfer(
			who,
			from_id,
			to_id,
			transfer_kind,
			amount,
			deal_order_id,
			blockchain_tx_id,
			currency,
		)?;

		let inner_id = TaskV2::<T>::to_id(&pending);

		Self::check_and_submit_transfer_as_task(&inner_id, pending)?;

		Ok((inner_id.into(), transfer))
	}

	pub fn register_transfer_internal_legacy(
		who: T::AccountId,
		from_id: AddressId<T::Hash>,
		to_id: AddressId<T::Hash>,
		transfer_kind: LegacyTransferKind,
		amount: ExternalAmount,
		deal_order_id: DealOrderId<T::BlockNumber, T::Hash>,
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

		DealOrders::<T>::mutate(
			deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order| -> Result<(), crate::Error<T>> {
				let mut deal_order = deal_order.as_mut().ok_or(Error::<T>::NonExistentDealOrder)?;
				let currency = match &from.blockchain {
					Blockchain::Evm(info) => match transfer_kind.clone() {
						LegacyTransferKind::Ethless(contract) => Currency::Evm(
							EvmCurrencyType::SmartContract(
								contract,
								EvmSupportedTransferKinds::try_from(vec![EvmTransferKind::Ethless])
									.expect("length 1 is less than the bound 2; qed"),
							),
							info.clone(),
						),
						LegacyTransferKind::Erc20(contract) => Currency::Evm(
							EvmCurrencyType::SmartContract(
								contract,
								EvmSupportedTransferKinds::try_from(vec![EvmTransferKind::Erc20])
									.expect("length 1 is less than the bound 2; qed"),
							),
							info.clone(),
						),
						_ => return Err(Error::<T>::UnsupportedTransferKind),
					},
				};
				let currency_id = CurrencyId::new::<T>(&currency);
				deal_order.terms.currency = currency_id;
				Ok(())
			},
		)?;

		let transfer = Transfer {
			blockchain: from.blockchain,
			kind: transfer_kind
				.clone()
				.try_into()
				.map_err(|()| Error::<T>::UnsupportedTransferKind)?,
			amount,
			block: Self::block_number(),
			from: from_id,
			to: to_id,
			deal_order_id,
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
			currency_to_check: crate::CurrencyOrLegacyTransferKind::TransferKind(transfer_kind),
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
	use frame_support::BoundedVec;

	#[test]
	fn register_transfer_internal_legacy_should_error_with_non_existent_lender_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";
			let bogus_address =
				AddressId::new::<Test>(&Blockchain::RINKEBY, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				bogus_address,
				deal_order.borrower_address_id,
				LegacyTransferKind::Native,
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NonExistentAddress);
		})
	}

	#[test]
	fn register_transfer_internal_legacy_should_error_with_non_existent_borrower_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";
			let bogus_address =
				AddressId::new::<Test>(&Blockchain::RINKEBY, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				bogus_address,
				LegacyTransferKind::Native,
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NonExistentAddress);
		})
	}

	#[test]
	fn register_transfer_internal_legacy_should_error_when_signer_doesnt_own_from_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				deal_order.borrower_address_id, // should match 1st argument
				deal_order.lender_address_id,
				LegacyTransferKind::Native,
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NotAddressOwner);
		})
	}

	#[test]
	fn register_transfer_internal_legacy_should_error_when_addresses_are_not_on_the_same_blockchain(
	) {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let second_borrower = RegisteredAddress::new("borrower2", Blockchain::LUNIVERSE);
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				second_borrower.address_id,
				LegacyTransferKind::Native,
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::AddressBlockchainMismatch);
		})
	}

	#[test]
	fn register_transfer_internal_legacy_should_error_when_transfer_kind_is_not_supported() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				deal_order.borrower_address_id,
				// not supported on Blockchain::RINKEBY
				LegacyTransferKind::Other(BoundedVec::try_from(b"other".to_vec()).unwrap()),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::UnsupportedTransferKind);
		})
	}

	#[test]
	fn register_transfer_internal_legacy_should_error_when_transfer_is_already_registered() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let (_transfer_id, transfer) = test_info.create_funding_transfer(&deal_order_id);

			let result = Creditcoin::register_transfer_internal_legacy(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				deal_order.borrower_address_id,
				LegacyTransferKind::Ethless(b"foo".into_bounded()),
				deal_order.terms.amount,
				deal_order_id,
				transfer.tx_id,
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::TransferAlreadyRegistered);
		})
	}

	#[test]
	fn register_transfer_internal_should_error_with_non_existent_lender_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";
			let bogus_address =
				AddressId::new::<Test>(&Blockchain::RINKEBY, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				bogus_address,
				deal_order.borrower_address_id,
				EvmTransferKind::Ethless.into(),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
				&test_info.currency.to_id::<Test>(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::NonExistentAddress);
		})
	}

	#[test]
	fn register_transfer_internal_should_error_with_non_existent_borrower_address() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let (deal_order_id, deal_order) = test_info.create_deal_order();
			let tx = "0xabcabcabc";
			let bogus_address =
				AddressId::new::<Test>(&Blockchain::RINKEBY, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				bogus_address,
				EvmTransferKind::Ethless.into(),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
				&test_info.currency.to_id::<Test>(),
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
				EvmTransferKind::Ethless.into(),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
				&test_info.currency.to_id::<Test>(),
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
			let second_borrower = RegisteredAddress::new("borrower2", Blockchain::LUNIVERSE);
			let tx = "0xabcabcabc";

			let result = Creditcoin::register_transfer_internal(
				test_info.lender.account_id,
				deal_order.lender_address_id,
				second_borrower.address_id,
				EvmTransferKind::Ethless.into(),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
				&test_info.currency.to_id::<Test>(),
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
				EvmTransferKind::Erc20.into(),
				deal_order.terms.amount,
				deal_order_id,
				tx.as_bytes().into_bounded(),
				&test_info.currency.to_id::<Test>(),
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
				EvmTransferKind::Ethless.into(),
				deal_order.terms.amount,
				deal_order_id,
				transfer.tx_id,
				&test_info.currency.to_id::<Test>(),
			)
			.unwrap_err();

			assert_eq!(result, crate::Error::<Test>::TransferAlreadyRegistered);
		})
	}
}
