pub mod caching;
pub mod disputing;
pub(crate) mod nonce;
pub mod sampling;
pub mod voting;

use super::authorship::Authorship;
use super::Error;
use super::{log, Config, Pallet};
use crate::tasks::LockGuard;
use crate::Call;
use alloc::vec;
use caching::OutputCache;
use frame_support::dispatch::Vec;
use frame_system::offchain::{Account, SendSignedTransaction, Signer};
use frame_system::offchain::{AppCrypto, SigningTypes};
use frame_system::Pallet as System;
use nonce::lock_key;
pub use nonce::nonce_key;
use sampling::Sampling;
use sp_core::sr25519::Public;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::One;
use sp_runtime::traits::Saturating;
use sp_runtime::RuntimeAppPublic;
use sp_std::boxed::Box;
use sp_std::fmt::Debug;

type SamplingProofOf<T> = <<T as Config>::Sampling as Sampling>::Proof;
pub type RuntimePublicOf<T> = <<T as Config>::AuthorityId as AppCrypto<
	<T as SigningTypes>::Public,
	<T as SigningTypes>::Signature,
>>::RuntimeAppPublic;

impl<T: Config> Pallet<T>
where
	RuntimePublicOf<T>: Into<T::Public> + AsRef<Public> + Debug + Clone,
{
	pub(crate) fn try_submit(
		block_number: T::BlockNumber,
		deadline: T::BlockNumber,
		task_id: T::Hash,
		signer: RuntimePublicOf<T>,
		call: Box<T::TaskCall>,
		guard: LockGuard<'_, '_, System<T>>,
		proof: SamplingProofOf<T>,
	) {
		match Self::submit_txn_with_synced_nonce(signer.into(), |_| {
			Call::<T>::submit_output { deadline, task_id, call: call.clone(), proof: proof.clone() }
				.into()
		}) {
			Ok(_) => {
				guard.forget();
			},
			// release the lock and try again later.
			Err(e) => {
				log::error!(
					target: "runtime::task", "@{block_number:?} Failed to send a dispatchable transaction: {e:?}",
				);
				T::OutputCache::clear(&task_id);
			},
		}
	}
}

// the method is not idempotent, there is no guarantee that you will get the same key if multiple exist.
impl<T: Config> Pallet<T> {
	pub fn authority_pubkey() -> Option<RuntimePublicOf<T>>
	where
		RuntimePublicOf<T>: sp_std::fmt::Debug,
	{
		let local_keys: Vec<RuntimePublicOf<T>> =
			<T::AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic::all()
				.into_iter()
				.collect();

		log::trace!(target: "runtime::task ", "local keys {local_keys:?}");
		T::Authorship::find_authorized(local_keys.iter())
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
		log::trace!(target: "runtime::task", "@{n:?} Offnonce {synced_nonce:?} Onnonce {:?}", account_data.nonce);

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
