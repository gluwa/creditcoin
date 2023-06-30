mod external_address;
mod register_transfer;

use ethers_core::types::RecoveryMessage;
pub use external_address::{address_is_well_formed, generate_external_address};
#[cfg(any(test, feature = "runtime-benchmarks"))]
pub use external_address::{EVMAddress, PublicToAddress};
use sha3::Keccak256;
use sp_core::U256;
use sp_runtime::BoundedVec;

use crate::{
	pallet::*,
	types::{Address, AddressId, SignatureType},
	Blockchain, DealOrderId, Error, ExternalAddress, Guid, Id, TransferId,
};
use ethers_core::types::Signature;
use frame_support::ensure;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

#[allow(unused_macros)]
macro_rules! try_get {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		crate::pallet::$storage::<$t>::try_get($key).map_err(|()| crate::pallet::Error::<$t>::$err)
	};
}

#[macro_export]
macro_rules! try_get_id {
	($storage: ident <$t: ident>, $key: expr, $err: ident) => {
		<$crate::pallet::$storage<$t> as DoubleMapExt<_, _, _, _, _, _, _, _, _, _>>::try_get_id(
			$key,
		)
		.map_err(|()| $crate::pallet::Error::<$t>::$err)
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
		Self::addresses(address_id).ok_or(Error::<T>::NonExistentAddress)
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

pub fn non_paying_error<T: Config>(
	error: crate::Error<T>,
) -> frame_support::dispatch::DispatchErrorWithPostInfo {
	frame_support::dispatch::DispatchErrorWithPostInfo {
		error: error.into(),
		post_info: frame_support::dispatch::PostDispatchInfo {
			actual_weight: None,
			pays_fee: frame_support::dispatch::Pays::No,
		},
	}
}

pub mod extensions {

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	#[extend::ext(name = HexToAddress)]
	pub(crate) impl<'a> &'a str {
		fn hex_to_address(self) -> crate::ExternalAddress {
			use sp_std::convert::TryInto;
			hex::decode(self.trim_start_matches("0x")).unwrap().try_into().unwrap()
		}
		fn into_bounded<S>(self) -> frame_support::BoundedVec<u8, S>
		where
			S: frame_support::pallet_prelude::Get<u32>,
		{
			self.as_bytes().into_bounded()
		}
	}

	#[cfg(any(test, feature = "runtime-benchmarks"))]
	#[extend::ext(name = IntoBounded)]
	pub(crate) impl<'a, S, T> &'a [T]
	where
		S: frame_support::pallet_prelude::Get<u32>,
		T: Clone + alloc::fmt::Debug,
	{
		fn try_into_bounded(self) -> Result<frame_support::BoundedVec<T, S>, crate::Vec<T>> {
			core::convert::TryFrom::try_from(self.to_vec())
		}

		fn into_bounded(self) -> frame_support::BoundedVec<T, S> {
			self.try_into_bounded().unwrap()
		}
	}
}

pub fn try_extract_address<T: Config>(
	signature_type: SignatureType,
	signature: [u8; 65],
	account_id: String,
	blockchain: Blockchain,
	address: ExternalAddress,
) -> Result<ExternalAddress, crate::Error<T>> {
	match signature_type {
		// Old Way
		SignatureType::EthSign => {
			ExtractEthSignPublicKey(signature, account_id, blockchain, address)
		},
		// New Way
		SignatureType::PersonalSign => ExtractPeronalSignPublicKey(signature, account_id),
	}
}

use sp_io::crypto::secp256k1_ecdsa_recover_compressed;

fn ExtractEthSignPublicKey<T: Config>(
	signature: [u8; 65],
	account_id: String,
	blockchain: Blockchain,
	address: ExternalAddress,
) -> Result<ExternalAddress, crate::Error<T>> {
	let message = sp_io::hashing::sha2_256(account_id.as_bytes());
	let message = &sp_io::hashing::blake2_256(message.as_ref());

	match secp256k1_ecdsa_recover_compressed(&signature, &message) {
		Ok(public_key) => {
			match generate_external_address(
				&blockchain,
				&address,
				sp_core::ecdsa::Public::from_raw(public_key),
			) {
				Some(s) => Ok(s),
				None => Err(crate::Error::EthSignExternalAddressGenerationFailed),
			}
		},
		Err(e) => Err(crate::Error::EthSignInvalidSignature),
	}
}

pub fn eth_message(message: String) -> [u8; 32] {
	let mut hasher = Keccak256::new();

	hasher.update(
		format!("{}{}{}", "\x19Ethereum Signed Message:\n", message.len(), message.to_string())
			.as_bytes(),
	);

	return hasher.finalize();
}

pub fn ExtractPeronalSignPublicKey<T: Config>(
	signature: [u8; 65],
	message: String,
) -> Result<ExternalAddress, crate::Error<T>> {
	// Build the signature object
	let v = signature[64];
	let r = U256::from_big_endian(&signature[0..32]);
	let s = U256::from_big_endian(&signature[32..64]);
	let signature = Signature { r, s, v: v.into() };

	// Create the message and format it properly
	let message = eth_message(message);
	let recovery_message = RecoveryMessage::Data(message.to_vec());

	if let Ok(address) = signature.recover(message) {
		let address = address.as_bytes();
		let address = address.to_vec();

		let res: ExternalAddress =
			BoundedVec::try_from(address).expect("20 bytes fits within bounds.");

		return Ok(res);
	}

	return Err(crate::Error::PerosnalSignFailedRecovery);
}
