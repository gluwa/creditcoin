use core::fmt::Debug;
use std::{convert::TryFrom, str::FromStr};

use super::errors::{
	RpcUrlError,
	VerificationFailureCause::{self, *},
};

use super::tasks::collect_coins::tests::{mock_rpc_for_collect_coins, RPC_RESPONSE_AMOUNT};
use super::tasks::BlockAndTime;
use super::tasks::Lockable;
use super::{
	errors::OffchainError,
	parse_eth_address,
	rpc::{Address, EthTransaction, EthTransactionReceipt},
	tasks::verify_transfer::ethless_transfer_function_abi,
	tasks::verify_transfer::validate_ethless_transfer,
	ETH_CONFIRMATIONS,
};
use crate::ocw::tasks::collect_coins::{testing_constants::CHAIN, tests::TX_HASH};
use crate::tests::adjust_deal_order_to_nonce;
use crate::tests::generate_address_with_proof;
use crate::types::{AddressId, CollectedCoins, CollectedCoinsId, TaskId};
use crate::Pallet as Creditcoin;
use crate::{
	helpers::HexToAddress,
	mock::{
		get_mock_amount, get_mock_contract, get_mock_from_address, get_mock_input_data,
		get_mock_nonce, get_mock_timestamp, get_mock_to_address, get_mock_tx_block_num,
		get_mock_tx_hash, roll_by_with_ocw, roll_to, roll_to_with_ocw, set_rpc_uri, AccountId,
		Call, ExtBuilder, Extrinsic, MockedRpcRequests, Origin, PendingRequestExt, RwLock, Test,
		ETHLESS_RESPONSES,
	},
	ocw::rpc::{errors::RpcError, JsonRpcError, JsonRpcResponse},
	ocw::tasks::StorageLock,
	tests::TestInfo,
	types::{DoubleMapExt, TransferId},
	Blockchain, CurrencyOrLegacyTransferKind, ExternalAddress, Id, LegacyTransferKind, LoanTerms,
	TransferKind, Transfers,
};
use alloc::sync::Arc;
use assert_matches::assert_matches;
use ethabi::Token;
use ethereum_types::{BigEndianHash, H160, U256, U64};
use frame_support::{assert_ok, once_cell::sync::Lazy, BoundedVec};
use frame_system::Pallet as System;
use parity_scale_codec::Decode;
use sp_core::H256;
use sp_io::offchain;
use sp_runtime::offchain::storage::MutateStorageError;
use sp_runtime::offchain::testing::TestOffchainExt;
use sp_runtime::traits::Dispatchable;
use sp_runtime::{
	offchain::{
		storage::{StorageRetrievalError, StorageValueRef},
		testing::OffchainState,
		Duration,
	},
	traits::IdentifyAccount,
};

fn make_external_address(hex_str: &str) -> ExternalAddress {
	BoundedVec::try_from(hex::decode(hex_str.trim_start_matches("0x")).unwrap()).unwrap()
}

#[track_caller]
fn assert_invalid_task<T: Debug>(
	result: Result<T, OffchainError>,
	cause: VerificationFailureCause,
) {
	assert_matches!(result, Err(OffchainError::InvalidTask(why)) => { assert_eq!(why, cause); } );
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
			from: *ETHLESS_FROM_ADDR,
			to: *ETHLESS_TO_ADDR,
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

	assert_invalid_task(parse_eth_address(&too_long), InvalidAddress);
	assert_invalid_task(parse_eth_address(&too_short), InvalidAddress);
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

static ETH_TRANSACTION: Lazy<EthTransaction> = Lazy::new(|| {
	let mut transaction = EthTransaction::default();
	transaction.block_number = Some(5u64.into());
	transaction.from = Some(*ETHLESS_FROM_ADDR);
	transaction.to = Some(*ETHLESS_CONTRACT_ADDR);
	transaction.set_input(&hex::decode(&*ETHLESS_INPUT).unwrap());
	transaction
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
			from: *ETHLESS_FROM_ADDR,
			to: *ETHLESS_TO_ADDR,
			contract: *ETHLESS_CONTRACT_ADDR,
			amount: get_mock_amount(),
			receipt: EthTransactionReceipt { status: Some(1u64.into()), ..Default::default() },
			transaction: ETH_TRANSACTION.clone(),
			tip: (ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS),
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
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			receipt: EthTransactionReceipt { status: Some(0u64.into()), ..Default::default() },
			..Default::default()
		}),
		TaskFailed,
	);
}

