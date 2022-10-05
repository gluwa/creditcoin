pub(crate) mod nonce;

use super::Error;
use super::{log, Authorities, Config, Pallet};
use alloc::vec;
use frame_support::dispatch::Vec;
use frame_system::offchain::AppCrypto;
use frame_system::offchain::{Account, SendSignedTransaction, Signer};
use frame_system::Pallet as System;
use nonce::lock_key;
pub use nonce::nonce_key;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::One;
use sp_runtime::traits::Saturating;
use sp_runtime::RuntimeAppPublic;

impl<T: Config> Pallet<T> {
	pub fn authority_pubkey() -> Option<T::Public>
	where
		<T::AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic: Into<T::Public>,
	{
		let local_keys =
			<T::AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic::all()
				.into_iter()
				.map(|p| {
					let pkey = p.into();
					(pkey.clone(), pkey.into_account())
				})
				.collect::<Vec<(T::Public, T::AccountId)>>();

		log::trace!(target: "task", "local keys {local_keys:?}");

		Authorities::<T>::iter_keys().find_map(|auth| {
			local_keys
				.iter()
				.find_map(|(pkey, acc)| if auth == *acc { Some(pkey.clone()) } else { None })
		})
	}

	pub fn offchain_signed_tx(
		auth_pubkey: T::Public,
		call: impl Fn(&Account<T>) -> T::TaskCall,
	) -> Result<(), Error<T>> {
		let signer = Signer::<T, T::AuthorityId>::any_account().with_filter(vec![auth_pubkey]);
		let result = signer.send_signed_transaction(call);

		if let Some((acc, res)) = result {
			if res.is_err() {
				log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(Error::OffchainSignedTxFailed);
			} else {
				return Ok(());
			}
		}

		log::error!("No local account available");
		Err(Error::NoLocalAcctForSignedTx)
	}

	pub fn submit_txn_with_synced_nonce(
		pubkey: T::Public,
		call: impl Fn(&Account<T>) -> T::TaskCall,
	) -> Result<(), Error<T>> {
		let auth_id: &T::AccountId = &pubkey.clone().into_account();
		let mut account_data = System::<T>::account(auth_id);

		let key = &lock_key(auth_id);
		let mut lock = Pallet::<T>::nonce_lock_new(key);
		let _guard = lock.lock();

		let key = &nonce_key(auth_id);
		let synced_nonce_storage = StorageValueRef::persistent(key);
		let synced_nonce = synced_nonce_storage.get::<T::Index>().ok().flatten();

		let n = System::<T>::block_number();
		log::trace!(target: "task", "@{n:?} Offnonce {synced_nonce:?} Onnonce {:?}", account_data.nonce);

		if let Some(nonce) = synced_nonce {
			if nonce > account_data.nonce {
				account_data.nonce = nonce;
				frame_system::Account::<T>::insert(auth_id, account_data.clone());
			}
		}

		Pallet::<T>::offchain_signed_tx(pubkey, call)
			.map(|_| synced_nonce_storage.set(&account_data.nonce.saturating_add(One::one())))
	}
}

pub(crate) mod tests;
