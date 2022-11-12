pub(crate) mod nonce;

use super::Error;
use super::{crypto, log, Authorities, Config, Pallet};
use alloc::vec;
use frame_support::dispatch::Vec;
use frame_support::traits::IsType;
use frame_system::offchain::{Account, SendSignedTransaction, Signer};
use frame_system::{Config as SystemConfig, Pallet as System};
use nonce::lock_key;
pub use nonce::nonce_key;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::traits::One;
use sp_runtime::traits::Saturating;
use sp_runtime::RuntimeAppPublic;

impl<T: Config> Pallet<T> {
	pub fn authority_id() -> Option<T::AccountIdFrom> {
		let local_keys = crypto::Public::all()
			.into_iter()
			.map(|p| sp_core::sr25519::Public::from(p).into())
			.collect::<Vec<T::AccountIdFrom>>();

		log::trace!(target: "task", "local keys {local_keys:?}");

		Authorities::<T>::iter_keys().find_map(|auth| {
			let acct = auth.clone().into();
			local_keys.contains(&acct).then(|| T::AccountIdFrom::from(auth))
		})
	}

	pub fn offchain_signed_tx(
		auth_id: T::AccountIdFrom,
		call: impl Fn(&Account<T>) -> T::TaskCall,
	) -> Result<(), Error<T>> {
		use sp_core::crypto::UncheckedFrom;
		let auth_bytes: &[u8; 32] = auth_id.as_ref();
		let public: T::PublicSigning = T::InternalPublic::unchecked_from(*auth_bytes).into();
		let signer = Signer::<T, T::AuthorityId>::any_account().with_filter(vec![public.into()]);
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
		auth_id: T::AccountIdFrom,
		call: impl Fn(&Account<T>) -> T::TaskCall,
	) -> Result<(), Error<T>> {
		let acc_id: &<T as SystemConfig>::AccountId = auth_id.into_ref();
		let mut account_data = System::<T>::account(acc_id);

		let key = &lock_key(auth_id.into_ref());
		let mut lock = Pallet::<T>::nonce_lock_new(key);
		let _guard = lock.lock();

		let key = &nonce_key(auth_id.into_ref());
		let synced_nonce_storage = StorageValueRef::persistent(key);
		let synced_nonce = synced_nonce_storage.get::<T::Index>().ok().flatten();

		let n = System::<T>::block_number();
		log::trace!(target: "task", "@{n:?} Offnonce {synced_nonce:?} Onnonce {:?}", account_data.nonce);

		if let Some(nonce) = synced_nonce {
			if nonce > account_data.nonce {
				account_data.nonce = nonce;
				frame_system::Account::<T>::insert(acc_id, account_data.clone());
			}
		}

		Pallet::<T>::offchain_signed_tx(auth_id, call)
			.map(|_| synced_nonce_storage.set(&account_data.nonce.saturating_add(One::one())))
	}
}

pub(crate) mod tests;