#[test]
fn ethless_transfer_tx_unconfirmed() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			tip: (ETH_TRANSACTION.block_number.unwrap() + ETH_CONFIRMATIONS / 2),
			..Default::default()
		}),
		TaskUnconfirmed,
	);
}

#[test]
fn ethless_transfer_tx_missing_to() {
	let mut transaction = ETH_TRANSACTION.clone();
	transaction.to = None;
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
		MissingReceiver,
	);
}

#[test]
fn ethless_transfer_tx_ahead_of_tip() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			tip: (ETH_TRANSACTION.block_number.unwrap() - 1),
			..Default::default()
		}),
		TaskInFuture,
	);
}

#[test]
fn ethless_transfer_contract_mismatch() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			contract: Address::from_str("0xbad1439a0e0bfdcd49939f9722866651a4aa9b3c").unwrap(),
			..Default::default()
		}),
		IncorrectContract,
	);
}

#[test]
fn ethless_transfer_from_mismatch() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			from: Address::from_str("0xbad349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").unwrap(),
			..Default::default()
		}),
		IncorrectSender,
	);
}

#[test]
fn ethless_transfer_to_mismatch() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			to: Address::from_str("0xbad8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").unwrap(),
			..Default::default()
		}),
		IncorrectReceiver,
	);
}

#[test]
fn ethless_transfer_invalid_input_data() {
	let mut transaction = ETH_TRANSACTION.clone();
	transaction.set_input("badbad".as_bytes());
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
		AbiMismatch,
	);
}

#[test]
fn ethless_transfer_amount_mismatch() {
	assert_invalid_task(
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
	let input = transfer.encode_input(&input_args.into_tokens()).unwrap();
	let mut transaction = ETH_TRANSACTION.clone();
	transaction.set_input(&input);
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
		IncorrectNonce,
	);
}

#[test]
fn ethless_transfer_pending() {
	let mut transaction = ETH_TRANSACTION.clone();
	transaction.block_number = None;
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
		TaskPending,
	)
}

#[test]
fn blockchain_rpc_url_missing() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		assert_eq!(Blockchain::ETHEREUM.rpc_url(), Err(RpcUrlError::NoValue));
	})
}

#[test]
fn blockchain_rpc_url_non_utf8() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		set_rpc_uri(&Blockchain::ETHEREUM, &[0x80]);

		assert_matches!(Blockchain::ETHEREUM.rpc_url().unwrap_err(), RpcUrlError::InvalidUrl(_));
	});
}

#[test]
fn blockchain_rpc_url_works() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		set_rpc_uri(&Blockchain::ETHEREUM, "rpcurl");

		assert_eq!(Blockchain::ETHEREUM.rpc_url().unwrap(), "rpcurl");
	})
}

#[test]
fn blockchain_rpc_url_invalid_scale() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		let eth = Blockchain::ETHEREUM;
		let key = eth.rpc_key();
		let rpc_url_storage = StorageValueRef::persistent(&key);
		rpc_url_storage.set(&[0x80]);

		assert_matches!(
			eth.rpc_url().unwrap_err(),
			RpcUrlError::StorageFailure(StorageRetrievalError::Undecodable)
		);
	});
}

