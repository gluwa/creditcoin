pub mod rpc;
use crate::{Call, Network};

use super::{
	pallet::{Config, Error, Pallet},
	ExternalAddress, ExternalAmount, ExternalTxId, OrderId,
};
use alloc::string::String;
use ethabi::{Function, Param, ParamType, StateMutability};
use frame_system::{
	offchain::{Account, SendSignedTransaction, Signer},
	pallet_prelude::BlockNumberFor,
};
use sp_runtime::offchain::storage::StorageValueRef;
use sp_std::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum ExternalChain {
	Ethereum,
	Ethless,
}

impl ExternalChain {
	pub fn rpc_url(self, network: &Network) -> Result<String, ()> {
		let mut buf = Vec::from(match self {
			ExternalChain::Ethless => "ethless-",
			ExternalChain::Ethereum => "ethereum-",
		});
		buf.extend(network.iter().copied());
		buf.extend("-rpc-url".bytes());
		let rpc_url_storage = StorageValueRef::persistent(&buf);
		if let Some(url_bytes) = rpc_url_storage.get::<Vec<u8>>().map_err(|e| {
			log::error!("failed to retrieve rpc url from storage: {:?}", e);
			()
		})? {
			Ok(String::from_utf8(url_bytes).map_err(|e| {
				log::error!("rpc url is invalid utf8: {}", e);
				()
			})?)
		} else {
			Err(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn verify_ethless_transfer(
		network: &Network,
		from: &ExternalAddress,
		to: &ExternalAddress,
		order_id: &OrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
	) -> Result<(), ()> {
		let transfer_func = Function {
			name: "transfer".into(),
			inputs: vec![
				Param { name: "_from".into(), kind: ParamType::Address, internal_type: None },
				Param { name: "_to".into(), kind: ParamType::Address, internal_type: None },
				Param { name: "_value".into(), kind: ParamType::Uint(256), internal_type: None },
				Param { name: "_fee".into(), kind: ParamType::Uint(256), internal_type: None },
				Param { name: "_nonce".into(), kind: ParamType::Uint(256), internal_type: None },
				Param { name: "_sig".into(), kind: ParamType::Bytes, internal_type: None },
			],
			outputs: vec![Param {
				name: "success".into(),
				kind: ParamType::Bool,
				internal_type: None,
			}],
			constant: false,
			state_mutability: StateMutability::NonPayable,
		};
		let rpc_url = ExternalChain::rpc_url(ExternalChain::Ethless, network)?;
		let tx = rpc::get_eth_transaction(tx_id, &rpc_url)?;

		todo!()
	}
}
