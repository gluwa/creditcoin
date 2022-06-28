use core::fmt::Debug;
use std::{convert::TryFrom, str::FromStr};

use super::errors::{
	RpcUrlError,
	VerificationFailureCause::{self, *},
};

use super::task::BlockAndTime;
use super::task::Task;
use super::Lockable;
use crate::ocw::collect_coins::{tests::TX_HASH, CONTRACT_CHAIN};
use crate::tests::generate_address_with_proof;
use crate::types::{AddressId, CollectedCoins, CollectedCoinsId, UnverifiedCollectedCoins};
use crate::Pallet as Creditcoin;
use crate::{
	mock::{
		get_mock_amount, get_mock_contract, get_mock_from_address, get_mock_input_data,
		get_mock_nonce, get_mock_timestamp, get_mock_to_address, roll_to, roll_to_with_ocw,
		set_rpc_uri, AccountId, Call, ExtBuilder, Extrinsic, MockedRpcRequests, Origin,
		PendingRequestExt, Test as TestRuntime, ETHLESS_RESPONSES,
	},
	ocw::rpc::{errors::RpcError, JsonRpcError, JsonRpcResponse},
	tests::{RefstrExt, TestInfo},
	types::{DoubleMapExt, TransferId},
	Blockchain, ExternalAddress, Id, LoanTerms, OrderId, TransferKind,
};
use alloc::sync::Arc;
use assert_matches::assert_matches;
use codec::Decode;
use ethabi::Token;
use ethereum_types::{BigEndianHash, H160, U256, U64};
use frame_support::{assert_ok, once_cell::sync::Lazy, BoundedVec};
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
use parking_lot::RwLock;
use sp_core::H256;
use sp_io::offchain;
use sp_runtime::{
	offchain::{
		storage::{StorageRetrievalError, StorageValueRef},
		testing::OffchainState,
		Duration,
	},
	traits::IdentifyAccount,
};

use super::{
	errors::OffchainError,
	ethless_transfer_function_abi, parse_eth_address,
	rpc::{Address, EthTransaction, EthTransactionReceipt},
	validate_ethless_transfer, ETH_CONFIRMATIONS,
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

static ETH_TRANSACTION: Lazy<EthTransaction> = Lazy::new(|| EthTransaction {
	block_number: Some(5u64.into()),
	from: Some(*ETHLESS_FROM_ADDR),
	to: Some(*ETHLESS_CONTRACT_ADDR),
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
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			transaction: EthTransaction { to: None, ..ETH_TRANSACTION.clone() },
			..Default::default()
		}),
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
	assert_invalid_task(
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
	let input = transfer.encode_input(&input_args.into_tokens()).unwrap().into();
	let transaction = EthTransaction { input, ..ETH_TRANSACTION.clone() };
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs { transaction, ..Default::default() }),
		IncorrectNonce,
	);
}

#[test]
fn ethless_transfer_pending() {
	assert_invalid_task(
		test_validate_ethless_transfer(EthlessTestArgs {
			transaction: EthTransaction { block_number: None, ..ETH_TRANSACTION.clone() },
			..Default::default()
		}),
		TaskPending,
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

		assert_matches!(Blockchain::Ethereum.rpc_url().unwrap_err(), RpcUrlError::InvalidUrl(_));
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
		to_external: ExternalAddress::try_from((*ETHLESS_TO_ADDR).0.to_vec()).unwrap(),
		from_external: ExternalAddress::try_from((*ETHLESS_FROM_ADDR).0.to_vec()).unwrap(),
		deadline: 10000,
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
			Err(OffchainError::InvalidTask(UnsupportedMethod))
		);

		transfer.kind = crate::TransferKind::Erc20(ExternalAddress::default());
		let unverified = make_unverified_transfer(transfer.clone());
		assert_matches!(
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(UnsupportedMethod))
		);

		transfer.kind = crate::TransferKind::Other(ExternalAddress::default());
		let unverified = make_unverified_transfer(transfer);
		assert_matches!(
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(UnsupportedMethod))
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

		requests.mock_get_transaction(&mut state.write());

		crate::mock::roll_by_with_ocw(1);
		assert!(logs_contain("Task verification encountered an error"));
	});
}

