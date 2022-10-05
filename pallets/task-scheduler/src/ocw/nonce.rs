use crate::{Config, Pallet};
use alloc::vec::Vec;
use codec::Encode;
use sp_runtime::offchain::storage_lock::{StorageLock, Time};
use sp_runtime::offchain::Duration;

const SYNCED_NONCE: &[u8] = b"creditcoin/OCW/nonce/nonce/";
const SYNCED_NONCE_LOCK: &[u8] = b"creditcoin/OCW/nonce/lock/";
const LOCK_DEADLINE: u64 = 50_000;

pub(super) fn lock_key<Id: Encode>(id: &Id) -> Vec<u8> {
	id.using_encoded(|encoded_id| SYNCED_NONCE_LOCK.iter().chain(encoded_id).copied().collect())
}

pub fn nonce_key<Id: Encode>(id: &Id) -> Vec<u8> {
	id.using_encoded(|encoded_id| SYNCED_NONCE.iter().chain(encoded_id).copied().collect())
}

impl<T: Config> Pallet<T> {
	pub(super) fn nonce_lock_new(key: &[u8]) -> StorageLock<'_, Time> {
		StorageLock::<Time>::with_deadline(key, Duration::from_millis(LOCK_DEADLINE))
	}
}
