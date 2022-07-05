pub mod errors;
pub mod rpc;
pub mod tasks;

use crate::{Call, LegacyTransferKind, OldBlockchain};
pub use errors::{OffchainError, VerificationFailureCause, VerificationResult};

use self::errors::RpcUrlError;

use super::{
	pallet::{Config, Error, Pallet},
	ExternalAddress,
};
use alloc::string::String;
use frame_system::offchain::{Account, SendSignedTransaction, Signer};
use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::prelude::*;

pub type OffchainResult<T, E = errors::OffchainError> = Result<T, E>;

impl OldBlockchain {
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
				OldBlockchain::Ethereum | OldBlockchain::Luniverse | OldBlockchain::Rinkeby,
				LegacyTransferKind::Erc20(_)
				| LegacyTransferKind::Ethless(_)
				| LegacyTransferKind::Native,
			) => true,
			(OldBlockchain::Bitcoin, LegacyTransferKind::Native) => true,
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

impl<T: Config> Pallet<T> {
	pub fn offchain_signed_tx(
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
}

pub(crate) struct LocalVerificationStatus<'a> {
	storage_ref: StorageValueRef<'a>,
	key: &'a [u8],
}

impl<'a> LocalVerificationStatus<'a> {
	pub(crate) fn new(storage_key: &'a [u8]) -> Self {
		Self { storage_ref: StorageValueRef::persistent(storage_key), key: storage_key }
	}

	pub(crate) fn is_complete(&self) -> bool {
		match self.storage_ref.get::<()>() {
			Ok(Some(())) => true,
			Ok(None) => false,
			Err(e) => {
				log::warn!(
					"Failed to decode offchain storage for {}: {:?}",
					hex::encode(self.key),
					e
				);
				true
			},
		}
	}

	pub(crate) fn mark_complete(&self) {
		self.storage_ref.set(&());
	}
}

#[cfg(test)]
mod tests;