fn set_up_verify_transfer_env(
	register_transfer: bool,
) -> (MockUnverifiedTransfer, MockedRpcRequests) {
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
	let unverified = make_unverified_transfer(transfer.clone());

	if register_transfer {
		let contract = get_mock_contract().hex_to_address();

		crate::DealOrders::<TestRuntime>::insert_id(deal_order_id.clone(), deal_order);

		assert_ok!(crate::mock::Creditcoin::register_funding_transfer(
			crate::mock::Origin::signed(test_info.lender.account_id),
			TransferKind::Ethless(contract),
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
			&*ETHLESS_RESPONSES,
		),
	)
}

#[test]
fn verify_transfer_ocw_works() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _pool| {
		crate::mock::roll_to(1);
		let (unverified, requests) = set_up_verify_transfer_env(false);

		requests.mock_all(&mut state.write());

		assert_matches!(crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified), Ok(_));
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
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
			Err(OffchainError::InvalidTask(TaskNonexistent))
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
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
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
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
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

		assert_matches!(crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified), Ok(transfer) => {assert!(transfer.timestamp.is_none())});
	});
}

#[test]
fn verify_transfer_get_block_invalid_address() {
	fn mock_requests(state: &Arc<RwLock<OffchainState>>) {
		MockedRpcRequests::new(
			Some("http://localhost:8545"),
			&crate::mock::get_mock_tx_hash(),
			&crate::mock::get_mock_tx_block_num(),
			&*ETHLESS_RESPONSES,
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
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&bad_from_unverified),
			Err(OffchainError::InvalidTask(InvalidAddress))
		);

		mock_requests(&state);

		let bad_to_unverified =
			MockUnverifiedTransfer { to_external: default(), ..unverified.clone() };

		assert_matches!(
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&bad_to_unverified),
			Err(OffchainError::InvalidTask(InvalidAddress))
		);

		mock_requests(&state);

		unverified.transfer.kind = TransferKind::Ethless(default());

		assert_matches!(
			crate::Pallet::<TestRuntime>::verify_transfer_ocw(&unverified),
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
		let dummy_url = "dummy";
		set_rpc_uri(&CONTRACT_CHAIN, &dummy_url);

		let mut rpcs =
			MockedRpcRequests::new(dummy_url, &*TX_HASH, &*BLOCK_NUMBER_STR, &*RESPONSES);
		rpcs.mock_get_block_number(&mut state.write());

		let (acc, addr, sign, _) = generate_address_with_proof("collector");

		assert_ok!(Creditcoin::<TestRuntime>::register_address(
			Origin::signed(acc.clone()),
			CONTRACT_CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline = TestRuntime::unverified_transfer_deadline();
		//register twice (oversubscribe) under different expiration (aka deadline).
		assert_ok!(Creditcoin::<TestRuntime>::request_collect_coins(
			Origin::signed(acc.clone()),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to(2);
		let deadline_2 = TestRuntime::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<TestRuntime>::request_collect_coins(
			Origin::signed(acc),
			addr.clone(),
			TX_HASH.hex_to_address()
		));

		//We now have 2 enqueued tasks.

		roll_to_with_ocw(3);

		let collected_coins_id =
			CollectedCoinsId::new::<TestRuntime>(TX_HASH.hex_to_address().as_slice());
		let collected_coins = CollectedCoins {
			to: AddressId::new::<TestRuntime>(&CONTRACT_CHAIN, addr.as_ref()),
			amount: RPC_RESPONSE_AMOUNT.as_u128(),
			tx_id: TX_HASH.hex_to_address(),
		};

		let tx = pool.write().transactions.pop().expect("persist collect_coins");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			tx.call,
			Call::Creditcoin(crate::Call::persist_collect_coins { collected_coins, deadline })
		);

		assert_ok!(tx.call.dispatch(Origin::signed(auth)));

		roll_to_with_ocw(deadline_2);

		//task expires without yielding txns.
		assert!(pool.read().transactions.is_empty());

		type H = <TestRuntime as SystemConfig>::Hash;
		type Bn = <TestRuntime as SystemConfig>::BlockNumber;
		type Id = CollectedCoinsId<H>;
		let key = <UnverifiedCollectedCoins as Task<TestRuntime, Bn, Id>>::status_key(
			&collected_coins_id,
		);

		type Y = <BlockAndTime<System<TestRuntime>> as Lockable>::Deadline;
		//lock set
		assert!(StorageValueRef::persistent(key.as_ref()).get::<Y>().expect("decoded").is_some());
	});
}

