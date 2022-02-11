use crate::{
	pallet::*,
	types::{Address, AddressId},
	Guid,
};
use frame_system::pallet_prelude::*;
use sp_io::hashing::sha2_256;
use sp_runtime::{traits::UniqueSaturatedInto, RuntimeAppPublic};
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
	) -> [u8; 32] {
		let expiration_block_u64: u64 = expiration_block.unique_saturated_into();
		let mut buf = lexical::to_string(expiration_block_u64).into_bytes();
		let block_end_idx = buf.len();
		buf.extend(core::iter::repeat(0u8).take(2 * (ask_guid.len() + bid_guid.len())));
		hex::encode_to_slice(&*ask_guid, &mut buf[block_end_idx..])
			.expect("we allocated 2 * (length of guid) bytes, it must be enough capacity; qed");
		hex::encode_to_slice(&*bid_guid, &mut buf[(block_end_idx + 2 * ask_guid.len())..])
			.expect("we just allocated 2 * (length of guid) bytes; qed");
		sha2_256(&buf)
	}
}

pub mod interest_rate {
	use crate::ExternalAmount;

	pub const INTEREST_RATE_PRECISION: u64 = 10_000;

	pub fn calc_interest(
		principal_amount: &ExternalAmount,
		interest_rate_bps: &ExternalAmount,
	) -> ExternalAmount {
		principal_amount * interest_rate_bps / INTEREST_RATE_PRECISION
	}
}

#[cfg(test)]
mod tests {
	use super::interest_rate::calc_interest;
	use crate::ExternalAmount;
	use ethereum_types::U256;

	#[test]
	pub fn test_calc_interest() {
		let principal_amount = ExternalAmount::from(100_000);
		let interest_rate_bps = ExternalAmount::from(1_000);
		let interest = calc_interest(&principal_amount, &interest_rate_bps);
		assert_eq!(interest, U256::from(10_000));
	}
}