#[test]
fn blockchain_supports_etherlike() {
	assert!(Blockchain::ETHEREUM.supports(&crate::LegacyTransferKind::Native));
	assert!(Blockchain::RINKEBY.supports(&crate::LegacyTransferKind::Native));
	assert!(Blockchain::LUNIVERSE.supports(&crate::LegacyTransferKind::Native));
	assert!(Blockchain::ETHEREUM.supports(&crate::LegacyTransferKind::Erc20(default())));
	assert!(Blockchain::RINKEBY.supports(&crate::LegacyTransferKind::Erc20(default())));
	assert!(Blockchain::LUNIVERSE.supports(&crate::LegacyTransferKind::Erc20(default())));
	assert!(Blockchain::ETHEREUM.supports(&crate::LegacyTransferKind::Ethless(default())));
	assert!(Blockchain::RINKEBY.supports(&crate::LegacyTransferKind::Ethless(default())));
	assert!(Blockchain::LUNIVERSE.supports(&crate::LegacyTransferKind::Ethless(default())));
}

#[test]
fn blockchain_unsupported() {
	assert!(!Blockchain::ETHEREUM.supports(&crate::LegacyTransferKind::Other(default())));
	assert!(!Blockchain::RINKEBY.supports(&crate::LegacyTransferKind::Other(default())));
	assert!(!Blockchain::LUNIVERSE.supports(&crate::LegacyTransferKind::Other(default())));
}

#[test]
fn offchain_signed_tx_works() {
	let mut ext = ExtBuilder::default();
	let acct_pubkey = ext.generate_authority();
	let acct = AccountId::from(acct_pubkey.into_account().0);
	let transfer_id = crate::TransferId::new::<crate::mock::Test>(&Blockchain::ETHEREUM, &[0]);
	ext.build_offchain_and_execute_with_state(|_state, pool| {
		crate::mock::roll_to(1);
		let call = crate::Call::<crate::mock::Test>::fail_task {
			task_id: transfer_id.into(),
			deadline: 10000,
			cause: IncorrectAmount,
		};
		assert_ok!(
			crate::Pallet::<crate::mock::Test>::offchain_signed_tx(acct.clone(), |_| call.clone(),)
		);
		crate::mock::roll_to(2);

		assert_matches!(pool.write().transactions.pop(), Some(tx) => {
			let tx = Extrinsic::decode(&mut &*tx).unwrap();
			assert_eq!(tx.call, crate::mock::Call::Creditcoin(call));
		});
	});
}

type MockTransfer = crate::Transfer<
	crate::mock::AccountId,
	crate::mock::BlockNumber,
	<Test as frame_system::Config>::Hash,
	u64,
>;
type MockUnverifiedTransfer = crate::UnverifiedTransfer<
	crate::mock::AccountId,
	crate::mock::BlockNumber,
	<Test as frame_system::Config>::Hash,
	u64,
>;

fn make_unverified_transfer(transfer: MockTransfer) -> MockUnverifiedTransfer {
	MockUnverifiedTransfer {
		transfer,
		to_external: ExternalAddress::try_from(ETHLESS_TO_ADDR.0.to_vec()).unwrap(),
		from_external: ExternalAddress::try_from(ETHLESS_FROM_ADDR.0.to_vec()).unwrap(),
		deadline: 10000,
		currency_to_check: crate::CurrencyOrLegacyTransferKind::TransferKind(
			LegacyTransferKind::Ethless(
				ExternalAddress::try_from(ETHLESS_CONTRACT_ADDR.0.to_vec()).unwrap(),
			),
		),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let (_, transfer) = test_info.make_transfer(
			&test_info.lender,
			&test_info.borrower,
			deal_order.terms.amount,
			&deal_order_id,
			"0xfafafa",
			None::<TransferKind>,
		);
		let mut unverified = make_unverified_transfer(transfer.clone());
		unverified.currency_to_check =
			crate::CurrencyOrLegacyTransferKind::TransferKind(crate::LegacyTransferKind::Native);
		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(UnsupportedMethod))
		);

		let mut unverified = make_unverified_transfer(transfer.clone());
		unverified.currency_to_check = crate::CurrencyOrLegacyTransferKind::TransferKind(
			LegacyTransferKind::Erc20(ExternalAddress::default()),
		);
		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(UnsupportedMethod))
		);

		let mut unverified = make_unverified_transfer(transfer);
		unverified.currency_to_check = crate::CurrencyOrLegacyTransferKind::TransferKind(
			LegacyTransferKind::Other(ExternalAddress::default()),
		);
		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(UnsupportedMethod))
		);
	});
}

