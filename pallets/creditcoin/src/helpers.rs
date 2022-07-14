mod external_address;

pub use external_address::{address_is_well_formed, generate_external_address};
#[cfg(any(test, feature = "runtime-benchmarks"))]
pub use external_address::{EVMAddress, PublicToAddress};

use crate::{
	pallet::*,
	types::{Address, AddressId},
	Blockchain, Currency, CurrencyId, DealOrderId, Error, EvmCurrencyType,
	EvmSupportedTransferKinds, EvmTransferKind, ExternalAmount, ExternalTxId, Guid, Id,
	LegacyTransferKind, Task, TaskId, Transfer, TransferId, TransferKind, UnverifiedTransfer,
};
use frame_support::{ensure, traits::Get};
use frame_system::pallet_prelude::*;
use sp_runtime::{traits::Saturating, RuntimeAppPublic};
use sp_std::prelude::*;
use tracing as log;

#[allow(unused_macros)]
macro_rules! try_get {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		crate::pallet::$storage::<$t>::try_get($key).map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

macro_rules! try_get_id {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		<crate::pallet::$storage<$t> as DoubleMapExt<_, _, _, _, _, _, _, _, _, _>>::try_get_id(
			$key,
		)
		.map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

type DealOrderFor<T> = crate::DealOrder<
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::BlockNumber,
	<T as frame_system::Config>::Hash,
	<T as pallet_timestamp::Config>::Moment,
>;
type TransferFor<T> = crate::Transfer<
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::BlockNumber,
	<T as frame_system::Config>::Hash,
	<T as pallet_timestamp::Config>::Moment,
>;

impl<T: Config> Pallet<T> {
	pub fn block_number() -> BlockNumberFor<T> {
		<frame_system::Pallet<T>>::block_number()
	}
	pub fn timestamp() -> T::Moment {
		<pallet_timestamp::Pallet<T>>::get()
	}
	pub fn get_address(address_id: &AddressId<T::Hash>) -> Result<Address<T::AccountId>, Error<T>> {
		Self::addresses(&address_id).ok_or(Error::<T>::NonExistentAddress)
	}

	pub fn authority_id() -> Option<T::AccountId> {
		let local_keys = crate::crypto::Public::all()
			.into_iter()
			.map(|p| sp_core::sr25519::Public::from(p).into())
			.collect::<Vec<T::FromAccountId>>();

		log::trace!(target: "OCW", "local keys {local_keys:?}");

		Authorities::<T>::iter_keys().find_map(|auth| {
			let acct = auth.clone().into();
			local_keys.contains(&acct).then_some(auth)
		})
	}

	pub fn try_mutate_deal_order_and_transfer(
		deal_order_id: &DealOrderId<T::BlockNumber, T::Hash>,
		transfer_id: &TransferId<T::Hash>,
		mutate_deal: impl FnOnce(
			&mut DealOrderFor<T>,
		) -> Result<Option<crate::Event<T>>, crate::Error<T>>,
		mutate_transfer: impl FnOnce(
			&mut TransferFor<T>,
			&DealOrderFor<T>,
		) -> Result<Option<crate::Event<T>>, crate::Error<T>>,
	) -> Result<(), crate::Error<T>> {
		let result = DealOrders::<T>::try_mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|value| {
				let deal_order = value.as_mut().ok_or(crate::Error::<T>::NonExistentDealOrder)?;
				let deal_event = mutate_deal(deal_order)?;

				let transfer_event = Transfers::<T>::try_mutate(transfer_id, |value| {
					let transfer = value.as_mut().ok_or(crate::Error::<T>::NonExistentTransfer)?;
					mutate_transfer(transfer, deal_order)
				})?;

				Ok((deal_event, transfer_event))
			},
		);

		match result {
			Ok((deal_event, transfer_event)) => {
				if let Some(event) = deal_event {
					Self::deposit_event(event);
				}
				if let Some(event) = transfer_event {
					Self::deposit_event(event)
				}

				Ok(())
			},
			Err(e) => Err(e),
		}
	}

	pub fn use_guid(guid: &Guid) -> Result<(), Error<T>> {
		ensure!(!<UsedGuids<T>>::contains_key(guid.clone()), Error::<T>::GuidAlreadyUsed);
		UsedGuids::<T>::insert(guid, ());
		Ok(())
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

pub fn non_paying_error<T: Config>(
	error: crate::Error<T>,
) -> frame_support::dispatch::DispatchErrorWithPostInfo {
	frame_support::dispatch::DispatchErrorWithPostInfo {
		error: error.into(),
		post_info: frame_support::dispatch::PostDispatchInfo {
			actual_weight: None,
			pays_fee: frame_support::weights::Pays::No,
		},
	}
}

#[cfg(any(test, feature = "runtime-benchmarks"))]
#[extend::ext]
pub(crate) impl<'a> &'a str {
	fn hex_to_address(self) -> crate::ExternalAddress {
		hex::decode(self.trim_start_matches("0x")).unwrap().try_into().unwrap()
	}
}

#[cfg(any(test, feature = "runtime-benchmarks"))]
#[extend::ext]
pub(crate) impl<'a, S, T> &'a [T]
where
	S: Get<u32>,
	T: Clone,
{
	fn try_into_bounded(self) -> Result<frame_support::BoundedVec<T, S>, ()> {
		core::convert::TryFrom::try_from(self.to_vec())
	}
	fn into_bounded(self) -> frame_support::BoundedVec<T, S> {
		core::convert::TryFrom::try_from(self.to_vec()).unwrap()
	}
}
