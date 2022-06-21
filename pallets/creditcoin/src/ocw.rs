pub mod collect_coins;
pub mod errors;
pub mod rpc;
pub mod task;
use crate::{Blockchain, Call, Id, Transfer, TransferKind, UnverifiedTransfer};
pub use codec::EncodeLike;
pub use errors::{OffchainError, VerificationFailureCause, VerificationResult};
use task::guard::LocalTaskStatus;

use self::{
	errors::RpcUrlError,
	rpc::{errors::RpcError, Address, EthBlock, EthTransaction, EthTransactionReceipt},
};

use super::{
	pallet::{Config, Error, Pallet, Store},
	ExternalAddress, ExternalAmount, ExternalTxId, OrderId,
};
use alloc::string::String;
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_types::{U256, U64};
use frame_support::ensure;
use frame_system::{
	offchain::{Account, SendSignedTransaction, Signer},
	pallet_prelude::BlockNumberFor,
};
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::traits::UniqueSaturatedFrom;
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
	let address_bytes = <[u8; 20]>::try_from(address.as_slice())
		.map_err(|_| VerificationFailureCause::InvalidAddress)?;
	let address = rpc::Address::from(address_bytes);
	Ok(address)
}

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
		constant: false,
		state_mutability: StateMutability::NonPayable,
	}
}

fn validate_ethless_transfer(
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

	let inputs = transfer_fn.decode_input(&transaction.input.0[4..]).map_err(|e| {
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

impl<T: Config> Pallet<T> {
	pub(crate) fn ocw_result_handler<O: core::fmt::Debug>(
		verification_result: VerificationResult<O>,
		success_dispatcher: impl Fn(O) -> Result<(), Error<T>>,
		failure_dispatcher: impl Fn(VerificationFailureCause) -> Result<(), Error<T>>,
		status: LocalTaskStatus,
		unverified_task: &impl core::fmt::Debug,
	) {
		log::debug!("Task Verification result: {:?}", verification_result);
		//test branches
		match verification_result {
			Ok(output) => {
				if let Err(e) = success_dispatcher(output) {
					log::error!("Failed to send success dispatchable transaction: {:?}", e);
					status.keep_alive();
				}
			},
			Err(OffchainError::InvalidTask(cause)) => {
				log::warn!("Failed to verify pending task {:?} : {:?}", unverified_task, cause);
				if cause.is_fatal() {
					if let Err(e) = failure_dispatcher(cause) {
						log::error!("Failed to send fail dispatchable transaction: {:?}", e);
						status.keep_alive();
					}
				} else {
					status.keep_alive();
				}
			},
			Err(error) => {
				log::error!("Task verification encountered an error {:?}", error);
				status.keep_alive();
			},
		}
	}

	pub fn verify_transfer_ocw(
		u_transfer: &UnverifiedTransfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>,
	) -> VerificationResult<Transfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>> {
		let UnverifiedTransfer {
			transfer: Transfer { blockchain, kind, order_id, amount, tx_id: tx, .. },
			from_external: from,
			to_external: to,
			..
		} = u_transfer;
		log::debug!("verifying OCW transfer");
		match kind {
			TransferKind::Ethless(contract) => {
				let timestamp = Self::verify_ethless_transfer(
					blockchain, contract, from, to, order_id, amount, tx,
				)?;
				Ok(Transfer { timestamp, ..u_transfer.transfer.clone() })
			},
			TransferKind::Native | TransferKind::Erc20(_) | TransferKind::Other(_) => {
				Err(VerificationFailureCause::UnsupportedMethod.into())
			},
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
		order_id: &OrderId<BlockNumberFor<T>, T::Hash>,
		amount: &ExternalAmount,
		tx_id: &ExternalTxId,
	) -> VerificationResult<Option<T::Moment>> {
		let rpc_url = blockchain.rpc_url()?;
		let tx = rpc::eth_get_transaction(tx_id, &rpc_url).map_err(|e| {
			if let RpcError::NoResult = e {
				OffchainError::InvalidTask(VerificationFailureCause::TaskNonexistent)
			} else {
				e.into()
			}
		})?;
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
			T::HashIntoNonce::from(order_id.hash()),
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

impl<T, K2> task::Task<T, T::BlockNumber, K2>
	for UnverifiedTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>
where
	T: Config,
	K2: EncodeLike<crate::types::TransferId<T::Hash>>,
{
	type VerifiedTask = Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>;

	fn verify(&self) -> VerificationResult<Self::VerifiedTask> {
		Pallet::<T>::verify_transfer_ocw(self)
	}

	fn failure_call(&self, deadline: T::BlockNumber, cause: VerificationFailureCause) -> Call<T> {
		Call::fail_transfer {
			deadline,
			transfer_id: crate::types::TransferId::new::<T>(
				&self.transfer.blockchain,
				&self.transfer.tx_id,
			),
			cause,
		}
	}

	fn success_call(&self, deadline: T::BlockNumber, verified_task: Self::VerifiedTask) -> Call<T> {
		Call::verify_transfer { transfer: verified_task, deadline }
	}

	fn is_complete(persistent_storage_key: K2) -> bool {
		<Pallet<T> as Store>::Transfers::contains_key(persistent_storage_key)
	}
}

#[cfg(test)]
mod tests;