#[test]
fn verify_transfer_ocw_returns_err() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		crate::mock::roll_to(1);
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let (_, transfer) = test_info.make_transfer(
			&test_info.lender,
			&test_info.borrower,
			deal_order.terms.amount,
			&deal_order_id,
			"0xfafafa",
			None::<TransferKind>,
		);
		let unverified = make_unverified_transfer(transfer);

		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::NoRpcUrl(_))
		);
	});
}

#[test]
#[tracing_test::traced_test]
fn offchain_worker_logs_error_when_transfer_validation_errors() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	ext.build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);

		let (_unverified, mut requests) = set_up_verify_transfer_env(true);

		requests.get_transaction.as_mut().unwrap().response = Some(
			serde_json::to_vec(&JsonRpcResponse::<bool> {
				jsonrpc: "2.0".into(),
				id: 1,
				error: Some(JsonRpcError { code: 555, message: "this is supposed to fail".into() }),
				result: None,
			})
			.unwrap(),
		);

		requests.mock_chain_id(&mut state.write());

		requests.mock_get_transaction(&mut state.write());

		crate::mock::roll_by_with_ocw(1);
		assert!(logs_contain("Task verification encountered an error"));
	});
}

#[test]
#[tracing_test::traced_test]
fn offchain_worker_should_log_and_forget_guard_when_task_is_already_handled() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	ext.build_offchain_and_execute(|| {
		crate::mock::roll_to(1);

		let (unverified, _) = set_up_verify_transfer_env(true);
		let id =
			TransferId::new::<Test>(&unverified.transfer.blockchain, &unverified.transfer.tx_id);
		// simulate a transfer that has already been handled
		Transfers::<Test>::insert(&id, &unverified.transfer);

		crate::mock::roll_by_with_ocw(1);
		assert!(logs_contain("Already handled Task"));

		// check that guard for the same ID has been released
		let storage_key = crate::ocw::tasks::storage_key(&TaskId::VerifyTransfer(id));
		let mut lock = StorageLock::<'_, BlockAndTime<System<Test>>>::with_block_and_time_deadline(
			&storage_key,
			1,
			Duration::from_millis(0),
		);

		let guard = lock.try_lock();
		assert!(guard.is_err());
	});
}

fn set_up_verify_transfer_env(
	register_transfer: bool,
) -> (MockUnverifiedTransfer, MockedRpcRequests) {
	let rpc_uri = "http://localhost:8545";
	set_rpc_uri(&Blockchain::RINKEBY, rpc_uri);

	let test_info = TestInfo {
		loan_terms: LoanTerms { amount: get_mock_amount(), ..Default::default() },
		..TestInfo::new_defaults()
	};
	let (deal_order_id, deal_order) = test_info.create_deal_order();

	let deal_id_hash = H256::from_uint(&get_mock_nonce());
	let deal_order_id =
		crate::DealOrderId::with_expiration_hash::<Test>(deal_order_id.expiration(), deal_id_hash);
	let (_, transfer) = test_info.make_transfer(
		&test_info.lender,
		&test_info.borrower,
		deal_order.terms.amount,
		&deal_order_id,
		crate::mock::get_mock_tx_hash(),
		Some(crate::EvmTransferKind::Ethless),
	);
	let unverified = make_unverified_transfer(transfer.clone());

	if register_transfer {
		crate::DealOrders::<Test>::insert_id(deal_order_id.clone(), deal_order);

		assert_ok!(crate::mock::Creditcoin::register_funding_transfer(
			crate::mock::Origin::signed(test_info.lender.account_id),
			crate::EvmTransferKind::Ethless.into(),
			deal_order_id,
			transfer.tx_id,
		));
	}

	(
		unverified,
		MockedRpcRequests::new(
			Some(rpc_uri),
			&crate::mock::get_mock_tx_hash(),
			&crate::mock::get_mock_tx_block_num(),
			&ETHLESS_RESPONSES,
		),
	)
}

