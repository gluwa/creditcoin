pub mod rpc;
use crate::{Call, Network};

use super::{
	pallet::{Config, Error, Pallet},
	ExternalAddress, ExternalAmount, ExternalTxId, OrderId,
};
use alloc::string::String;
use core::str::FromStr;
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use frame_support::ensure;
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

const ETH_CONFIRMATIONS: u64 = 12;

fn split_ethless_address(address: &ExternalAddress) -> Result<(rpc::Address, rpc::Address), ()> {
	let mut segments = address.split(|&byte| byte == b'@');
	let contract = segments.next().ok_or(())?;
	let contract = core::str::from_utf8(contract).map_err(|_| ())?;
	let contract = rpc::Address::from_str(contract).map_err(|_| ())?;
	let address = segments.next().ok_or(())?;
	let address = core::str::from_utf8(address).map_err(|_| ())?;
	let address = rpc::Address::from_str(address).map_err(|_| ())?;
	Ok((contract, address))
}

impl<T: Config> Pallet<T> {
	pub fn offchain_signed_tx(
		auth_id: T::FromAccountId,
		call: impl Fn(&Account<T>) -> Call<T>,
	) -> Result<(), Error<T>> {
		use sp_core::crypto::UncheckedFrom;
		let auth_bytes: &[u8; 32] = auth_id.as_ref();
		let public = T::InternalPublic::unchecked_from(*auth_bytes);
		let public: T::PublicSigning = public.into();
		let signer =
			Signer::<T, T::AuthorityId>::any_account().with_filter(sp_std::vec![public.into()]);
		let result = signer.send_signed_transaction(call);

		if let Some((acc, res)) = result {
			if res.is_err() {
				log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(Error::OffchainSignedTxFailed)
			} else {
				return Ok(())
			}
		}

		log::error!("No local account available");
		Err(Error::NoLocalAcctForSignedTx)
	}

	pub fn verify_ethless_transfer(
		network: &Network,
		from: &ExternalAddress,
		to: &ExternalAddress,
		order_id: &OrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
	) -> Result<(), ()> {
		#[allow(deprecated)]
		let transfer_fn = Function {
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
		let tx = rpc::eth_get_transaction(tx_id, &rpc_url)?;
		let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, &rpc_url)?;
		ensure!(tx_receipt.is_success(), ());
		let block_number = tx.block_number.ok_or(())?;
		let eth_tip = rpc::eth_get_block_number(&rpc_url)?;
		ensure!(block_number < eth_tip, ());
		let diff = eth_tip - block_number;
		ensure!(diff.as_u64() >= ETH_CONFIRMATIONS, ());

		let (from_contract, from_addr) = split_ethless_address(from)?;
		let (to_contract, to_addr) = split_ethless_address(to)?;
		ensure!(from_contract == to_contract, ());

		let ethless_contract = from_contract;

		if let Some(to) = tx.to {
			ensure!(to == ethless_contract, ());
		} else {
			return Err(())
		}

		let inputs = transfer_fn.decode_input(&tx.input.0).map_err(|e| {
			log::error!("failed to decode inputs: {:?}", e);
			()
		})?;
		ensure!(inputs.len() == transfer_fn.inputs.len(), ());

		let input_from = match inputs.get(0) {
			Some(Token::Address(addr)) => addr,
			_ => return Err(()),
		};
		ensure!(input_from == &from_addr, ());

		let input_to = match inputs.get(1) {
			Some(Token::Address(addr)) => addr,
			_ => return Err(()),
		};
		ensure!(input_to == &to_addr, ());

		let input_amount = match inputs.get(2) {
			Some(Token::Uint(value)) => ExternalAmount::from(value),
			_ => return Err(()),
		};
		ensure!(&input_amount == amount, ());

		let input_nonce = match inputs.get(4) {
			Some(Token::Uint(value)) => value,
			_ => return Err(()),
		};
		let expected_nonce = match order_id {
			OrderId::Deal(_) => ethereum_types::U256::zero(),
			OrderId::Repayment(_) => ethereum_types::U256::one(),
		};
		ensure!(input_nonce == &expected_nonce, ());

		Ok(())
	}
}
