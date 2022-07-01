pub(crate) mod errors;
mod nonce;
pub(crate) mod rpc;
pub(crate) mod tasks;

use self::{errors::RpcUrlError, rpc::errors::RpcError};
use super::{
	pallet::{Config, Error, Pallet},
	ExternalAddress,
};
use crate::{Blockchain, Call, ExternalTxId, LegacyTransferKind};
use alloc::string::String;
pub(crate) use errors::{OffchainError, VerificationFailureCause, VerificationResult};
use frame_support::traits::IsType;
use frame_system::offchain::{Account, SendSignedTransaction, Signer};
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
use nonce::lock_key;
pub use nonce::nonce_key;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::traits::{One, Saturating};
use sp_std::prelude::*;

pub(crate) type OffchainResult<T, E = errors::OffchainError> = Result<T, E>;

impl Blockchain {
	pub fn rpc_url(&self) -> OffchainResult<String, errors::RpcUrlError> {
		let chain_prefix = self.as_bytes();
		let mut buf = Vec::from(chain_prefix);
		buf.extend("-rpc-uri".bytes());
		let rpc_url_storage = StorageValueRef::persistent(&buf);
		if let Some(url_bytes) = rpc_url_storage.get::<Vec<u8>>()? {
			Ok(String::from_utf8(url_bytes)?)
		} else {
			Err(RpcUrlError::NoValue)
		}
	}
	pub fn supports(&self, kind: &LegacyTransferKind) -> bool {
		match (self, kind) {
			(
				Blockchain::Ethereum | Blockchain::Luniverse | Blockchain::Rinkeby,
				LegacyTransferKind::Erc20(_)
				| LegacyTransferKind::Ethless(_)
				| LegacyTransferKind::Native,
			) => true,
			(Blockchain::Bitcoin, LegacyTransferKind::Native) => true,
			(_, _) => false, // TODO: refine this later
		}
	}
}

const ETH_CONFIRMATIONS: u64 = 12;

fn parse_eth_address(address: &ExternalAddress) -> OffchainResult<rpc::Address> {
	let address_bytes = <[u8; 20]>::try_from(address.as_slice())
		.map_err(|_| VerificationFailureCause::InvalidAddress)?;
	let address = rpc::Address::from(address_bytes);
	Ok(address)
}

fn eth_get_transaction(tx_id: &ExternalTxId, rpc_url: &str) -> OffchainResult<rpc::EthTransaction> {
	rpc::eth_get_transaction(tx_id, rpc_url).map_err(|e| {
		if let RpcError::NoResult = e {
			OffchainError::InvalidTask(VerificationFailureCause::TransactionNotFound)
		} else {
			e.into()
		}
	})
}

impl<T: Config> Pallet<T> {
	fn offchain_signed_tx(
		auth_id: T::FromAccountId,
		call: impl Fn(&Account<T>) -> Call<T>,
	) -> Result<(), Error<T>> {
		use sp_core::crypto::UncheckedFrom;
		let auth_bytes: &[u8; 32] = auth_id.as_ref();
		let public: T::PublicSigning = T::InternalPublic::unchecked_from(*auth_bytes).into();
		let signer =
			Signer::<T, T::AuthorityId>::any_account().with_filter(sp_std::vec![public.into()]);
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
		auth_id: T::FromAccountId,
		call: impl Fn(&Account<T>) -> Call<T>,
	) -> Result<(), Error<T>> {
		let acc_id: &<T as SystemConfig>::AccountId = auth_id.into_ref();
		let mut account_data = System::<T>::account(acc_id);

		let key = &lock_key(auth_id.into_ref());
		let mut lock = Pallet::<T>::nonce_lock_new(key);
		let _guard = lock.lock();

		let key = &nonce_key(auth_id.into_ref());
		let synced_nonce_storage = StorageValueRef::persistent(key);
		let synced_nonce = synced_nonce_storage.get::<T::Index>().ok().flatten();

		let n = Self::block_number();
		tracing::trace!(target: "OCW", "@{n:?} Offnonce {synced_nonce:?} Onnonce {:?}", account_data.nonce);

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

#[cfg(test)]
mod tests;
