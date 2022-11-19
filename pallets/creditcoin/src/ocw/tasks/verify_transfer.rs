use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_types::U64;
use frame_support::ensure;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::U256;
use sp_runtime::traits::UniqueSaturatedFrom;
#[cfg(not(feature = "std"))]
use sp_std::prelude::*;

use crate::{
	ocw::{
		self, parse_eth_address,
		rpc::{self, Address, EthBlock, EthTransaction, EthTransactionReceipt},
		OffchainError, OffchainResult, VerificationFailureCause, VerificationResult,
		ETH_CONFIRMATIONS,
	},
	Blockchain, Config, Currency, DealOrderId, EvmChainId, EvmInfo, ExternalAddress,
	ExternalAmount, ExternalTxId, Id, LegacyTransferKind, Transfer, UnverifiedTransfer,
};

pub(crate) fn ethless_transfer_function_abi() -> Function {
	#[allow(deprecated)]
	Function {
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
		constant: Some(false),
		state_mutability: StateMutability::NonPayable,
	}
}

pub(in crate::ocw) fn validate_ethless_transfer(
	from: &Address,
	to: &Address,
	contract: &Address,
	amount: &ExternalAmount,
	receipt: &EthTransactionReceipt,
	transaction: &EthTransaction,
	eth_tip: U64,
	id_hash: impl ethereum_types::BigEndianHash<Uint = U256>,
) -> OffchainResult<()> {
	let transfer_fn = ethless_transfer_function_abi();
	ensure!(receipt.is_success(), VerificationFailureCause::TaskFailed);

	let block_number = transaction.block_number.ok_or(VerificationFailureCause::TaskPending)?;

	ensure!(block_number <= eth_tip, VerificationFailureCause::TaskInFuture);

	let diff = eth_tip - block_number;

	ensure!(diff.as_u64() >= ETH_CONFIRMATIONS, VerificationFailureCause::TaskUnconfirmed);

	if let Some(to) = &transaction.to {
		ensure!(to == contract, VerificationFailureCause::IncorrectContract);
	} else {
		return Err(VerificationFailureCause::MissingReceiver.into());
	}

	let inputs = transfer_fn.decode_input(transaction.input()).map_err(|e| {
		log::error!("failed to decode inputs: {:?}", e);
		VerificationFailureCause::AbiMismatch
	})?;

	// IncorrectInputLength and IncorrectInputType are unreachable
	// under normal circumstances. We get AbiMismatch or InvalidData errors
	ensure!(
		inputs.len() == transfer_fn.inputs.len(),
		VerificationFailureCause::IncorrectInputLength
	);

	let input_from = match inputs.get(0) {
		Some(Token::Address(addr)) => addr,
		_ => return Err(VerificationFailureCause::IncorrectInputType.into()),
	};
	ensure!(input_from == from, VerificationFailureCause::IncorrectSender);

	let input_to = match inputs.get(1) {
		Some(Token::Address(addr)) => addr,
		_ => return Err(VerificationFailureCause::IncorrectInputType.into()),
	};
	ensure!(input_to == to, VerificationFailureCause::IncorrectReceiver);

	let input_amount = match inputs.get(2) {
		Some(Token::Uint(value)) => ExternalAmount::from(value),
		_ => return Err(VerificationFailureCause::IncorrectInputType.into()),
	};
	ensure!(&input_amount == amount, VerificationFailureCause::IncorrectAmount);

	let nonce = match inputs.get(4) {
		Some(Token::Uint(value)) => ExternalAmount::from(value),
		_ => return Err(VerificationFailureCause::IncorrectInputType.into()),
	};
	let expected_nonce = id_hash.into_uint();
	ensure!(nonce == expected_nonce, VerificationFailureCause::IncorrectNonce);

	Ok(())
}

fn verify_chain_id(rpc_url: &str, expected: EvmChainId) -> VerificationResult<()> {
	let id = rpc::eth_chain_id(rpc_url)?.as_u64();
	if id == expected.as_u64() {
		Ok(())
	} else {
		Err(OffchainError::IncorrectChainId)
	}
}

impl<T: Config> crate::Pallet<T> {
	pub fn verify_transfer_ocw(
		transfer: &UnverifiedTransfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>,
	) -> VerificationResult<Option<T::Moment>> {
		let UnverifiedTransfer {
			transfer: Transfer { blockchain, deal_order_id, amount, tx_id: tx, .. },
			from_external: from,
			to_external: to,
			currency_to_check,
			..
		} = transfer;
		log::debug!("verifying OCW transfer");
		match currency_to_check {
			crate::CurrencyOrLegacyTransferKind::TransferKind(kind) => match kind {
				LegacyTransferKind::Ethless(contract) => Self::verify_ethless_transfer(
					blockchain,
					contract,
					from,
					to,
					deal_order_id,
					amount,
					tx,
					None,
				),
				LegacyTransferKind::Native
				| LegacyTransferKind::Erc20(_)
				| LegacyTransferKind::Other(_) => Err(VerificationFailureCause::UnsupportedMethod.into()),
			},
			crate::CurrencyOrLegacyTransferKind::Currency(currency) => match currency {
				Currency::Evm(currency_type, EvmInfo { chain_id }) => match currency_type {
					crate::EvmCurrencyType::SmartContract(contract, _) => {
						Self::verify_ethless_transfer(
							blockchain,
							contract,
							from,
							to,
							deal_order_id,
							amount,
							tx,
							Some(*chain_id),
						)
					},
				},
			},
		}
	}

	pub fn verify_ethless_transfer(
		blockchain: &Blockchain,
		contract_address: &ExternalAddress,
		from: &ExternalAddress,
		to: &ExternalAddress,
		deal_order_id: &DealOrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
		chain_id: Option<EvmChainId>,
	) -> VerificationResult<Option<T::Moment>> {
		let rpc_url = blockchain.rpc_url()?;

		if let Some(chain_id) = chain_id {
			verify_chain_id(&rpc_url, chain_id)?;
		}

		let tx = ocw::eth_get_transaction(tx_id, &rpc_url)?;
		let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, &rpc_url)?;
		let eth_tip = rpc::eth_get_block_number(&rpc_url)?;

		let tx_block_num = tx.block_number;

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
			T::HashIntoNonce::from(deal_order_id.hash()),
		)?;

		let timestamp = if let Some(num) = tx_block_num {
			if let Ok(EthBlock { timestamp: block_timestamp }) =
				rpc::eth_get_block_by_number(num, &rpc_url)
			{
				Some(T::Moment::unique_saturated_from(block_timestamp.as_u64()))
			} else {
				None
			}
		} else {
			None
		};

		Ok(timestamp)
	}
}