#[test]
fn verify_transfer_ocw_works() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, requests) = set_up_verify_transfer_env(false);

		requests.mock_all(&mut state.write());

		assert_matches!(crate::Pallet::<Test>::verify_transfer_ocw(&unverified), Ok(Some(_)));
	});
}

#[test]
fn verify_transfer_get_transaction_error() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, mut requests) = set_up_verify_transfer_env(false);
		requests.get_transaction.set_empty_response();

		requests.mock_get_transaction(&mut state.write());

		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(TransactionNotFound))
		);
	});
}

#[test]
fn verify_transfer_get_transaction_receipt_error() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, mut requests) = set_up_verify_transfer_env(false);

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
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::RpcError(RpcError::NoResult))
		);
	});
}

#[test]
fn verify_transfer_get_block_number_error() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, mut requests) = set_up_verify_transfer_env(false);

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
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::RpcError(RpcError::NoResult))
		);
	});
}

#[test]
fn verify_transfer_get_block_by_number_error() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, mut requests) = set_up_verify_transfer_env(false);

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

		assert_matches!(crate::Pallet::<Test>::verify_transfer_ocw(&unverified), Ok(None));
	});
}

#[test]
fn verify_transfer_get_block_invalid_address() {
	fn mock_requests(state: &Arc<RwLock<OffchainState>>) {
		MockedRpcRequests::new(
			Some("http://localhost:8545"),
			&crate::mock::get_mock_tx_hash(),
			&crate::mock::get_mock_tx_block_num(),
			&ETHLESS_RESPONSES,
		)
		.mock_get_block_number(&mut state.write());
	}
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (mut unverified, ..) = set_up_verify_transfer_env(false);

		mock_requests(&state);

		let bad_from_unverified =
			MockUnverifiedTransfer { from_external: default(), ..unverified.clone() };

		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&bad_from_unverified),
			Err(OffchainError::InvalidTask(InvalidAddress))
		);

		mock_requests(&state);

		let bad_to_unverified =
			MockUnverifiedTransfer { to_external: default(), ..unverified.clone() };

		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&bad_to_unverified),
			Err(OffchainError::InvalidTask(InvalidAddress))
		);

		mock_requests(&state);

		unverified.currency_to_check =
			CurrencyOrLegacyTransferKind::TransferKind(LegacyTransferKind::Ethless(default()));

		assert_matches!(
			crate::Pallet::<Test>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(InvalidAddress))
		);
	});
}

