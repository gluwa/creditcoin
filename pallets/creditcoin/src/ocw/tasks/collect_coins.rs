use crate::types::Blockchain;
use frame_support::RuntimeDebug;
use hex_literal::hex;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::H160;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DeployedContract {
	pub address: H160,
	pub chain: Blockchain,
}

impl DeployedContract {
	const DEFAULT_CHAIN: Blockchain = Blockchain::Ethereum;
}

impl Default for DeployedContract {
	fn default() -> Self {
		let contract_chain: Blockchain = DeployedContract::DEFAULT_CHAIN;
		let contract_address: H160 = H160(hex!("a3EE21C306A700E682AbCdfe9BaA6A08F3820419"));
		Self { address: contract_address, chain: contract_chain }
	}
}
