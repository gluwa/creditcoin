pub mod errors;
pub mod rpc;
use crate::{Blockchain, Call, Id, Transfer, TransferKind, UnverifiedTransfer};
pub use errors::{OffchainError, VerificationFailureCause, VerificationResult};

use self::{
	errors::RpcUrlError,
	rpc::{Address, EthBlock, EthTransaction, EthTransactionReceipt},
};

use super::{
	pallet::{Config, Error, Pallet},
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
	ensure!(receipt.is_success(), VerificationFailureCause::TransferFailed);

	let block_number = transaction.block_number.ok_or(VerificationFailureCause::TransferPending)?;

	ensure!(block_number <= eth_tip, VerificationFailureCause::TransferInFuture);

	let diff = eth_tip - block_number;

	ensure!(diff.as_u64() >= ETH_CONFIRMATIONS, VerificationFailureCause::TransferUnconfirmed);

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
	pub fn verify_transfer_ocw(
		transfer: &UnverifiedTransfer<T::AccountId, BlockNumberFor<T>, T::Hash, T::Moment>,
	) -> OffchainResult<VerificationResult<T::Moment>> {
		let UnverifiedTransfer {
			transfer: Transfer { blockchain, kind, order_id, amount, tx_id: tx, .. },
			from_external: from,
			to_external: to,
		} = transfer;
		let result = match kind {
			TransferKind::Ethless(contract) => {
				Self::verify_ethless_transfer(blockchain, contract, from, to, order_id, amount, tx)
			},
			TransferKind::Native | TransferKind::Erc20(_) | TransferKind::Other(_) => {
				Err(VerificationFailureCause::UnsupportedMethod.into())
			},
		};
		match result {
			Ok(timestamp) => Ok(VerificationResult::Success { timestamp }),
			Err(e) => match e {
				OffchainError::InvalidTransfer(failure) => Ok(VerificationResult::from(failure)),
				error => Err(error),
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
	) -> OffchainResult<Option<T::Moment>> {
		let rpc_url = blockchain.rpc_url()?;
		let tx = rpc::eth_get_transaction(tx_id, &rpc_url)?;
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

#[cfg(test)]
mod tests {
	use core::fmt::Debug;
	use std::{convert::TryFrom, str::FromStr};

	use super::errors::{
		RpcUrlError,
		VerificationFailureCause::{self, *},
	};
	use crate::{
		mock::MockedRpcRequests,
		mock::{
			get_mock_amount, get_mock_contract, get_mock_from_address, get_mock_input_data,
			get_mock_nonce, get_mock_to_address, set_rpc_uri, Test as TestRuntime,
		},
		ocw::{
			errors::VerificationResult,
			rpc::{errors::RpcError, JsonRpcResponse},
		},
		tests::TestInfo,
		Id, LoanTerms, TransferKind,
	};
	use alloc::sync::Arc;
	use assert_matches::assert_matches;
	use codec::Decode;
	use ethabi::Token;
	use ethereum_types::{BigEndianHash, H160, U256, U64};
	use frame_support::{assert_ok, once_cell::sync::Lazy, BoundedVec};
	use parking_lot::RwLock;
	use sp_core::H256;
	use sp_runtime::{
		offchain::{
			storage::{StorageRetrievalError, StorageValueRef},
			testing::OffchainState,
		},
		traits::IdentifyAccount,
	};

	use super::{
		errors::OffchainError,
		ethless_transfer_function_abi, parse_eth_address,
		rpc::{Address, EthTransaction, EthTransactionReceipt},
		validate_ethless_transfer, ETH_CONFIRMATIONS,
	};
	use crate::{
		mock::{AccountId, ExtBuilder},
		Blockchain, ExternalAddress,
	};

	fn make_external_address(hex_str: &str) -> ExternalAddress {
		BoundedVec::try_from(hex::decode(hex_str.trim_start_matches("0x")).unwrap()).unwrap()
	}

	#[track_caller]
	fn assert_invalid_transfer<T: Debug>(
		result: Result<T, OffchainError>,
		cause: VerificationFailureCause,
	) {
		assert_matches!(result, Err(OffchainError::InvalidTransfer(why)) => { assert_eq!(why, cause); } );
	}

	fn default_nonce() -> U256 {
		U256::from_dec_str(
			"979732326222468652918279417612319888321218652914508214827914231471334244789",
		)
		.unwrap()
	}

	#[derive(Clone, Debug, PartialEq)]
	struct TransferContractInput {
		from: Address,
		to: Address,
		value: U256,
		fee: U256,
		nonce: U256,
		sig: Vec<u8>,
	}

	impl Default for TransferContractInput {
		fn default() -> Self {
			Self {
				from: ETHLESS_FROM_ADDR.clone(),
				to: ETHLESS_TO_ADDR.clone(),
				value: U256::from(100),
				fee: 1.into(),
				nonce: default_nonce(),
				sig: vec![],
			}
		}
	}

	impl TransferContractInput {
		fn into_tokens(self) -> Vec<Token> {
			vec![
				Token::Address(self.from),
				Token::Address(self.to),
				Token::Uint(self.value),
				Token::Uint(self.fee),
				Token::Uint(self.nonce),
				Token::Bytes(self.sig),
			]
		}
	}

	#[test]
	fn eth_address_bad_len() {
		let too_long = make_external_address("0xb794f5ea0ba39494ce839613fffba742795792688888");
		let too_short = make_external_address("0xb794f5ea0b");

		assert_invalid_transfer(parse_eth_address(&too_long), InvalidAddress);
		assert_invalid_transfer(parse_eth_address(&too_short), InvalidAddress);
	}

	#[test]
	fn eth_address_valid() {
		let address: ExternalAddress =
			make_external_address("0xb794f5ea0ba39494ce839613fffba74279579268");

		let expected = H160::from(<[u8; 20]>::try_from(address.as_slice()).unwrap());
		assert_eq!(parse_eth_address(&address).unwrap(), expected);
	}

	static ETHLESS_INPUT: Lazy<String> =
		Lazy::new(|| get_mock_input_data().trim_start_matches("0x").into());

	static ETHLESS_FROM_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str(&get_mock_from_address()).unwrap());
	static ETHLESS_CONTRACT_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str(&get_mock_contract()).unwrap());
	static ETHLESS_TO_ADDR: Lazy<Address> =
		Lazy::new(|| Address::from_str(&get_mock_to_address()).unwrap());

	static ETH_TRANSACTION: Lazy<EthTransaction> = Lazy::new(|| EthTransaction {
		block_number: Some(5u64.into()),
		from: Some(ETHLESS_FROM_ADDR.clone()),
		to: Some(ETHLESS_CONTRACT_ADDR.clone()),
		input: hex::decode(&*ETHLESS_INPUT).unwrap().into(),
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
		nonce: U256,
	}

	impl Default for EthlessTestArgs {
		fn default() -> Self {
			Self {
				from: ETHLESS_FROM_ADDR.clone(),
				to: ETHLESS_TO_ADDR.clone(),
				contract: ETHLESS_CONTRACT_ADDR.clone(),
				amount: get_mock_amount(),
				receipt: EthTransactionReceipt { status: Some(1u64.into()), ..Default::default() },
				transaction: ETH_TRANSACTION.clone(),
				tip: U64::from(ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS),
				nonce: get_mock_nonce(),
			}
		}
	}

	fn test_validate_ethless_transfer(args: EthlessTestArgs) -> Result<(), OffchainError> {
		let EthlessTestArgs { from, to, contract, amount, receipt, transaction, tip, nonce } = args;

		validate_ethless_transfer(
			&from,
			&to,
			&contract,
			&amount,
			&receipt,
			&transaction,
			tip,
			H256::from_uint(&nonce),
		)
	}

	fn default<T: Default>() -> T {
		Default::default()
	}

	#[test]
	fn ethless_transfer_valid() {
		assert_ok!(test_validate_ethless_transfer(EthlessTestArgs::default()));
	}

	#[test]
	fn ethless_transfer_tx_failed() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				receipt: EthTransactionReceipt { status: Some(0u64.into()), ..Default::default() },
				..Default::default()
			}),
			TransferFailed,
		);
	}

	#[test]
	fn ethless_transfer_tx_unconfirmed() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				tip: U64::from(ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS / 2),
				..Default::default()
			}),
			TransferUnconfirmed,
		);
	}

	#[test]
	fn ethless_transfer_tx_missing_to() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				transaction: EthTransaction { to: None, ..ETH_TRANSACTION.clone() },
				..Default::default()
			}),
			MissingReceiver,
		);
	}

	#[test]
	fn ethless_transfer_tx_ahead_of_tip() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				tip: U64::from(ETH_TRANSACTION.block_number.unwrap() - 1),
				..Default::default()
			}),
			TransferInFuture,
		);
	}

	#[test]
	fn ethless_transfer_contract_mismatch() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				contract: Address::from_str("0xbad1439a0e0bfdcd49939f9722866651a4aa9b3c").unwrap(),
				..Default::default()
			}),
			IncorrectContract,
		);
	}

	#[test]
	fn ethless_transfer_from_mismatch() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				from: Address::from_str("0xbad349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").unwrap(),
				..Default::default()
			}),
			IncorrectSender,
		);
	}

	#[test]
	fn ethless_transfer_to_mismatch() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				to: Address::from_str("0xbad8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").unwrap(),
				..Default::default()
			}),
			IncorrectReceiver,
		);
	}

	#[test]
	fn ethless_transfer_invalid_input_data() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				transaction: EthTransaction {
					input: Vec::from("badbad".as_bytes()).into(),
					..ETH_TRANSACTION.clone()
				},
				..Default::default()
			}),
			AbiMismatch,
		);
	}

	#[test]
	fn ethless_transfer_amount_mismatch() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				amount: U256::from(1),
				..Default::default()
			}),
			IncorrectAmount,
		);
	}

	#[test]
	fn ethless_transfer_nonce_mismatch() {
		let transfer = ethless_transfer_function_abi();
		let input_args = TransferContractInput { nonce: 1.into(), ..Default::default() };
		let input = transfer.encode_input(&input_args.into_tokens()).unwrap().into();
		let transaction = EthTransaction { input, ..ETH_TRANSACTION.clone() };
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
			IncorrectNonce,
		);
	}

	#[test]
	fn ethless_transfer_pending() {
		assert_invalid_transfer(
			test_validate_ethless_transfer(EthlessTestArgs {
				transaction: EthTransaction { block_number: None, ..ETH_TRANSACTION.clone() },
				..Default::default()
			}),
			TransferPending,
		)
	}

	#[test]
	fn blockchain_rpc_url_missing() {
		ExtBuilder::default().build_offchain_and_execute(|| {
			assert_eq!(Blockchain::Ethereum.rpc_url(), Err(RpcUrlError::NoValue));
		})
	}

	#[test]
	fn blockchain_rpc_url_non_utf8() {
		ExtBuilder::default().build_offchain_and_execute(|| {
			set_rpc_uri(&Blockchain::Ethereum, &[0x80]);

			assert_matches!(
				Blockchain::Ethereum.rpc_url().unwrap_err(),
				RpcUrlError::InvalidUrl(_)
			);
		});
	}

	#[test]
	fn blockchain_rpc_url_invalid_scale() {
		ExtBuilder::default().build_offchain_and_execute(|| {
			let rpc_url_storage = StorageValueRef::persistent(&*b"ethereum-rpc-uri");
			rpc_url_storage.set(&[0x80]);

			assert_matches!(
				dbg!(Blockchain::Ethereum.rpc_url()).unwrap_err(),
				RpcUrlError::StorageFailure(StorageRetrievalError::Undecodable)
			);
		});
	}

	#[test]
	fn blockchain_supports_etherlike() {
		assert!(Blockchain::Ethereum.supports(&crate::TransferKind::Native));
		assert!(Blockchain::Rinkeby.supports(&crate::TransferKind::Native));
		assert!(Blockchain::Luniverse.supports(&crate::TransferKind::Native));
		assert!(Blockchain::Ethereum.supports(&crate::TransferKind::Erc20(default())));
		assert!(Blockchain::Rinkeby.supports(&crate::TransferKind::Erc20(default())));
		assert!(Blockchain::Luniverse.supports(&crate::TransferKind::Erc20(default())));
		assert!(Blockchain::Ethereum.supports(&crate::TransferKind::Ethless(default())));
		assert!(Blockchain::Rinkeby.supports(&crate::TransferKind::Ethless(default())));
		assert!(Blockchain::Luniverse.supports(&crate::TransferKind::Ethless(default())));
	}

	#[test]
	fn blockchain_unsupported() {
		assert!(!Blockchain::Other(default()).supports(&crate::TransferKind::Native));
		assert!(!Blockchain::Other(default()).supports(&crate::TransferKind::Erc20(default())));
		assert!(!Blockchain::Other(default()).supports(&crate::TransferKind::Ethless(default())));
		assert!(!Blockchain::Other(default()).supports(&crate::TransferKind::Other(default())));

		assert!(!Blockchain::Ethereum.supports(&crate::TransferKind::Other(default())));
		assert!(!Blockchain::Rinkeby.supports(&crate::TransferKind::Other(default())));
		assert!(!Blockchain::Luniverse.supports(&crate::TransferKind::Other(default())));
		assert!(!Blockchain::Bitcoin.supports(&crate::TransferKind::Other(default())));

		assert!(!Blockchain::Bitcoin.supports(&crate::TransferKind::Erc20(default())));
		assert!(!Blockchain::Bitcoin.supports(&crate::TransferKind::Ethless(default())));
	}

	#[test]
	fn blockchain_supports_bitcoin_native_transfer() {
		assert!(Blockchain::Bitcoin.supports(&crate::TransferKind::Native));
	}

	#[test]
	fn offchain_signed_tx_works() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let acct = AccountId::from(acct_pubkey.into_account().0);
		let transfer_id = crate::TransferId::new::<crate::mock::Test>(&Blockchain::Ethereum, &[0]);
		ext.build_offchain_and_execute_with_state(|_state, pool| {
			crate::mock::roll_to(1);
			let call = crate::Call::<crate::mock::Test>::fail_transfer {
				transfer_id,
				cause: IncorrectAmount,
			};
			assert_ok!(crate::Pallet::<crate::mock::Test>::offchain_signed_tx(
				acct.clone(),
				|_| call.clone(),
			));
			crate::mock::roll_to(2);

			assert_matches!(pool.write().transactions.pop(), Some(tx) => {
				let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
				assert_eq!(tx.call, crate::mock::Call::Creditcoin(call));
			});
		});
	}

	type MockTransfer = crate::Transfer<
		crate::mock::AccountId,
		crate::mock::BlockNumber,
		<TestRuntime as frame_system::Config>::Hash,
		u64,
	>;
	type MockUnverifiedTransfer = crate::UnverifiedTransfer<
		crate::mock::AccountId,
		crate::mock::BlockNumber,
		<TestRuntime as frame_system::Config>::Hash,
		u64,
	>;

	fn make_unverified_transfer(transfer: MockTransfer) -> MockUnverifiedTransfer {
		MockUnverifiedTransfer {
			transfer,
			to_external: ExternalAddress::try_from(ETHLESS_TO_ADDR.clone().0.to_vec()).unwrap(),
			from_external: ExternalAddress::try_from(ETHLESS_FROM_ADDR.clone().0.to_vec()).unwrap(),
		}
	}

	#[extend::ext]
	impl H160 {
		fn to_external_address(&self) -> ExternalAddress {
			ExternalAddress::try_from(self.0.clone().to_vec()).unwrap()
		}
	}

	#[test]
	fn verify_transfer_ocw_fails_on_unsupported_method() {
		ExtBuilder::default().build_offchain_and_execute(|| {
			crate::mock::roll_to(1);
			let test_info = TestInfo::new_defaults();
			let (deal_order, deal_order_id) = test_info.create_deal_order();
			let (mut transfer, _transfer_id) = test_info.make_transfer(
				&test_info.lender,
				&test_info.borrower,
				deal_order.terms.amount,
				&deal_order_id,
				"0xfafafa",
				crate::TransferKind::Native,
			);
			let unverified = make_unverified_transfer(transfer.clone());
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Failure(UnsupportedMethod))
			);

			transfer.kind = crate::TransferKind::Erc20(ExternalAddress::default());
			let unverified = make_unverified_transfer(transfer.clone());
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Failure(UnsupportedMethod))
			);

			transfer.kind = crate::TransferKind::Other(ExternalAddress::default());
			let unverified = make_unverified_transfer(transfer.clone());
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Failure(UnsupportedMethod))
			);
		});
	}

	#[test]
	fn verify_transfer_ocw_returns_err() {
		ExtBuilder::default().build_offchain_and_execute(|| {
			crate::mock::roll_to(1);
			let test_info = TestInfo::new_defaults();
			let (deal_order, deal_order_id) = test_info.create_deal_order();
			let (transfer, _) = test_info.make_transfer(
				&test_info.lender,
				&test_info.borrower,
				deal_order.terms.amount,
				&deal_order_id,
				"0xfafafa",
				crate::TransferKind::Ethless(ETHLESS_CONTRACT_ADDR.to_external_address()),
			);
			let unverified = make_unverified_transfer(transfer);

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Err(OffchainError::NoRpcUrl(_))
			);
		});
	}

	fn set_up_verify_transfer_env() -> (MockUnverifiedTransfer, MockedRpcRequests) {
		let rpc_uri = "http://localhost:8545";
		set_rpc_uri(&Blockchain::Rinkeby, rpc_uri);

		let test_info = TestInfo {
			loan_terms: LoanTerms { amount: get_mock_amount(), ..Default::default() },
			..TestInfo::new_defaults()
		};
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		let deal_id_hash = H256::from_uint(&get_mock_nonce());
		let deal_order_id = crate::DealOrderId::with_expiration_hash::<TestRuntime>(
			deal_order_id.expiration(),
			deal_id_hash,
		);
		let (transfer, _) = test_info.make_transfer(
			&test_info.lender,
			&test_info.borrower,
			deal_order.terms.amount,
			&deal_order_id,
			crate::mock::get_mock_tx_hash(),
			crate::TransferKind::Ethless(ETHLESS_CONTRACT_ADDR.to_external_address()),
		);
		let unverified = make_unverified_transfer(transfer);

		(
			unverified,
			MockedRpcRequests::new(
				Some(rpc_uri),
				&crate::mock::get_mock_tx_hash(),
				&crate::mock::get_mock_tx_block_num(),
			),
		)
	}

	#[test]
	fn verify_transfer_ocw_works() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (unverified, requests) = set_up_verify_transfer_env();

			requests.mock_all(&mut state.write());

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Success { timestamp: Some(_) })
			);
		});
	}

	#[test]
	fn verify_transfer_get_transaction_error() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (unverified, mut requests) = set_up_verify_transfer_env();

			requests.get_transaction.as_mut().unwrap().response = Some(
				serde_json::to_vec(&JsonRpcResponse::<bool> {
					jsonrpc: "2.0".into(),
					id: 1,
					error: None,
					result: None,
				})
				.unwrap(),
			);

			requests.mock_get_transaction(&mut state.write());

			// should this be a VerificationResult::Failure ?
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Err(OffchainError::RpcError(RpcError::NoResult))
			);
		});
	}

	#[test]
	fn verify_transfer_get_transaction_receipt_error() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (unverified, mut requests) = set_up_verify_transfer_env();

			requests.get_transaction_receipt.as_mut().unwrap().response = Some(
				serde_json::to_vec(&JsonRpcResponse::<bool> {
					jsonrpc: "2.0".into(),
					id: 1,
					error: None,
					result: None,
				})
				.unwrap(),
			);

			requests.mock_get_transaction_receipt(&mut state.write());

			// should this be a VerificationResult::Failure ?
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Err(OffchainError::RpcError(RpcError::NoResult))
			);
		});
	}

	#[test]
	fn verify_transfer_get_block_number_error() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (unverified, mut requests) = set_up_verify_transfer_env();

			requests.get_block_number.as_mut().unwrap().response = Some(
				serde_json::to_vec(&JsonRpcResponse::<bool> {
					jsonrpc: "2.0".into(),
					id: 1,
					error: None,
					result: None,
				})
				.unwrap(),
			);

			requests.mock_get_block_number(&mut state.write());

			// should this be a VerificationResult::Failure ?
			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Err(OffchainError::RpcError(RpcError::NoResult))
			);
		});
	}

	#[test]
	fn verify_transfer_get_block_by_number_error() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (unverified, mut requests) = set_up_verify_transfer_env();

			requests.get_block_by_number.as_mut().unwrap().response = Some(
				serde_json::to_vec(&JsonRpcResponse::<bool> {
					jsonrpc: "2.0".into(),
					id: 1,
					error: None,
					result: None,
				})
				.unwrap(),
			);

			requests.mock_all(&mut state.write());

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Success { timestamp: None })
			);
		});
	}

	#[test]
	fn verify_transfer_get_block_invalid_address() {
		fn mock_requests(state: &Arc<RwLock<OffchainState>>) {
			MockedRpcRequests::new(
				Some("http://localhost:8545"),
				&crate::mock::get_mock_tx_hash(),
				&crate::mock::get_mock_tx_block_num(),
			)
			.mock_get_block_number(&mut state.write());
		}
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
			crate::mock::roll_to(1);
			let (mut unverified, ..) = set_up_verify_transfer_env();

			mock_requests(&state);

			let bad_from_unverified =
				MockUnverifiedTransfer { from_external: default(), ..unverified.clone() };

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&bad_from_unverified),
				Ok(VerificationResult::Failure(InvalidAddress))
			);

			mock_requests(&state);

			let bad_to_unverified =
				MockUnverifiedTransfer { to_external: default(), ..unverified.clone() };

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&bad_to_unverified),
				Ok(VerificationResult::Failure(InvalidAddress))
			);

			mock_requests(&state);

			unverified.transfer.kind = TransferKind::Ethless(default());

			assert_matches!(
				crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
				Ok(VerificationResult::Failure(InvalidAddress))
			);
		});
	}
}
