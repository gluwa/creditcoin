pub mod errors;
pub mod rpc;
use crate::{Blockchain, Call, Transfer, TransferKind, UnverifiedTransfer};

use self::{
	errors::{OffchainError, RpcUrlError},
	rpc::{Address, EthTransaction, EthTransactionReceipt},
};

use super::{
	pallet::{Config, Error, Pallet},
	ExternalAddress, ExternalAmount, ExternalTxId, OrderId,
};
use alloc::string::String;
use core::str::FromStr;
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_types::U64;
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

fn validate_ethless_transfer(
	from: &Address,
	to: &Address,
	contract: &Address,
	amount: &ExternalAmount,
	receipt: &EthTransactionReceipt,
	transaction: &EthTransaction,
	eth_tip: U64,
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
		outputs: vec![Param { name: "success".into(), kind: ParamType::Bool, internal_type: None }],
		constant: false,
		state_mutability: StateMutability::NonPayable,
	};
	ensure!(
		receipt.is_success(),
		OffchainError::InvalidTransfer("ethless transfer was not successful")
	);
	let block_number = transaction
		.block_number
		.ok_or(OffchainError::InvalidTransfer("ethless transfer is still pending"))?;
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

	if let Some(to) = &transaction.to {
		ensure!(
			to == contract,
			OffchainError::InvalidTransfer("transaction was not sent through the ethless contract")
		);
	} else {
		return Err(OffchainError::InvalidTransfer(
			"ethless transaction lacks a receiver (contract creation transaction)",
		));
	}

	let inputs = transfer_fn.decode_input(&transaction.input.0[4..]).map_err(|e| {
		log::error!("failed to decode inputs: {:?}", e);
		OffchainError::InvalidTransfer(
			"ethless transfer inputs were not decodable with the expected ABI",
		)
	})?;
	ensure!(
		inputs.len() == transfer_fn.inputs.len(),
		OffchainError::InvalidTransfer("ethless transfer inputs were not of the expected length")
	);

	let input_from = match inputs.get(0) {
		Some(Token::Address(addr)) => addr,
		_ => {
			return Err(OffchainError::InvalidTransfer(
				"first input to ethless transfer was not an address",
			))
		},
	};
	ensure!(
		input_from == from,
		OffchainError::InvalidTransfer(
			"sender of ethless transfer does not match expected address"
		)
	);

	let input_to = match inputs.get(1) {
		Some(Token::Address(addr)) => addr,
		_ => {
			return Err(OffchainError::InvalidTransfer(
				"second input to ethless transfer was not an address",
			))
		},
	};
	ensure!(
		input_to == to,
		OffchainError::InvalidTransfer(
			"receiver of ethless transfer does not match expected address"
		)
	);

	let input_amount = match inputs.get(2) {
		Some(Token::Uint(value)) => ExternalAmount::from(value),
		_ => {
			return Err(OffchainError::InvalidTransfer(
				"third input to ethless transfer was not a Uint",
			))
		},
	};
	ensure!(
		&input_amount == amount,
		OffchainError::InvalidTransfer(
			"ethless transfer input amount does not match expected amount"
		)
	);

	Ok(())
}