use super::collect_coins::tests::{BLOCK_NUMBER_STR, RESPONSES, RPC_RESPONSE_AMOUNT};
use sp_runtime::traits::Dispatchable;

//tasks can be oversubscribed with different deadlines
#[test]
fn task_deadline_oversubscription() {
	let ext = ExtBuilder::default();
	ext.build_offchain_and_execute_with_state(|_, _| {
		let (acc, addr, sign, _) = generate_address_with_proof("collector");

		assert_ok!(Creditcoin::<TestRuntime>::register_address(
			Origin::signed(acc.clone()),
			CONTRACT_CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline_1 = TestRuntime::unverified_transfer_deadline();
		//register twice under different (expiration aka deadline)
		assert_ok!(Creditcoin::<TestRuntime>::request_collect_coins(
			Origin::signed(acc.clone()),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to(2);
		let deadline_2 = TestRuntime::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<TestRuntime>::request_collect_coins(
			Origin::signed(acc),
			addr,
			TX_HASH.hex_to_address()
		));

		let collected_coins_id =
			CollectedCoinsId::new::<TestRuntime>(TX_HASH.hex_to_address().as_slice());

		assert!(Creditcoin::<TestRuntime>::pending_collect_coins(
			deadline_1,
			collected_coins_id.clone()
		)
		.is_some());
		assert!(Creditcoin::<TestRuntime>::pending_collect_coins(deadline_2, collected_coins_id)
			.is_some());
	});
}

use crate::mock::{get_mock_tx_block_num, get_mock_tx_hash, roll_by_with_ocw};
use crate::tests::adjust_deal_order_to_nonce;

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
		let blockchain = Blockchain::Rinkeby;

		let tx_block_num_value =
			u64::from_str_radix(tx_block_num.trim_start_matches("0x"), 16).unwrap();

		set_rpc_uri(&Blockchain::Rinkeby, &dummy_url);

		let loan_amount = get_mock_amount();
		let terms = LoanTerms { amount: loan_amount, ..Default::default() };

		let test_info = TestInfo { blockchain, loan_terms: terms, ..Default::default() };

		let (_, deal_order_id) = test_info.create_deal_order();

		let deal_order_id = adjust_deal_order_to_nonce(&deal_order_id, get_mock_nonce());

		let lender = test_info.lender.account_id;
		assert_ok!(Creditcoin::<TestRuntime>::register_funding_transfer(
			Origin::signed(lender),
			TransferKind::Ethless(contract),
			deal_order_id,
			tx_hash.hex_to_address(),
		));

		let mock_unconfirmed_tx = || {
			let mut requests =
				MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &*ETHLESS_RESPONSES);
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

		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &*ETHLESS_RESPONSES)
			.mock_all(&mut state.write());

		roll_by_with_ocw(1);

		// we should have retried and successfully verified the transfer
		let tx = pool.write().transactions.pop().expect("verify transfer");
		assert!(pool.read().transactions.is_empty());
		let verify_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_matches!(verify_tx.call, Call::Creditcoin(crate::Call::verify_transfer { .. }));
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
		let blockchain = Blockchain::Rinkeby;

		// mocks for when we expect failure
		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &*ETHLESS_RESPONSES)
			.mock_get_block_number(&mut state.write());
		// mocks for when we expect success
		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &*ETHLESS_RESPONSES)
			.mock_all(&mut state.write());

		set_rpc_uri(&Blockchain::Rinkeby, &dummy_url);

		let loan_amount = get_mock_amount();
		let terms = LoanTerms { amount: loan_amount, ..Default::default() };

		let test_info =
			TestInfo { blockchain: blockchain.clone(), loan_terms: terms, ..Default::default() };
		let (_, deal_order_id) = test_info.create_deal_order();
		let lender = test_info.lender.account_id.clone();

		// test that we get a "fail_transfer" tx when verification fails
		assert_ok!(Creditcoin::<TestRuntime>::register_funding_transfer(
			Origin::signed(lender.clone()),
			TransferKind::Ethless(contract.clone()),
			deal_order_id.clone(),
			tx_hash.hex_to_address(),
		));
		let deadline = TestRuntime::unverified_transfer_deadline();

		roll_to_with_ocw(1);

		let transfer_id = TransferId::new::<TestRuntime>(&blockchain, &tx_hash.hex_to_address());
		let tx = pool.write().transactions.pop().expect("fail transfer");
		assert!(pool.read().transactions.is_empty());
		let fail_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			fail_tx.call,
			Call::Creditcoin(crate::Call::fail_transfer {
				transfer_id,
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

		assert_ok!(Creditcoin::<TestRuntime>::register_funding_transfer(
			Origin::signed(lender.clone()),
			TransferKind::Ethless(contract.clone()),
			fake_deal_order_id.clone(),
			tx_hash.hex_to_address(),
		));

		let deadline_2 = TestRuntime::unverified_transfer_deadline();

		let expected_transfer = crate::Transfer {
			blockchain: test_info.blockchain.clone(),
			kind: TransferKind::Ethless(contract),
			amount: loan_amount,
			block: System::<TestRuntime>::block_number(),
			from: test_info.lender.address_id.clone(),
			to: test_info.borrower.address_id,
			order_id: OrderId::Deal(fake_deal_order_id),
			is_processed: false,
			account_id: lender,
			tx_id: tx_hash.hex_to_address(),
			timestamp: Some(get_mock_timestamp()),
		};

		//We expect the guard to expire on the next roll, sleep to meet time requirements.
		let lock_expires = offchain::timestamp().add(Duration::from_millis(1));
		offchain::sleep_until(lock_expires);

		//guard run at 1 is reacquireable at...
		roll_to_with_ocw(deadline);

		let tx = pool.write().transactions.pop().expect("verify transfer");
		assert!(pool.read().transactions.is_empty());
		let verify_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			verify_tx.call,
			Call::Creditcoin(crate::Call::verify_transfer {
				transfer: expected_transfer,
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
		let dummy_url = "dummy";
		set_rpc_uri(&CONTRACT_CHAIN, &dummy_url);
		let mut rpcs =
			MockedRpcRequests::new(dummy_url, &*TX_HASH, &*BLOCK_NUMBER_STR, &*RESPONSES);
		rpcs.mock_get_block_number(&mut state.write());

		let (acc, addr, sign, _) = generate_address_with_proof("collector");
		assert_ok!(Creditcoin::<TestRuntime>::register_address(
			Origin::signed(acc.clone()),
			CONTRACT_CHAIN,
			addr.clone(),
			sign
		));

		roll_to(1);
		let deadline = TestRuntime::unverified_transfer_deadline();
		assert_ok!(Creditcoin::<TestRuntime>::request_collect_coins(
			Origin::signed(acc),
			addr.clone(),
			TX_HASH.hex_to_address()
		));
		roll_to_with_ocw(2);

		let tx = pool.write().transactions.pop().expect("persist collect_coins");
		assert!(pool.read().transactions.is_empty());
		let tx = Extrinsic::decode(&mut &*tx).unwrap();

		let collected_coins = CollectedCoins {
			to: AddressId::new::<TestRuntime>(&CONTRACT_CHAIN, addr.as_ref()),
			amount: RPC_RESPONSE_AMOUNT.as_u128(),
			tx_id: TX_HASH.hex_to_address(),
		};

		assert_eq!(
			tx.call,
			Call::Creditcoin(crate::Call::persist_collect_coins { collected_coins, deadline })
		);

		let key = {
			let collected_coins_id =
				CollectedCoinsId::new::<TestRuntime>(TX_HASH.hex_to_address().as_slice());

			type H = <TestRuntime as SystemConfig>::Hash;
			type Bn = <TestRuntime as SystemConfig>::BlockNumber;
			type Id = CollectedCoinsId<H>;
			<UnverifiedCollectedCoins as Task<TestRuntime, Bn, Id>>::status_key(&collected_coins_id)
		};

		type Y = <BlockAndTime<System<TestRuntime>> as Lockable>::Deadline;
		//lock set
		let Y { block_number, .. } = StorageValueRef::persistent(key.as_ref())
			.get::<Y>()
			.expect("decoded")
			.expect("deadline");
		println!("{block_number} {deadline}");
		assert!(block_number >= deadline - 1);
	});
}
