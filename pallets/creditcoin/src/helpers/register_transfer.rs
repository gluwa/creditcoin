use crate::{
	pallet::*, types::AddressId, Blockchain, Currency, CurrencyId, DealOrderId, Error,
	EvmCurrencyType, EvmSupportedTransferKinds, EvmTransferKind, ExternalAmount, ExternalTxId, Id,
	LegacyTransferKind, Task, TaskId, Transfer, TransferId, TransferKind, UnverifiedTransfer,
};

use frame_support::{ensure, traits::Get};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

impl<T: Config> Pallet<T> {
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
		let from = Self::get_address(&from_id)?;
		let to = Self::get_address(&to_id)?;

		ensure!(from.owner == who, Error::<T>::NotAddressOwner);

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressBlockchainMismatch);

		let transfer_id = TransferId::new::<T>(&from.blockchain, &blockchain_tx_id);
		ensure!(!Transfers::<T>::contains_key(&transfer_id), Error::<T>::TransferAlreadyRegistered);

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

		let deadline = block.saturating_add(T::UnverifiedTaskTimeout::get());

		let pending = UnverifiedTransfer {
			from_external: from.value,
			to_external: to.value,
			transfer: transfer.clone(),
			deadline,
			currency_to_check: crate::CurrencyOrLegacyTransferKind::Currency(currency),
		};
		let task_id = TaskId::from(transfer_id.clone());
		let pending = Task::from(pending);
		PendingTasks::<T>::insert(&deadline, &task_id, &pending);

		Ok((transfer_id, transfer))
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

		let transfer_id = TransferId::new::<T>(&from.blockchain, &blockchain_tx_id);
		ensure!(!Transfers::<T>::contains_key(&transfer_id), Error::<T>::TransferAlreadyRegistered);

		let block = Self::block_number();

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
			block,
			from: from_id,
			to: to_id,
			deal_order_id,
			is_processed: false,
			account_id: who,
			tx_id: blockchain_tx_id,
			timestamp: None,
		};

		let deadline = block.saturating_add(T::UnverifiedTaskTimeout::get());

		let pending = UnverifiedTransfer {
			from_external: from.value,
			to_external: to.value,
			transfer: transfer.clone(),
			deadline,
			currency_to_check: crate::CurrencyOrLegacyTransferKind::TransferKind(transfer_kind),
		};
		let task_id = TaskId::from(transfer_id.clone());
		let pending = Task::from(pending);
		PendingTasks::<T>::insert(&deadline, &task_id, &pending);

		Ok((transfer_id, transfer))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{ExtBuilder, Test};
	use crate::pallet::Pallet as Creditcoin;
	use crate::tests::{IntoBounded, RegisteredAddress, TestInfo};
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
				LegacyTransferKind::Native,
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
