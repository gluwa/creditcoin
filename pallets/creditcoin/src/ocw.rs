pub mod errors;
pub(crate) mod rpc;
pub(crate) mod tasks;

use self::{errors::RpcUrlError, rpc::errors::RpcError};
use super::ExternalAddress;
use crate::{Blockchain, ExternalTxId};
use alloc::string::String;
pub(crate) use errors::{OffchainError, VerificationFailureCause, VerificationResult};
use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::prelude::*;

pub(crate) type OffchainResult<T, E = errors::OffchainError> = Result<T, E>;

impl Blockchain {
	pub fn rpc_url(&self) -> OffchainResult<String, RpcUrlError> {
		let key = self.rpc_key();
		let rpc_url_storage = StorageValueRef::persistent(&key);
		if let Some(url_bytes) = rpc_url_storage.get::<Vec<u8>>()? {
			Ok(String::from_utf8(url_bytes)?)
		} else {
			Err(RpcUrlError::NoValue)
		}
	}

	pub fn rpc_key(&self) -> Vec<u8> {
		let chain_prefix = self.as_bytes();
		let mut buf = Vec::from(chain_prefix);
		buf.extend("-rpc-uri".bytes());
		buf
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

#[cfg(test)]
mod tests;
