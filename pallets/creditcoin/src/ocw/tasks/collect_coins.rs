use crate::pallet::{Config as CreditcoinConfig, Pallet};
use crate::{
	ocw::{
		self,
		errors::{VerificationFailureCause, VerificationResult},
		rpc::{self, EthTransaction, EthTransactionReceipt},
		OffchainResult, ETH_CONFIRMATIONS,
	},
	Blockchain,
};
use crate::{types::UnverifiedCollectedCoins, ExternalAddress, ExternalAmount};
use core::default::Default;
use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_types::U64;
use frame_support::{ensure, RuntimeDebug};
use hex_literal::hex;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::H160;
use sp_runtime::SaturatedConversion;
#[cfg_attr(feature = "std", allow(unused_imports))]
use sp_std::prelude::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GCreContract {
	pub address: sp_core::H160,
	pub chain: Blockchain,
}

impl GCreContract {
	const DEFAULT_CHAIN: Blockchain = Blockchain::ETHEREUM;
}

impl Default for GCreContract {
	fn default() -> Self {
		let contract_chain: Blockchain = GCreContract::DEFAULT_CHAIN;
		let contract_address: H160 =
			sp_core::H160(hex!("a3EE21C306A700E682AbCdfe9BaA6A08F3820419"));
		Self { address: contract_address, chain: contract_chain }
	}
}

impl GCreContract {
	///exchange has been deprecated, use burn instead
	fn burn_vested_cc_abi() -> Function {
		#[allow(deprecated)]
		Function {
			name: "burn".into(),
			inputs: vec![Param {
				name: "value".into(),
				kind: ParamType::Uint(256),
				internal_type: None,
			}],
			outputs: vec![Param {
				name: "success".into(),
				kind: ParamType::Bool,
				internal_type: None,
			}],
			constant: Some(false),
			state_mutability: StateMutability::NonPayable,
		}
	}
}

pub fn validate_collect_coins(
	to: &ExternalAddress,
	receipt: &EthTransactionReceipt,
	transaction: &EthTransaction,
	eth_tip: U64,
	contract_address: &H160,
) -> OffchainResult<ExternalAmount> {
	ensure!(receipt.is_success(), VerificationFailureCause::TaskFailed);

	let block_number = transaction.block_number.ok_or(VerificationFailureCause::TaskPending)?;

	let diff = (eth_tip)
		.checked_sub(block_number)
		.ok_or(VerificationFailureCause::TaskInFuture)?;
	ensure!(diff.as_u64() >= ETH_CONFIRMATIONS, VerificationFailureCause::TaskUnconfirmed);

	if let Some(to) = &transaction.to {
		ensure!(to == contract_address, VerificationFailureCause::IncorrectContract);
	} else {
		return Err(VerificationFailureCause::MissingReceiver.into());
	}

	if let Some(from) = &transaction.from {
		ensure!(from[..] == to[..], VerificationFailureCause::IncorrectSender)
	} else {
		return Err(VerificationFailureCause::MissingSender.into());
	}

	let transfer_fn = GCreContract::burn_vested_cc_abi();
	ensure!(!transaction.is_input_empty(), VerificationFailureCause::EmptyInput);

	{
		let selector = transaction.selector();
		if selector != transfer_fn.short_signature() {
			log::error!(
				"function selector mismatch, expected: {}, got: {}",
				hex::encode(&transfer_fn.short_signature()),
				hex::encode(selector)
			);
			return Err(VerificationFailureCause::AbiMismatch.into());
		}
	}

	let inputs = transfer_fn.decode_input(transaction.input()).map_err(|e| {
		log::error!("failed to decode inputs: {:?}", e);
		VerificationFailureCause::AbiMismatch
	})?;

	match inputs.get(0) {
		Some(Token::Uint(value)) => Ok(ExternalAmount::from(value)),
		_ => Err(VerificationFailureCause::IncorrectInputType.into()),
	}
}

