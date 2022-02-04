pub mod errors;
pub mod rpc;
use crate::{Blockchain, Call, PendingTransfer, Transfer, TransferKind};

use self::errors::{OffchainError, RpcUrlError};

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

pub type OffchainResult<T, E = errors::OffchainError> = Result<T, E>;

impl Blockchain {
	pub fn rpc_url(&self) -> OffchainResult<String, errors::RpcUrlError> {
		let chain_prefix = self.as_bytes();
		let mut buf = Vec::from(chain_prefix);
		buf.extend("-rpc-url".bytes());
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

fn parse_ethless_address(address: &ExternalAddress) -> OffchainResult<rpc::Address> {
	let address = core::str::from_utf8(address).map_err(|err| {
		log::error!("ethless address {:?} is not valid utf8: {}", address, err);
		OffchainError::InvalidTransfer("ethless address is invalid utf8")
	})?;
	let address = rpc::Address::from_str(address).map_err(|err| {
		log::error!("ethless address {:?} is not valid hex: {}", address, err);
		OffchainError::InvalidTransfer("ethless address is invalid hex")
	})?;
	Ok(address)
}

impl<T: Config> Pallet<T> {
	pub fn verify_transfer(
		transfer: &PendingTransfer<T::AccountId, BlockNumberFor<T>, T::Hash>,
	) -> OffchainResult<()> {
		let PendingTransfer {
			transfer: Transfer { blockchain, kind, order, amount, tx, .. },
			from,
			to,
		} = transfer;
		match kind {
			TransferKind::Native => Err(OffchainError::InvalidTransfer(
				"support for native transfers is not yet implemented",
			)),
			TransferKind::Erc20(_) => Err(OffchainError::InvalidTransfer(
				"support for erc20 transfers is not yet implemented",
			)),
			TransferKind::Ethless(contract) =>
				Self::verify_ethless_transfer(blockchain, contract, from, to, order, amount, tx),
			TransferKind::Other(_) => Err(OffchainError::InvalidTransfer(
				"support for other transfers is not yet implemented",
			)),
		}
	}

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
		blockchain: &Blockchain,
		contract_address: &ExternalAddress,
		from: &ExternalAddress,
		to: &ExternalAddress,
		_order_id: &OrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
	) -> OffchainResult<()> {
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
		let rpc_url = blockchain.rpc_url()?;
		let tx = rpc::eth_get_transaction(tx_id, &rpc_url)?;
		let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, &rpc_url)?;
		ensure!(
			tx_receipt.is_success(),
			OffchainError::InvalidTransfer("ethless transfer was not successful")
		);
		let block_number = tx
			.block_number
			.ok_or(OffchainError::InvalidTransfer("ethless transfer is still pending"))?;
		let eth_tip = rpc::eth_get_block_number(&rpc_url)?;
		ensure!(
			block_number < eth_tip,
			OffchainError::InvalidTransfer(
				"block number of ethless transfer is greater than the ethereum tip"
			)
		);
		let diff = eth_tip - block_number;
		ensure!(
			diff.as_u64() >= ETH_CONFIRMATIONS,
			OffchainError::InvalidTransfer("ethless transfer does not have enough confirmations")
		);

		let from_addr = parse_ethless_address(from)?;
		let to_addr = parse_ethless_address(to)?;

		let ethless_contract = parse_ethless_address(contract_address)?;

		if let Some(to) = tx.to {
			ensure!(
				to == ethless_contract,
				OffchainError::InvalidTransfer(
					"transaction was not sent through the ethless contract"
				)
			);
		} else {
			return Err(OffchainError::InvalidTransfer(
				"ethless transaction lacks a receiver (contract creation transaction)",
			))
		}

		let inputs = transfer_fn.decode_input(&tx.input.0[4..]).map_err(|e| {
			log::error!("failed to decode inputs: {:?}", e);
			OffchainError::InvalidTransfer(
				"ethless transfer inputs were not decodable with the expected ABI",
			)
		})?;
		ensure!(
			inputs.len() == transfer_fn.inputs.len(),
			OffchainError::InvalidTransfer(
				"ethless transfer inputs were not of the expected length"
			)
		);

		let input_from = match inputs.get(0) {
			Some(Token::Address(addr)) => addr,
			_ =>
				return Err(OffchainError::InvalidTransfer(
					"first input to ethless transfer was not an address",
				)),
		};
		ensure!(
			input_from == &from_addr,
			OffchainError::InvalidTransfer(
				"sender of ethless transfer does not match expected address"
			)
		);

		let input_to = match inputs.get(1) {
			Some(Token::Address(addr)) => addr,
			_ =>
				return Err(OffchainError::InvalidTransfer(
					"second input to ethless transfer was not an address",
				)),
		};
		ensure!(
			input_to == &to_addr,
			OffchainError::InvalidTransfer(
				"receiver of ethless transfer does not match expected address"
			)
		);

		let input_amount = match inputs.get(2) {
			Some(Token::Uint(value)) => ExternalAmount::from(value),
			_ =>
				return Err(OffchainError::InvalidTransfer(
					"third input to ethless transfer was not a Uint",
				)),
		};
		ensure!(
			&input_amount == amount,
			OffchainError::InvalidTransfer(
				"ethless transfer input amount does not match expected amount"
			)
		);

		Ok(())
	}
}