#[test]
fn completed_oversubscribed_tasks_are_skipped() {
	let mut ext = ExtBuilder::default();
	let acct_pubkey = ext.generate_authority();
	let auth = AccountId::from(acct_pubkey.into_account().0);
	ext.build_offchain_and_execute_with_state(|state, pool| {
		mock_rpc_for_collect_coins(&state);

		let (acc, addr, sign, _) = generate_address_with_proof("collector");

		assert_ok!(Creditcoin::<Test>::register_address(
			Origin::signed(acc.clone()),
			CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline = Test::unverified_transfer_deadline();
		//register twice (oversubscribe) under different expiration (aka deadline).
		assert_ok!(Creditcoin::<Test>::request_collect_coins(
			Origin::signed(acc.clone()),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to(2);
		let deadline_2 = Test::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<Test>::request_collect_coins(
			Origin::signed(acc),
			addr.clone(),
			TX_HASH.hex_to_address()
		));

		//We now have 2 enqueued tasks.

		roll_to_with_ocw(3);

		let collected_coins_id =
			CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());
		let collected_coins = CollectedCoins {
			to: AddressId::new::<Test>(&CHAIN, addr.as_ref()),
			amount: RPC_RESPONSE_AMOUNT.as_u128(),
			tx_id: TX_HASH.hex_to_address(),
		};

		let tx = pool.write().transactions.pop().expect("persist collect_coins");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			tx.call,
			Call::Creditcoin(crate::Call::persist_task_output {
				task_output: (collected_coins_id.clone(), collected_coins).into(),
				deadline
			})
		);

		assert_ok!(tx.call.dispatch(Origin::signed(auth)));

		roll_to_with_ocw(deadline_2);

		//task expires without yielding txns.
		assert!(pool.read().transactions.is_empty());

		let key = super::tasks::storage_key(&TaskId::from(collected_coins_id));

		type Y = <BlockAndTime<System<Test>> as Lockable>::Deadline;
		//lock set
		assert!(StorageValueRef::persistent(key.as_ref()).get::<Y>().expect("decoded").is_some());
	});
}

//tasks can be oversubscribed with different deadlines
#[test]
fn task_deadline_oversubscription() {
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|_, _| {
		let (acc, addr, sign, _) = generate_address_with_proof("collector");

		assert_ok!(Creditcoin::<Test>::register_address(
			Origin::signed(acc.clone()),
			CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline_1 = Test::unverified_transfer_deadline();
		//register twice under different (expiration aka deadline)
		assert_ok!(Creditcoin::<Test>::request_collect_coins(
			Origin::signed(acc.clone()),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to(2);
		let deadline_2 = Test::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<Test>::request_collect_coins(
			Origin::signed(acc),
			addr,
			TX_HASH.hex_to_address()
		));

		let collected_coins_id =
			CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());

		assert!(Creditcoin::<Test>::pending_tasks(
			deadline_1,
			TaskId::from(collected_coins_id.clone())
		)
		.is_some());
		assert!(Creditcoin::<Test>::pending_tasks(deadline_2, TaskId::from(collected_coins_id))
			.is_some());
	});
}

#[test]
#[tracing_test::traced_test]
fn ocw_retries() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	ext.build_offchain_and_execute_with_state(|state, pool| {
		roll_to(1);

		let dummy_url = "dummy";
		let tx_hash = get_mock_tx_hash();
		let contract = get_mock_contract().hex_to_address();
		let tx_block_num = get_mock_tx_block_num();
		let blockchain = Blockchain::RINKEBY;

		let tx_block_num_value =
			u64::from_str_radix(tx_block_num.trim_start_matches("0x"), 16).unwrap();

		set_rpc_uri(&Blockchain::RINKEBY, &dummy_url);

		let loan_amount = get_mock_amount();
		let terms = LoanTerms { amount: loan_amount, ..Default::default() };

		let test_info = TestInfo { blockchain, loan_terms: terms, ..Default::default() };

		let (deal_order_id, _) = test_info.create_deal_order();

		let deal_order_id = adjust_deal_order_to_nonce(&deal_order_id, get_mock_nonce());

		let lender = test_info.lender.account_id;
		assert_ok!(Creditcoin::<Test>::register_funding_transfer_legacy(
			Origin::signed(lender),
			LegacyTransferKind::Ethless(contract),
			deal_order_id,
			tx_hash.hex_to_address(),
		));

		let mock_unconfirmed_tx = || {
			let mut requests =
				MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &ETHLESS_RESPONSES);
			requests.get_block_number.set_response(JsonRpcResponse {
				jsonrpc: "2.0".into(),
				id: 1,
				error: None,
				result: Some(format!("0x{:x}", tx_block_num_value + 1)),
			});

			requests.mock_get_block_number(&mut state.write());
		};

		// mock requests so the tx is unconfirmed
		mock_unconfirmed_tx();

		roll_by_with_ocw(1);
		assert!(logs_contain("TaskUnconfirmed"));

		// we failed, we should retry again here

		mock_unconfirmed_tx();

		roll_by_with_ocw(1);
		assert!(logs_contain("TaskUnconfirmed"));

		// now mock requests so the tx is confirmed

		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &ETHLESS_RESPONSES)
			.mock_all(&mut state.write());

		roll_by_with_ocw(1);

		// we should have retried and successfully verified the transfer
		let tx = pool.write().transactions.pop().expect("verify transfer");
		assert!(pool.read().transactions.is_empty());
		let verify_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_matches!(
			verify_tx.call,
			crate::mock::Call::Creditcoin(crate::Call::persist_task_output { .. })
		);
	});
}

