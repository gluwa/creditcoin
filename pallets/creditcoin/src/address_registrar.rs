use crate::types::Blockchain;

pub trait ExternalAddressRegistrar {
	fn is_blockchain_supported(&self, blockchain: &Blockchain) -> bool;
}
