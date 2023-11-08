mod external_address;
mod register_transfer;

pub use external_address::{address_is_well_formed, generate_external_address};
#[cfg(any(test, feature = "runtime-benchmarks"))]
pub use external_address::{EVMAddress, PublicToAddress};
use pallet_balances::PositiveImbalance;

use crate::{
	pallet::*,
	types::{Address, AddressId, OwnershipProof},
	Blockchain, DealOrderId, Error, ExternalAddress, Guid, Id, TransferId,
};
use frame_support::ensure;
use frame_support::traits::tokens::currency::Currency as CurrencyT;
use frame_support::traits::ExistenceRequirement::AllowDeath;
use frame_support::traits::WithdrawReasons;
use frame_system::pallet_prelude::*;
use sp_runtime::SaturatedConversion;
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

use sp_io::crypto::secp256k1_ecdsa_recover_compressed;

/// Try to extract an external address for a particular blockchain through a signature and an account id which acts as a message.
/// This function supports the older and insecure EthSign signing method and the new PersonalSign standard that is supported by Metamask.
pub fn try_extract_address<T: Config>(
	ownership_proof: OwnershipProof,
	account_id: &[u8],
	blockchain: &Blockchain,
	address: &ExternalAddress,
) -> Result<ExternalAddress, crate::Error<T>> {
	match ownership_proof {
		// Old insecure signing method
		OwnershipProof::EthSign(signature) => {
			extract_public_key_eth_sign(signature.into(), account_id, blockchain, address)
		},
		// New Way
		OwnershipProof::PersonalSign(signature) => {
			extract_public_key_personal_sign(signature.into(), account_id, blockchain, address)
		},
	}
}

fn extract_public_key_eth_sign<T: Config>(
	signature: [u8; 65],
	account_id: &[u8],
	blockchain: &Blockchain,
	address: &ExternalAddress,
) -> Result<ExternalAddress, Error<T>> {
	let message = sp_io::hashing::sha2_256(account_id);
	let message = &sp_io::hashing::blake2_256(message.as_ref());

	match secp256k1_ecdsa_recover_compressed(&signature, message) {
		Ok(public_key) => {
			match generate_external_address(
				blockchain,
				address,
				sp_core::ecdsa::Public::from_raw(public_key),
			) {
				Some(s) => Ok(s),
				None => Err(Error::EthSignExternalAddressGenerationFailed),
			}
		},
		Err(_) => Err(Error::InvalidSignature),
	}
}

pub fn eth_message(message: &[u8; 32]) -> [u8; 32] {
	let mut bytes: Vec<u8> = vec![];
	let salt = b"\x19Ethereum Signed Message:\n32";

	bytes.extend_from_slice(salt);
	bytes.extend_from_slice(message);

	sp_io::hashing::keccak_256(&bytes)
}

pub fn extract_public_key_personal_sign<T: Config>(
	signature: [u8; 65],
	account_id: &[u8],
	blockchain: &Blockchain,
	address: &ExternalAddress,
) -> Result<ExternalAddress, Error<T>> {
	let message = sp_io::hashing::blake2_256(account_id);
	let message = eth_message(&message);

	match secp256k1_ecdsa_recover_compressed(&signature, &message) {
		Ok(public_key) => {
			match generate_external_address(
				blockchain,
				address,
				sp_core::ecdsa::Public::from_raw(public_key),
			) {
				Some(s) => Ok(s),
				None => Err(Error::PersonalSignExternalAddressGenerationFailed),
			}
		},
		Err(_) => Err(Error::InvalidSignature),
	}
}

#[test]
fn test_extract_public_key_personal_sign() {
	let expected_hash =
		hex::decode("cc2da28afbc18b601ee75cebaea68b70189c6eaae842c8971b31cd181dceda8c").unwrap();

	let raw_address: [u8; 32] = [
		136, 220, 52, 23, 213, 5, 142, 196, 180, 80, 62, 12, 18, 234, 26, 10, 137, 190, 32, 15,
		233, 137, 34, 66, 61, 67, 52, 1, 79, 166, 176, 238,
	];

	let message = eth_message(&raw_address);

	assert_eq!(message.as_slice(), expected_hash.as_slice());
}

pub fn blockchain_is_supported(blockchain: &Blockchain) -> bool {
	match blockchain {
		Blockchain::Luniverse | Blockchain::Ethereum | Blockchain::Rinkeby => true,
		Blockchain::Bitcoin => false,
		Blockchain::Other(_) => false,
	}
}

pub fn burn_and_settle<T: Config>(
	who: T::AccountId,
	amount: T::Balance,
) -> Result<(), PositiveImbalance<T>> {
	let imbalance: pallet_balances::PositiveImbalance<T> =
		<pallet_balances::Pallet<T>>::burn(amount);

	let settlement_result = <pallet_balances::Pallet<T> as CurrencyT<T::AccountId>>::settle(
		&who,
		imbalance,
		WithdrawReasons::TRANSFER,
		AllowDeath,
	);

	settlement_result
}

pub fn can_burn_amount<T: Config>(who: T::AccountId, amount: T::Balance) -> bool {
	let balance = <pallet_balances::Pallet<T> as CurrencyT<T::AccountId>>::free_balance(&who);

	let res = balance.saturated_into::<u128>();
	let amount_128 = amount.saturated_into::<u128>();

	res >= amount_128
}