#[test]
fn duplicate_retry_fail_and_succeed() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	ext.build_offchain_and_execute_with_state(|state, pool| {
		let dummy_url = "dummy";
		let tx_hash = get_mock_tx_hash();
		let contract = get_mock_contract().hex_to_address();
		let tx_block_num = get_mock_tx_block_num();
		let blockchain = Blockchain::RINKEBY;

		// mocks for when we expect failure
		{
			let mut state = state.write();
			MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &ETHLESS_RESPONSES)
				.mock_chain_id(&mut state)
				.mock_get_block_number(&mut state);
		}
		// mocks for when we expect success
		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &ETHLESS_RESPONSES)
			.mock_all(&mut state.write());

		set_rpc_uri(&Blockchain::RINKEBY, &dummy_url);

		let loan_amount = get_mock_amount();
		let currency = crate::tests::ethless_currency(contract.clone());

		let test_info = TestInfo::with_currency(currency);
		let test_info = TestInfo {
			loan_terms: LoanTerms { amount: loan_amount, ..test_info.loan_terms },
			..test_info
		};
		let (deal_order_id, _) = test_info.create_deal_order();
		let lender = test_info.lender.account_id.clone();

		// test that we get a "fail_transfer" tx when verification fails
		assert_ok!(Creditcoin::<Test>::register_funding_transfer(
			Origin::signed(lender.clone()),
			crate::EvmTransferKind::Ethless.into(),
			deal_order_id.clone(),
			tx_hash.hex_to_address(),
		));
		let deadline = Test::unverified_transfer_deadline();

		roll_to_with_ocw(1);

		let transfer_id = TransferId::new::<Test>(&blockchain, &tx_hash.hex_to_address());
		let tx = pool.write().transactions.pop().expect("fail transfer");
		assert!(pool.read().transactions.is_empty());
		let fail_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			fail_tx.call,
			Call::Creditcoin(crate::Call::fail_task {
				task_id: transfer_id.clone().into(),
				deadline,
				cause: VerificationFailureCause::IncorrectNonce
			})
		);

		// test for successful verification

		// this is kind of a gross hack, basically when I made the test transfer on luniverse to pull the mock responses
		// I didn't pass the proper `nonce` to the smart contract, and it's a pain to redo the transaction and update all the tests,
		// so here we just "change" the deal_order_id to one with a `hash` that matches the expected nonce so that the transfer
		// verification logic is happy
		let fake_deal_order_id = adjust_deal_order_to_nonce(&deal_order_id, get_mock_nonce());

		assert_ok!(Creditcoin::<Test>::register_funding_transfer_legacy(
			Origin::signed(lender.clone()),
			LegacyTransferKind::Ethless(contract),
			fake_deal_order_id.clone(),
			tx_hash.hex_to_address(),
		));

		let deadline_2 = Test::unverified_transfer_deadline();

		let expected_transfer = crate::Transfer {
			blockchain: test_info.blockchain.clone(),
			kind: TransferKind::Evm(crate::EvmTransferKind::Ethless),
			amount: loan_amount,
			block: System::<Test>::block_number(),
			from: test_info.lender.address_id.clone(),
			to: test_info.borrower.address_id,
			deal_order_id: fake_deal_order_id,
			is_processed: false,
			account_id: lender,
			tx_id: tx_hash.hex_to_address(),
			timestamp: Some(get_mock_timestamp()),
		};

		//We expect the guard to expire on the next roll, sleep to meet time requirements.
		let lock_expires = offchain::timestamp().add(Duration::from_millis(1));
		offchain::sleep_until(lock_expires);

		//guard picked at 1 is reacquireable at...
		roll_to_with_ocw(deadline);

		let tx = pool.write().transactions.pop().expect("verify transfer");
		assert!(pool.read().transactions.is_empty());
		let verify_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			verify_tx.call,
			Call::Creditcoin(crate::Call::persist_task_output {
				task_output: (transfer_id, expected_transfer).into(),
				deadline: deadline_2
			})
		);
	});
}