impl<T: CreditcoinConfig> Pallet<T> {
	///Amount is saturated to u128, don't exchange more than u128::MAX at once.
	pub fn verify_collect_coins_ocw(
		u_cc: &UnverifiedCollectedCoins,
	) -> VerificationResult<T::Balance> {
		log::debug!("verifying OCW Collect Coins");
		let UnverifiedCollectedCoins { to, tx_id, contract: GCreContract { address, chain } } =
			u_cc;
		let rpc_url = &chain.rpc_url()?;
		let tx = ocw::eth_get_transaction(tx_id, rpc_url)?;
		let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, rpc_url)?;
		let eth_tip = rpc::eth_get_block_number(rpc_url)?;

		let amount = validate_collect_coins(to, &tx_receipt, &tx, eth_tip, address)?;

		let amount = amount.saturated_into::<u128>().saturated_into::<T::Balance>();

		Ok(amount)
	}
}

#[cfg(any(test, feature = "runtime-benchmarks"))]
pub(crate) mod testing_constants {
	use super::{Blockchain, GCreContract};

	pub const CHAIN: Blockchain = GCreContract::DEFAULT_CHAIN;
}

#[cfg(test)]
pub(crate) mod tests {

	use super::*;
	use crate::mock::PendingRequestExt;
	use crate::TaskId;
	use std::collections::HashMap;

	// txn.from has been overriden by 'generate_address_with_proof("collector")'
	static RESPONSES: Lazy<HashMap<String, JsonRpcResponse<serde_json::Value>>> = Lazy::new(|| {
		serde_json::from_slice(include_bytes!("../../tests/collectCoins.json")).unwrap()
	});