impl<T: Config> Pallet<T> {
	pub fn verify_transfer_ocw(
		transfer: &UnverifiedTransfer<T::AccountId, BlockNumberFor<T>, T::Hash>,
	) -> OffchainResult<()> {
		let UnverifiedTransfer {
			transfer: Transfer { blockchain, kind, order_id, amount, tx, .. },
			from_external: from,
			to_external: to,
		} = transfer;
		match kind {
			TransferKind::Native => Err(OffchainError::InvalidTransfer(
				"support for native transfers is not yet implemented",
			)),
			TransferKind::Erc20(_) => Err(OffchainError::InvalidTransfer(
				"support for erc20 transfers is not yet implemented",
			)),
			TransferKind::Ethless(contract) => {
				Self::verify_ethless_transfer(blockchain, contract, from, to, order_id, amount, tx)
			},
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

	pub fn verify_ethless_transfer(
		blockchain: &Blockchain,
		contract_address: &ExternalAddress,
		from: &ExternalAddress,
		to: &ExternalAddress,
		_order_id: &OrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
	) -> OffchainResult<()> {
		let rpc_url = blockchain.rpc_url()?;
		let tx = rpc::eth_get_transaction(tx_id, &rpc_url)?;
		let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, &rpc_url)?;
		let eth_tip = rpc::eth_get_block_number(&rpc_url)?;

		let from_addr = parse_eth_address(from)?;
		let to_addr = parse_eth_address(to)?;

		let ethless_contract = parse_eth_address(contract_address)?;

		validate_ethless_transfer(
			&from_addr,
			&to_addr,
			&ethless_contract,
			amount,
			&tx_receipt,
			&tx,
			eth_tip,
		)?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::{convert::TryFrom, str::FromStr};

	use ethereum_types::{H160, U256, U64};
	use frame_support::{assert_ok, once_cell::sync::Lazy, BoundedVec};

	use super::{
		errors::OffchainError,
		parse_eth_address,
		rpc::{Address, EthTransaction, EthTransactionReceipt},
		validate_ethless_transfer, ETH_CONFIRMATIONS,
	};
	use crate::ExternalAddress;

	fn make_external_address(bytes: impl AsRef<[u8]>) -> ExternalAddress {
		BoundedVec::try_from(bytes.as_ref().to_vec()).unwrap()
	}

	fn assert_invalid_transfer<T>(result: Result<T, OffchainError>) {
		assert!(matches!(result, Err(OffchainError::InvalidTransfer(_))));
	}

	#[test]
	fn eth_address_non_utf8() {
		let address = make_external_address([0xfeu8, 0xfeu8, 0xffu8, 0xffu8]);

		assert!(matches!(parse_eth_address(&address), Err(OffchainError::InvalidTransfer(_))));
	}

	#[test]
	fn eth_address_bad_hex() {
		let address = make_external_address("0xP794f5ea0ba39494ce839613fffba74279579268");

		assert_invalid_transfer(parse_eth_address(&address));
	}

	#[test]
	fn eth_address_bad_len() {
		let too_long = make_external_address("0xb794f5ea0ba39494ce839613fffba742795792688888");
		let too_short = make_external_address("0xb794f5ea0b");

		assert_invalid_transfer(parse_eth_address(&too_long));
		assert_invalid_transfer(parse_eth_address(&too_short));
	}

	#[test]
	fn eth_address_valid() {
		let address_str = "0xb794f5ea0ba39494ce839613fffba74279579268";
		let address: ExternalAddress =
			BoundedVec::try_from(address_str.as_bytes().to_vec()).unwrap();

		let expected = H160::from_str(address_str).unwrap();
		assert_ok!(parse_eth_address(&address).map_err(|_| ()), expected);
	}

	const INPUT: &str = "0982d5b0000000000000000000000000f04349b4a760f5aed02131e0daa9bb99a1d1d1e5000000000000000000000000bbb8bbaf43fe8b9e5572b1860d5c94ac7ed87bb900000000000000000\
		000000000000000000000000000000000000000033336ec000000000000000000000000000000000000000000000000000000000323f4ac022a8243b45b35d97d7eb3192e7b95a3bbbe5cd0170059bd3a4c8da2912\
		841b500000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000410bec91682052bb53450c69a82ebc006d08113\
		6aef6cdb910bffab429620168792ab3b859b97d18cf53e6057208e20615b8a3ce6128fcb278b324d3e7ed9461671b00000000000000000000000000000000000000000000000000000000000000";

	const ETHLESS_FROM_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str("0xf04349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").unwrap());
	const ETHLESS_CONTRACT_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str("0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c").unwrap());
	const ETHLESS_TO_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str("0xBBb8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").unwrap());

	const ETH_TRANSACTION: Lazy<EthTransaction> = Lazy::new(|| EthTransaction {
		block_number: Some(5u64.into()),
		from: Some(ETHLESS_FROM_ADDR.clone()),
		to: Some(ETHLESS_CONTRACT_ADDR.clone()),
		input: hex::decode(INPUT).unwrap().into(),
		..Default::default()
	});

	struct EthlessTestArgs {
		from: Address,
		to: Address,
		contract: Address,
		amount: U256,
		receipt: EthTransactionReceipt,
		transaction: EthTransaction,
		tip: U64,
	}

	impl Default for EthlessTestArgs {
		fn default() -> Self {
			Self {
				from: ETHLESS_FROM_ADDR.clone(),
				to: ETHLESS_TO_ADDR.clone(),
				contract: ETHLESS_CONTRACT_ADDR.clone(),
				amount: U256::from(53688044u64),
				receipt: EthTransactionReceipt { status: Some(1u64.into()), ..Default::default() },
				transaction: ETH_TRANSACTION.clone(),
				tip: U64::from(ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS),
			}
		}
	}

	fn test_validate_ethless_transfer(args: EthlessTestArgs) -> Result<(), OffchainError> {
		let EthlessTestArgs { from, to, contract, amount, receipt, transaction, tip } = args;

		validate_ethless_transfer(&from, &to, &contract, &amount, &receipt, &transaction, tip)
	}

	#[test]
	fn ethless_transfer_valid() {
		assert_ok!(test_validate_ethless_transfer(EthlessTestArgs::default()));
	}

	#[test]
	fn ethless_transfer_tx_failed() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			receipt: EthTransactionReceipt { status: Some(0u64.into()), ..Default::default() },
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_tx_unconfirmed() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			tip: U64::from(ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS / 2),
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_tx_ahead_of_tip() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			tip: U64::from(ETH_TRANSACTION.block_number.unwrap() - 1),
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_contract_mismatch() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			contract: Address::from_str("0xbad1439a0e0bfdcd49939f9722866651a4aa9b3c").unwrap(),
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_from_mismatch() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			from: Address::from_str("0xbad349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").unwrap(),
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_to_mismatch() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			to: Address::from_str("0xbad8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").unwrap(),
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_invalid_input_data() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			transaction: EthTransaction {
				input: Vec::from("badbad".as_bytes()).into(),
				..ETH_TRANSACTION.clone()
			},
			..Default::default()
		}));
	}

	#[test]
	fn ethless_transfer_amount_mismatch() {
		assert_invalid_transfer(test_validate_ethless_transfer(EthlessTestArgs {
			amount: U256::from(1),
			..Default::default()
		}));
	}
}