#[test]
fn effective_guard_lifetime_until_task_expiration() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	ext.build_offchain_and_execute_with_state(|state, pool| {
		mock_rpc_for_collect_coins(&state);

		let (acc, addr, sign, _) = generate_address_with_proof("collector");
		assert_ok!(Creditcoin::<Test>::register_address(
			Origin::signed(acc.clone()),
			CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline = Test::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<Test>::request_collect_coins(
			Origin::signed(acc),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to_with_ocw(2);

		let tx = pool.write().transactions.pop().expect("persist collect_coins");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();

		let collected_coins_id =
			CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());
		let collected_coins = CollectedCoins {
			to: AddressId::new::<Test>(&CHAIN, addr.as_ref()),
			amount: RPC_RESPONSE_AMOUNT.as_u128(),
			tx_id: TX_HASH.hex_to_address(),
		};

		assert_eq!(
			tx.call,
			Call::Creditcoin(crate::Call::persist_task_output {
				task_output: (collected_coins_id, collected_coins).into(),
				deadline
			})
		);

		let key = {
			let collected_coins_id =
				CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());

			super::tasks::storage_key(&TaskId::from(collected_coins_id))
		};

		type Y = <BlockAndTime<System<Test>> as Lockable>::Deadline;
		//lock set
		let Y { block_number, .. } = StorageValueRef::persistent(key.as_ref())
			.get::<Y>()
			.expect("decoded")
			.expect("deadline");
		println!("{block_number} {deadline}");
		assert!(block_number >= deadline - 1);
	});
}

#[test]
fn parallel_worker_trivial() {
	let (offchain, _) = TestOffchainExt::new();
	const TRIES_PER_THREAD: u32 = 10_000;
	const THREADS: u32 = 2;
	const TOTAL: u32 = THREADS * TRIES_PER_THREAD;
	const STORAGE_KEY: &[u8] = b"demo_status";

	let handles: Vec<_> = (0..THREADS)
		.into_iter()
		.map(|_| {
			let offchain = offchain.clone();

			std::thread::spawn(move || {
				let ext_builder = ExtBuilder::default();
				let (mut ext, _) = ext_builder.build_with(offchain);
				let execute = || {
					let mut tries = 0;
					let a = StorageValueRef::persistent(STORAGE_KEY);
					while tries < TRIES_PER_THREAD {
						tries += 1;
						'spin: loop {
							let res = a.mutate::<u32, (), _>(|a: Result<Option<u32>, _>| {
								let v = if let Ok(a) = a { a } else { None };
								match v {
									Some(a) => Ok(a + 1),
									None => Ok(1),
								}
							});
							match res {
								Ok(_) => break 'spin,
								Err(MutateStorageError::ConcurrentModification(..)) => {
									continue 'spin
								},
								Err(MutateStorageError::ValueFunctionFailed(..)) => {
									unreachable!()
								},
							};
						}
					}
				};

				ext.execute_with(execute);
			})
		})
		.collect();

	for h in handles {
		h.join().expect("testing context is shared");
	}

	let ext_builder = ExtBuilder::default();
	let (mut ext, _) = ext_builder.build_with(offchain);
	ext.execute_with(|| {
		let val = StorageValueRef::persistent(STORAGE_KEY).get::<u32>().unwrap().unwrap();
		assert_eq!(val, TOTAL);
	});
}