	static BLOCK_NUMBER: Lazy<U64> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let bn =
			responses["eth_getTransactionByHash"].result.clone().unwrap()["blockNumber"].clone();
		serde_json::from_value(bn).unwrap()
	});

	static BLOCK_NUMBER_STR: Lazy<String> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let bn =
			responses["eth_getTransactionByHash"].result.clone().unwrap()["blockNumber"].clone();
		serde_json::from_value(bn).unwrap()
	});

	static VESTING_CONTRACT: Lazy<H160> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let val = responses["eth_getTransactionByHash"].result.clone().unwrap()["to"].clone();
		let val: String = serde_json::from_value(val).unwrap();
		let vesting_contract = hex::decode(val.trim_start_matches("0x")).unwrap();
		H160::from(<[u8; 20]>::try_from(vesting_contract.as_slice()).unwrap())
	});

	// txn.from has been overriden by 'generate_address_with_proof("collector")'
	static FROM: Lazy<String> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let val = responses["eth_getTransactionByHash"].result.clone().unwrap()["from"].clone();
		serde_json::from_value(val).unwrap()
	});

	static INPUT: Lazy<rpc::Bytes> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let val = responses["eth_getTransactionByHash"].result.clone().unwrap()["input"].clone();
		let val: String = serde_json::from_value(val).unwrap();
		let input_bytes = hex::decode(val.trim_start_matches("0x")).unwrap();
		input_bytes.into()
	});

	pub(crate) static TX_HASH: Lazy<String> = Lazy::new(|| {
		let responses = &*RESPONSES;
		let val = responses["eth_getTransactionByHash"].result.clone().unwrap()["hash"].clone();
		serde_json::from_value(val).unwrap()
	});

	pub(crate) static RPC_RESPONSE_AMOUNT: Lazy<sp_core::U256> = Lazy::new(|| {
		let transfer_fn = GCreContract::burn_vested_cc_abi();

		let inputs = transfer_fn.decode_input(&(INPUT.0)[4..]).unwrap();

		let amount = inputs.get(0).unwrap();
		if let Token::Uint(value) = amount {
			ExternalAmount::from(value)
		} else {
			panic!("Not Token::Uint");
		}
	});

	use crate::helpers::extensions::HexToAddress;
	use crate::helpers::non_paying_error;
	use crate::mock::{
		roll_by_with_ocw, set_rpc_uri, AccountId, Balances, ExtBuilder, MockedRpcRequests,
		OffchainState, RuntimeOrigin, RwLock, Test,
	};
	use crate::ocw::tasks::TaskV2;
	use crate::ocw::{
		errors::{OffchainError, VerificationFailureCause as Cause},
		rpc::{EthTransaction, EthTransactionReceipt},
		ETH_CONFIRMATIONS,
	};
	use crate::tests::generate_address_with_proof;
	use crate::types::{AddressId, CollectedCoins, CollectedCoinsId};
	use crate::Pallet as Creditcoin;
	use crate::{ocw::rpc::JsonRpcResponse, ExternalAddress};
	use alloc::sync::Arc;
	use assert_matches::assert_matches;
	use frame_support::{assert_noop, assert_ok, once_cell::sync::Lazy, traits::Currency};
	use frame_system::Pallet as System;
	use frame_system::RawOrigin;
	use pallet_offchain_task_scheduler::tasks::TaskScheduler;
	use pallet_offchain_task_scheduler::Pallet as TaskSchedulerPallet;
	use parity_scale_codec::Decode;
	use sp_runtime::traits::{BadOrigin, IdentifyAccount};
	use sp_runtime::{ArithmeticError, TokenError};

	use std::convert::TryFrom;

	use super::testing_constants::CHAIN;

	fn prepare_rpc_mocks() -> MockedRpcRequests {
		let dummy_url = "dummy";
		let contract_chain = Creditcoin::<Test>::collect_coins_contract();
		set_rpc_uri(&contract_chain.chain, &dummy_url);

		MockedRpcRequests::new(dummy_url, &TX_HASH, &BLOCK_NUMBER_STR, &RESPONSES)
	}

	/// call from externalities context
	pub(crate) fn mock_rpc_for_collect_coins(state: &Arc<RwLock<OffchainState>>) {
		let mut rpcs = prepare_rpc_mocks();
		rpcs.mock_get_block_number(&mut state.write());
	}

	struct PassingCollectCoins {
		to: ExternalAddress,
		receipt: EthTransactionReceipt,
		transaction: EthTransaction,
		eth_tip: U64,
		contract_address: H160,
	}

	impl Default for PassingCollectCoins {
		fn default() -> Self {
			let base_height = *BLOCK_NUMBER;
			let vesting_contract = *VESTING_CONTRACT;
			let to = FROM.hex_to_address();
			let tx_from = H160::from(<[u8; 20]>::try_from(to.as_slice()).unwrap());

			let mut transaction = EthTransaction::default();
			transaction.block_number = Some(base_height);
			transaction.from = Some(tx_from);
			transaction.to = Some(vesting_contract);
			transaction.set_input(&INPUT.0);

			Self {
				to,
				receipt: EthTransactionReceipt { status: Some(1u64.into()), ..Default::default() },
				transaction,
				eth_tip: (base_height + ETH_CONFIRMATIONS),
				contract_address: GCreContract::default().address,
			}
		}
	}

	impl PassingCollectCoins {
		fn validate(self) -> OffchainResult<ExternalAmount> {
			let PassingCollectCoins { to, receipt, transaction, eth_tip, contract_address } = self;
			super::validate_collect_coins(&to, &receipt, &transaction, eth_tip, &contract_address)
		}
	}

	fn assert_invalid(res: OffchainResult<ExternalAmount>, cause: VerificationFailureCause) {
		assert_matches!(res, Err(OffchainError::InvalidTask(c)) =>{ assert_eq!(c,cause); });
	}

	#[test]
	fn valid() {
		assert_matches!(PassingCollectCoins::default().validate(), Ok(_));
	}

	#[test]
	fn txn_success() {
		let mut pcc = PassingCollectCoins::default();
		pcc.receipt.status = Some(0u64.into());
		assert_invalid(pcc.validate(), Cause::TaskFailed);
	}

	#[test]
	fn pending() {
		let mut transaction = EthTransaction::default();
		transaction.block_number = None;
		let pcc = PassingCollectCoins { transaction, ..Default::default() };
		assert_invalid(pcc.validate(), Cause::TaskPending);
	}

	#[test]
	fn in_the_future() {
		let pcc = PassingCollectCoins { eth_tip: 0u64.into(), ..Default::default() };
		assert_invalid(pcc.validate(), Cause::TaskInFuture);
	}

	#[test]
	fn unconfirmed() {
		let mut pcc = PassingCollectCoins::default();
		pcc.eth_tip = pcc.transaction.block_number.unwrap();
		assert_invalid(pcc.validate(), Cause::TaskUnconfirmed);
	}

	#[test]
	fn missing_receiver() {
		let mut pcc = PassingCollectCoins::default();
		pcc.transaction.to = None;
		assert_invalid(pcc.validate(), Cause::MissingReceiver);
	}

	#[test]
	fn incorrect_contract() {
		let mut pcc = PassingCollectCoins::default();
		let address = [0u8; 20];
		let address = H160::from(<[u8; 20]>::try_from(address.as_slice()).unwrap());
		pcc.transaction.to = Some(address);
		assert_invalid(pcc.validate(), Cause::IncorrectContract);
	}

	#[test]
	fn missing_sender() {
		let mut pcc = PassingCollectCoins::default();
		pcc.transaction.from = None;
		assert_invalid(pcc.validate(), Cause::MissingSender);
	}

	#[test]
	fn incorrect_sender() {
		let mut pcc = PassingCollectCoins::default();
		let address = [0u8; 20];
		let address = H160::from(<[u8; 20]>::try_from(address.as_slice()).unwrap());
		pcc.transaction.from = Some(address);
		assert_invalid(pcc.validate(), Cause::IncorrectSender);
	}

	#[test]
	fn empty_input() {
		let mut pcc = PassingCollectCoins::default();
		pcc.transaction.set_input(b"");
		assert_invalid(pcc.validate(), Cause::EmptyInput);
	}

	#[test]
	fn amount_set() -> OffchainResult<()> {
		let pcc = PassingCollectCoins::default();
		let PassingCollectCoins { to, receipt, transaction, eth_tip, contract_address } = pcc;
		let amount =
			super::validate_collect_coins(&to, &receipt, &transaction, eth_tip, &contract_address)?;
		assert_eq!(amount, *RPC_RESPONSE_AMOUNT);
		Ok(())
	}

	#[test]
	fn fail_collect_coins_should_error_when_not_signed() {
		let ext = ExtBuilder::default();
		let expected_collected_coins_id =
			crate::CollectedCoinsId::new::<crate::mock::Test>(&CHAIN, &[0]);

		ext.build_offchain_and_execute_with_state(|_state, _pool| {
			assert_noop!(
				Creditcoin::<Test>::fail_task(
					RuntimeOrigin::none(),
					Test::unverified_transfer_deadline(),
					expected_collected_coins_id.clone().into(),
					Cause::AbiMismatch,
				),
				BadOrigin
			);
		});
	}

	#[test]
	fn fail_collect_coins_should_error_when_no_authority() {
		let ext = ExtBuilder::default();
		let (molly, _, _, _) = generate_address_with_proof("malicious");
		let expected_collected_coins_id =
			crate::CollectedCoinsId::new::<crate::mock::Test>(&CHAIN, &[0]);

		ext.build_offchain_and_execute_with_state(|_state, _pool| {
			assert_noop!(
				Creditcoin::<Test>::fail_task(
					RuntimeOrigin::signed(molly),
					Test::unverified_transfer_deadline(),
					expected_collected_coins_id.clone().into(),
					Cause::AbiMismatch,
				),
				crate::Error::<Test>::InsufficientAuthority
			);
		});
	}

	#[test]
	fn fail_collect_coins_should_fail_when_transfer_has_already_been_registered() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);

		ext.build_offchain_and_execute_with_state(|_state, _pool| {
			System::<Test>::set_block_number(1);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc),
				CHAIN,
				addr,
				sign
			));

			let deadline = Test::unverified_transfer_deadline();

			let pcc = PassingCollectCoins::default();

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &pcc.to[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id =
				crate::CollectedCoinsId::new::<crate::mock::Test>(&CHAIN, &collected_coins.tx_id);

			assert_ok!(Creditcoin::<Test>::persist_task_output(
				RuntimeOrigin::signed(auth.clone()),
				deadline,
				(collected_coins_id.clone(), collected_coins).into(),
			));

			assert_noop!(
				Creditcoin::<Test>::fail_task(
					RuntimeOrigin::signed(auth),
					Test::unverified_transfer_deadline(),
					collected_coins_id.into(),
					Cause::AbiMismatch,
				),
				crate::Error::<Test>::CollectCoinsAlreadyRegistered
			);
		});
	}

	#[test]
	fn fail_collect_coins_emits_events() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		let expected_collected_coins_id =
			crate::CollectedCoinsId::new::<crate::mock::Test>(&CHAIN, &[0]);

		ext.build_offchain_and_execute_with_state(|_state, _pool| {
			System::<Test>::set_block_number(1);

			assert_ok!(Creditcoin::<Test>::fail_task(
				RuntimeOrigin::signed(auth),
				Test::unverified_transfer_deadline(),
				expected_collected_coins_id.clone().into(),
				Cause::AbiMismatch,
			));

			let event = System::<Test>::events().pop().expect("an event").event;
			assert_matches!(
				event,
				crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::CollectCoinsFailedVerification(collected_coins_id, cause)) => {
					assert_eq!(collected_coins_id, expected_collected_coins_id);
					assert_eq!(cause, Cause::AbiMismatch);
				}
			);
		});
	}

	#[test]
	fn ocw_fail_collect_coins_works() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let acct = AccountId::from(acct_pubkey.into_account().0);
		let expected_collected_coins_id =
			crate::CollectedCoinsId::new::<crate::mock::Test>(&CHAIN, &[0]);
		ext.build_offchain_and_execute_with_state(|_state, pool| {
			crate::mock::roll_to(1);
			let call = crate::Call::<Test>::fail_task {
				task_id: expected_collected_coins_id.into(),
				cause: Cause::AbiMismatch,
				deadline: Test::unverified_transfer_deadline(),
			};
			assert_ok!(TaskSchedulerPallet::<Test>::offchain_signed_tx(acct.clone(), |_| call
				.clone()
				.into(),));
			crate::mock::roll_to(2);

			assert_matches!(pool.write().transactions.pop(), Some(tx) => {
			let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
			assert_eq!(tx.call, crate::mock::RuntimeCall::Creditcoin(call));
			});
		});
	}

	#[test]
	fn persist_collect_coins() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			System::<Test>::set_block_number(1);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr,
				sign
			));

			let deadline = Test::unverified_transfer_deadline();

			let pcc = PassingCollectCoins::default();

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &pcc.to[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};

			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			let balance = <Balances as Currency<AccountId>>::total_balance;

			let pre_authority_balance = balance(&auth);
			let pre_collector_balance = balance(&acc);

			assert_ok!(Creditcoin::<Test>::persist_task_output(
				RuntimeOrigin::signed(auth.clone()),
				deadline,
				(collected_coins_id.clone(), collected_coins.clone()).into(),
			));

			let event = <frame_system::Pallet<Test>>::events().pop().expect("an event").event;

			assert_matches!(
				event,
				crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::CollectedCoinsMinted(id, item)) => {
					assert_eq!(id, collected_coins_id);
					assert_eq!(item, collected_coins);
				}
			);
			//do not mint into authority
			assert_eq!(pre_authority_balance, balance(&auth));
			// assert on deposit
			assert_eq!(pre_collector_balance.saturating_add(collected_coins.amount), balance(&acc));
		});
	}

	#[test]
	fn persist_unregistered_address() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			let pcc = PassingCollectCoins::default();

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &pcc.to[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			let deadline = Test::unverified_transfer_deadline();

			assert_noop!(
				Creditcoin::<Test>::persist_task_output(
					RuntimeOrigin::signed(auth),
					deadline,
					(collected_coins_id, collected_coins).into(),
				),
				crate::Error::<Test>::NonExistentAddress
			);
		});
	}

	#[test]
	fn persist_more_than_max_balance_should_error() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");
			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc),
				CHAIN,
				addr,
				sign
			));

			let pcc = PassingCollectCoins::default();

			// lower free balance so that collect coins would overflow
			let cash = <crate::mock::Balances as Currency<AccountId>>::minimum_balance();
			<crate::mock::Balances as Currency<AccountId>>::make_free_balance_be(&auth, cash);

			let collected_coins_id =
				crate::CollectedCoinsId::new::<Test>(&CHAIN, &TX_HASH.hex_to_address());
			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &pcc.to[..]),
				amount: u128::MAX,
				tx_id: TX_HASH.hex_to_address(),
			};

			assert_noop!(
				Creditcoin::<Test>::persist_task_output(
					RuntimeOrigin::signed(auth),
					Test::unverified_transfer_deadline(),
					(collected_coins_id, collected_coins).into(),
				),
				ArithmeticError::Overflow
			);
		});
	}

	#[test]
	fn request_persisted_not_reentrant() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _pool| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			assert_ok!(Creditcoin::<Test>::persist_task_output(
				RuntimeOrigin::signed(auth),
				Test::unverified_transfer_deadline(),
				(collected_coins_id, collected_coins).into(),
			));

			roll_by_with_ocw(1);

			assert_noop!(
				Creditcoin::<Test>::request_collect_coins(
					RuntimeOrigin::signed(acc),
					addr,
					TX_HASH.hex_to_address(),
				),
				crate::Error::<Test>::CollectCoinsAlreadyRegistered
			);
		});
	}

	#[test]
	fn request_pending_not_reentrant() {
		let mut ext = ExtBuilder::default();
		ext.generate_authority();
		ext.build_offchain_and_execute_with_state(|_, _| {
			System::<Test>::set_block_number(1);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc.clone()),
				addr.clone(),
				TX_HASH.hex_to_address()
			));

			let collected_coins_id =
				CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());

			let event = <frame_system::Pallet<Test>>::events().pop().expect("an event").event;
			assert_matches!(
				event,
				crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::CollectCoinsRegistered(collect_coins_id, pending)) => {
					assert_eq!(collect_coins_id, collected_coins_id);

					let id = TaskV2::<Test>::to_id(&pending);
					assert!( <Test>::is_scheduled( &Test::unverified_transfer_deadline(), &id));

					let UnverifiedCollectedCoins { to, tx_id, .. } = pending;
					assert_eq!(to, addr);
					assert_eq!(tx_id, TX_HASH.hex_to_address());
				}
			);

			assert_noop!(
				Creditcoin::<Test>::request_collect_coins(
					RuntimeOrigin::signed(acc),
					addr,
					TX_HASH.hex_to_address(),
				),
				crate::Error::<Test>::CollectCoinsAlreadyRegistered
			);

			assert!(Creditcoin::<Test>::collected_coins(collected_coins_id).is_none());
		});
	}

	#[test]
	fn request_address_not_registered() {
		let ext = ExtBuilder::default();
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, _, _) = generate_address_with_proof("collector");

			assert_noop!(
				Creditcoin::<Test>::request_collect_coins(
					RuntimeOrigin::signed(acc),
					addr,
					TX_HASH.hex_to_address(),
				),
				crate::Error::<Test>::NonExistentAddress
			);
		});
	}

	#[test]
	fn request_not_owner() {
		let ext = ExtBuilder::default();
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");
			let (molly, _, _, _) = generate_address_with_proof("malicious");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc),
				CHAIN,
				addr.clone(),
				sign
			));

			assert_noop!(
				Creditcoin::<Test>::request_collect_coins(
					RuntimeOrigin::signed(molly),
					addr,
					TX_HASH.hex_to_address(),
				),
				crate::Error::<Test>::NotAddressOwner
			);
		});
	}

	#[test]
	fn persist_not_authority() {
		let ext = ExtBuilder::default();
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (molly, addr, _, _) = generate_address_with_proof("malicious");

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			assert_noop!(
				Creditcoin::<Test>::persist_task_output(
					RuntimeOrigin::signed(molly),
					Test::unverified_transfer_deadline(),
					(collected_coins_id, collected_coins).into(),
				),
				crate::Error::<Test>::InsufficientAuthority
			);
		});
	}

	#[test]
	fn persist_is_submitted() {
		let mut ext = ExtBuilder::default();
		ext.generate_authority();
		ext.build_offchain_and_execute_with_state(|state, pool| {
			mock_rpc_for_collect_coins(&state);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc),
				addr.clone(),
				TX_HASH.hex_to_address()
			));

			let deadline = Test::unverified_transfer_deadline();

			roll_by_with_ocw(1);

			assert!(!pool.read().transactions.is_empty());

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			let call = crate::Call::<crate::mock::Test>::persist_task_output {
				task_output: (collected_coins_id, collected_coins).into(),
				deadline,
			};

			assert_matches!(pool.write().transactions.pop(), Some(tx) => {
				let tx = crate::mock::Extrinsic::decode(&mut &*tx).unwrap();
				assert_eq!(tx.call, crate::mock::RuntimeCall::Creditcoin(call));
			});
		});
	}

	#[test]
	fn persist_not_reentrant() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc),
				CHAIN,
				addr.clone(),
				sign
			));

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			assert_ok!(Creditcoin::<Test>::persist_task_output(
				RuntimeOrigin::signed(auth.clone()),
				Test::unverified_transfer_deadline(),
				(collected_coins_id.clone(), collected_coins.clone()).into(),
			));

			assert_noop!(
				Creditcoin::<Test>::persist_task_output(
					RuntimeOrigin::signed(auth),
					Test::unverified_transfer_deadline(),
					(collected_coins_id, collected_coins).into(),
				),
				non_paying_error(crate::Error::<Test>::CollectCoinsAlreadyRegistered)
			);
		});
	}

	#[test]
	fn unverified_collect_coins_are_removed() {
		let mut ext = ExtBuilder::default();
		ext.generate_authority();
		ext.build_offchain_and_execute_with_state(|state, _| {
			mock_rpc_for_collect_coins(&state);

			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr.clone(),
				sign
			));

			assert_ok!(Creditcoin::<Test>::request_collect_coins(
				RuntimeOrigin::signed(acc),
				addr,
				TX_HASH.hex_to_address()
			));
			let deadline = Test::unverified_transfer_deadline();

			roll_by_with_ocw(deadline);

			let collected_coins_id =
				CollectedCoinsId::new::<Test>(&CHAIN, TX_HASH.hex_to_address().as_slice());

			roll_by_with_ocw(1);

			assert!(Creditcoin::<Test>::pending_tasks(deadline, TaskId::from(collected_coins_id))
				.is_none());
		});
	}

	#[test]
	fn owner_credited() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: RPC_RESPONSE_AMOUNT.as_u128(),
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr,
				sign
			));

			assert_ok!(Creditcoin::<Test>::persist_task_output(
				RuntimeOrigin::signed(auth.clone()),
				Test::unverified_transfer_deadline(),
				(collected_coins_id, collected_coins.clone()).into(),
			));

			assert_eq!(
				frame_system::pallet::Account::<Test>::get(&acc).data.free,
				collected_coins.amount
			);
		});
	}

	#[test]
	fn selector_mismatch() {
		let ext = ExtBuilder::default();
		ext.build_offchain_and_execute_with_state(|state, _| {
			mock_rpc_for_collect_coins(&state);

			let (_, to, ..) = generate_address_with_proof("collector");
			let tx_id = &TX_HASH.hex_to_address();

			let rpc_url = &CHAIN.rpc_url().unwrap();
			let mut tx = rpc::eth_get_transaction(tx_id, rpc_url).unwrap();
			let tx_receipt = rpc::eth_get_transaction_receipt(tx_id, rpc_url).unwrap();
			let eth_tip = rpc::eth_get_block_number(rpc_url).unwrap();
			let PassingCollectCoins { contract_address, .. } = Default::default();
			validate_collect_coins(&to, &tx_receipt, &tx, eth_tip, &contract_address)
				.expect("valid");
			// Forged selector
			tx.set_input(b"ffffffff");
			assert_matches!(
				validate_collect_coins(&to, &tx_receipt, &tx, eth_tip, &contract_address),
				Err(OffchainError::InvalidTask(VerificationFailureCause::AbiMismatch))
			);
		});
	}

	#[test]
	fn set_collect_coins_only_as_root() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let _auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_and_execute(|| {
			let contract = GCreContract {
				address: sp_core::H160(hex!("aaaaabbbbbcccccdddddeeeeefffff08F3820419")),
				chain: Blockchain::RINKEBY,
			};
			assert_ok!(Creditcoin::<Test>::set_collect_coins_contract(
				RawOrigin::Root.into(),
				contract.clone()
			));
			let from_storage = Creditcoin::<Test>::collect_coins_contract();
			assert_eq!(contract, from_storage);
			assert_ne!(from_storage, GCreContract::default());

			let (acc, ..) = generate_address_with_proof("somebody");

			assert_noop!(
				Creditcoin::<Test>::set_collect_coins_contract(
					RawOrigin::Signed(acc).into(),
					contract.clone()
				),
				BadOrigin
			);

			assert_noop!(
				Creditcoin::<Test>::set_collect_coins_contract(RawOrigin::None.into(), contract),
				BadOrigin
			);
		});
	}

	#[test]
	fn gcrecontract_value_query_is_default() {
		let contract = GCreContract::default();
		let ext = ExtBuilder::default();
		ext.build_and_execute(|| {
			let value_query = Creditcoin::<Test>::collect_coins_contract();
			assert_eq!(contract, value_query);
		});
	}

	#[test]
	fn persist_minimum_existential_deposit_errors() {
		let mut ext = ExtBuilder::default();
		let acct_pubkey = ext.generate_authority();
		let auth = AccountId::from(acct_pubkey.into_account().0);
		ext.build_offchain_and_execute_with_state(|_, _| {
			let (acc, addr, sign, _) = generate_address_with_proof("collector");

			let collected_coins = CollectedCoins {
				to: AddressId::new::<Test>(&CHAIN, &addr[..]),
				amount: 1u128,
				tx_id: TX_HASH.hex_to_address(),
			};
			let collected_coins_id = CollectedCoinsId::new::<Test>(&CHAIN, &collected_coins.tx_id);

			assert_ok!(Creditcoin::<Test>::register_address(
				RuntimeOrigin::signed(acc.clone()),
				CHAIN,
				addr,
				sign
			));

			assert_eq!(Balances::total_balance(&acc), 0);

			assert_noop!(
				Creditcoin::<Test>::persist_task_output(
					RuntimeOrigin::signed(auth.clone()),
					Test::unverified_transfer_deadline(),
					(collected_coins_id, collected_coins).into(),
				),
				TokenError::BelowMinimum
			);
		});
	}

	#[test]
	fn transaction_not_found() {
		ExtBuilder::default().build_offchain_and_execute_with_state(|state, _| {
			let mut rpcs = prepare_rpc_mocks();
			rpcs.get_transaction.set_empty_response();
			rpcs.mock_get_transaction(&mut state.write());

			let (_, addr, _, _) = generate_address_with_proof("collector");
			let cc = UnverifiedCollectedCoins {
				to: addr,
				tx_id: TX_HASH.hex_to_address(),
				contract: GCreContract::default(),
			};
			assert_matches!(
				Creditcoin::<Test>::verify_collect_coins_ocw(&cc),
				Err(OffchainError::InvalidTask(VerificationFailureCause::TransactionNotFound))
			);
		});
	}
}
