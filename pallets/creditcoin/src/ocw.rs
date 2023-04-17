pub mod errors;
pub(crate) mod rpc;
pub(crate) mod tasks;

use self::{errors::RpcUrlError, rpc::errors::RpcError};
use super::ExternalAddress;
use crate::{Blockchain, ExternalTxId, TransferKind};
use alloc::string::String;
pub(crate) use errors::{OffchainError, VerificationFailureCause, VerificationResult};
use sp_runtime::offchain::storage::StorageValueRef;
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
	pub fn supports(&self, kind: &TransferKind) -> bool {
		match (self, kind) {
			(
				Blockchain::Ethereum | Blockchain::Luniverse | Blockchain::Rinkeby,
				TransferKind::Erc20(_) | TransferKind::Ethless(_) | TransferKind::Native,
			) => true,
			(Blockchain::Bitcoin, TransferKind::Native) => true,
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

#[cfg(test)]
mod tests;
