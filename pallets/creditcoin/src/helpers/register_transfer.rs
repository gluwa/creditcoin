use crate::{
	AddressId, Config, Currency, CurrencyId, DealOrderId, Error, EvmCurrencyType,
	EvmSupportedTransferKinds, EvmTransferKind, ExternalAmount, ExternalTxId, LegacyTransferKind,
	Pallet, PendingTasks, Task, TaskId, Transfer, TransferId, TransferKind, UnverifiedTransfer,
};
use frame_support::ensure;
use frame_system::pallet_prelude::BlockNumberFor;

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

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressPlatformMismatch);

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

		ensure!(from.blockchain == to.blockchain, Error::<T>::AddressPlatformMismatch);

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
