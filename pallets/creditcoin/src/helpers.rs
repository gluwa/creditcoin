use crate::{
	pallet::*,
	types::{Address, AddressId},
	DealOrderId, Error, Guid, Id, LoanTerms, TransferId,
};
use codec::Encode;
use frame_support::ensure;
use frame_system::pallet_prelude::*;
use sp_io::hashing::sha2_256;
use sp_runtime::RuntimeAppPublic;
use sp_std::prelude::*;

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

		log::trace!("{:?}", local_keys);

		Authorities::<T>::iter_keys().find_map(|auth| {
			let acct = auth.clone().into();
			local_keys.contains(&acct).then(|| auth)
		})
	}

	pub fn register_deal_order_message(
		expiration_block: T::BlockNumber,
		ask_guid: &Guid,
		bid_guid: &Guid,
		loan_terms: &LoanTerms<T::Moment>,
	) -> [u8; 32] {
		let all_encoded = expiration_block
			.encode()
			.into_iter()
			.chain(ask_guid.encode())
			.chain(bid_guid.encode())
			.chain(loan_terms.encode())
			.collect::<Vec<u8>>();

		sha2_256(&all_encoded)
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
}

#[cfg(test)]
mod tests {
	use crate::{ExternalAmount, LoanTerms};

	#[test]
	fn register_deal_order_message_works() {
		use core::convert::TryFrom;
		use frame_support::BoundedVec;
		let expiration_block = 5;
		let ask_guid = BoundedVec::try_from(b"asdfasdfasdfasdf".to_vec()).unwrap();
		let bid_guid = BoundedVec::try_from(b"qwerqwerqwerqwer".to_vec()).unwrap();

		let loan_terms =
			LoanTerms { amount: ExternalAmount::from(1u64), interest_rate: 10, maturity: 10 };

		// "expected" derived from creating the same message hash via PolkadotJs
		let expected: [u8; 32] = [
			46, 130, 146, 236, 109, 135, 57, 106, 137, 172, 43, 134, 74, 91, 53, 45, 152, 197, 25,
			65, 220, 98, 8, 250, 51, 3, 163, 238, 102, 83, 2, 123,
		];

		let msg = crate::Pallet::<crate::mock::Test>::register_deal_order_message(
			expiration_block,
			&ask_guid,
			&bid_guid,
			&loan_terms,
		);

		assert_eq!(msg, expected);
	}

	#[test]
	fn register_deal_order_message_empty_guids() {
		use core::convert::TryFrom;
		use frame_support::BoundedVec;
		let expiration_block = 5;
		let ask_guid = BoundedVec::try_from(vec![]).unwrap();
		let bid_guid = BoundedVec::try_from(vec![]).unwrap();

		let loan_terms =
			LoanTerms { amount: ExternalAmount::from(1u64), interest_rate: 10, maturity: 10 };

		// "expected" derived from creating the same message hash via PolkadotJs
		let expected: [u8; 32] = [
			163, 128, 181, 134, 42, 116, 134, 42, 137, 220, 103, 210, 192, 253, 121, 158, 168, 28,
			88, 83, 38, 163, 19, 127, 7, 150, 247, 10, 26, 128, 115, 219,
		];

		let msg = crate::Pallet::<crate::mock::Test>::register_deal_order_message(
			expiration_block,
			&ask_guid,
			&bid_guid,
			&loan_terms,
		);

		assert_eq!(msg, expected);
	}
}
