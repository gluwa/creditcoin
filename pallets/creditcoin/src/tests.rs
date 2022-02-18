use std::collections::HashMap;

use crate::{
	mock::*, AddressId, Blockchain, ExternalAddress, ExternalAmount, Id, LoanTerms, OrderId,
	TransferKind,
};
use bstr::B;
use codec::Decode;
use ethereum_types::H256;
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};
use sp_runtime::offchain::storage::StorageValueRef;

#[extend::ext]
impl<'a, S> &'a [u8]
where
	S: Get<u32>,
{
	fn try_into_bounded(self) -> Result<BoundedVec<u8, S>, ()> {
		core::convert::TryFrom::try_from(self.to_vec())
	}
	fn into_bounded(self) -> BoundedVec<u8, S> {
		core::convert::TryFrom::try_from(self.to_vec()).unwrap()
	}
}

pub fn loan_terms() -> LoanTerms<u64> {
	LoanTerms {
		amount: ExternalAmount::from(1_000_0000u64),
		interest_rate: 0,
		maturity: 1_000_000_000_000,
	}
}

#[derive(Clone, Debug)]
pub struct TestInfo {
	blockchain: Blockchain,
	origin_account_id: AccountId,
	address_id: AddressId<H256>,
	loan_terms: LoanTerms<u64>,
}

pub fn prepare_test(address: &str) -> TestInfo {
	let account_id: <Test as frame_system::Config>::AccountId = AccountId::new([0; 32]);
	let blockchain = Blockchain::Rinkeby;
	let address: ExternalAddress = address.as_bytes().into_bounded();
	let address_id = AddressId::new::<Test>(&blockchain, &address);

	assert_ok!(Creditcoin::register_address(
		Origin::signed(account_id.clone()),
		blockchain.clone(),
		address.clone(),
	));

	let loan_terms = loan_terms();
	TestInfo { blockchain, origin_account_id: account_id, address_id, loan_terms }
}

#[test]
fn register_address_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);
		let blockchain = Blockchain::Rinkeby;
		let value = B("someaddressvalue").into_bounded();
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.clone(),
			value.clone(),
		));
		let address_id = crate::AddressId::new::<Test>(&blockchain, &value);
		let address = crate::Address { blockchain, value, owner: acct };

		assert_eq!(Creditcoin::addresses(address_id), Some(address));
	});
}

#[test]
fn register_address_pre_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: <Test as frame_system::Config>::AccountId = AccountId::new([0; 32]);
		let blockchain = Blockchain::Rinkeby;
		let address = B("someaddressvalue").into_bounded();
		assert_ok!(Creditcoin::register_address(
			Origin::signed(acct.clone()),
			blockchain.clone(),
			address.clone(),
		));

		assert_noop!(
			Creditcoin::register_address(Origin::signed(acct.clone()), blockchain, address,),
			crate::Error::<Test>::AddressAlreadyRegistered
		);
	})
}

const ETHLESS_RESPONSES: &[u8] = include_bytes!("tests/ethlessTransfer.json");

#[test]
fn verify_ethless_transfer() {
	let (mut ext, state, _) = ExtBuilder::default().build_offchain();
	let dummy_url = "dummy";
	let tx_hash = "0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620e9";
	let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();
	{
		let mut state = state.write();
		let responses: HashMap<String, serde_json::Value> =
			serde_json::from_slice(ETHLESS_RESPONSES).unwrap();
		let get_transaction =
			pending_rpc_request("eth_getTransactionByHash", vec![tx_hash.into()], &responses);
		let get_transaction_receipt = pending_rpc_request(
			"eth_getTransactionReceipt",
			vec![tx_hash.into()],
			dummy_url,
			&responses,
		);
		let block_number = pending_rpc_request("eth_blockNumber", None, dummy_url, &responses);

		state.expect_request(get_transaction);
		state.expect_request(get_transaction_receipt);
		state.expect_request(block_number);
	}

	ext.execute_with(|| {
		let rpc_url_storage = StorageValueRef::persistent(B("rinkeby-rpc-uri"));
		rpc_url_storage.set(&dummy_url);

		let from = B("0xf04349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").into_bounded();
		let to = B("0xBBb8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").into_bounded();
		let order_id = crate::OrderId::Deal(crate::DealOrderId::dummy());
		let amount = sp_core::U256::from(53688044u64);
		let tx_id = tx_hash.as_bytes().into_bounded();

		assert_ok!(Creditcoin::verify_ethless_transfer(
			&Blockchain::Rinkeby,
			&contract,
			&from,
			&to,
			&order_id,
			&amount,
			&tx_id
		));
	});
}

#[test]
fn register_transfer_ocw() {
	let mut ext = ExtBuilder::default();
	ext.generate_authority();
	let (mut ext, state, pool) = ext.build_offchain();
	let dummy_url = "dummy";
	let tx_hash = "0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620e9";
	let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();
	{
		let mut state = state.write();
		let responses: HashMap<String, serde_json::Value> =
			serde_json::from_slice(ETHLESS_RESPONSES).unwrap();
		let get_transaction = pending_rpc_request(
			"eth_getTransactionByHash",
			vec![tx_hash.into()],
			dummy_url,
			&responses,
		);
		let get_transaction_receipt = pending_rpc_request(
			"eth_getTransactionReceipt",
			vec![tx_hash.into()],
			dummy_url,
			&responses,
		);
		let block_number = pending_rpc_request("eth_blockNumber", None, dummy_url, &responses);

		state.expect_request(get_transaction);
		state.expect_request(get_transaction_receipt);
		state.expect_request(block_number);
	}

	ext.execute_with(|| {
		let rpc_url_storage = StorageValueRef::persistent(B("rinkeby-rpc-uri"));
		rpc_url_storage.set(&dummy_url);

		let lender = AccountId::new([0; 32]);
		let debtor = AccountId::new([1; 32]);

		let loan_amount = ExternalAmount::from(53688044u64);

		let blockchain = Blockchain::Rinkeby;
		let expiration = 1000000;

		let lender_addr = B("0xf04349B4A760F5Aed02131e0dAA9bB99a1d1d1e5").into_bounded();
		let lender_address_id = crate::AddressId::new::<Test>(&blockchain, &lender_addr);
		assert_ok!(Creditcoin::register_address(
			Origin::signed(lender.clone()),
			blockchain.clone(),
			lender_addr
		));

		let debtor_addr = B("0xBBb8bbAF43fE8b9E5572B1860d5c94aC7ed87Bb9").into_bounded();
		let debtor_address_id = crate::AddressId::new::<Test>(&blockchain, &debtor_addr);
		assert_ok!(Creditcoin::register_address(
			Origin::signed(debtor.clone()),
			blockchain.clone(),
			debtor_addr
		));

		let terms = LoanTerms {
			amount: loan_amount.clone(),
			interest_rate: 0,
			maturity: 1_000_000_000_000,
		};

		let ask_guid = B("deadbeef").into_bounded();
		let ask_id = crate::AskOrderId::new::<Test>(System::block_number() + expiration, &ask_guid);
		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(lender.clone()),
			lender_address_id.clone(),
			terms.clone(),
			expiration,
			ask_guid.clone()
		));

		let bid_guid = B("beaddeef").into_bounded();
		let bid_id = crate::BidOrderId::new::<Test>(System::block_number() + expiration, &bid_guid);
		assert_ok!(Creditcoin::add_bid_order(
			Origin::signed(debtor.clone()),
			debtor_address_id.clone(),
			terms.clone(),
			expiration,
			bid_guid.clone()
		));

		let offer_id =
			crate::OfferId::new::<Test>(System::block_number() + expiration, &ask_id, &bid_id);
		assert_ok!(Creditcoin::add_offer(
			Origin::signed(lender.clone()),
			ask_id.clone(),
			bid_id.clone(),
			expiration
		));

		let deal_order_id =
			crate::DealOrderId::new::<Test>(System::block_number() + expiration, &offer_id);
		assert_ok!(Creditcoin::add_deal_order(
			Origin::signed(debtor.clone()),
			offer_id.clone(),
			expiration
		));

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(lender.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.as_bytes().into_bounded()
		));
		let expected_transfer = crate::Transfer {
			blockchain,
			kind: TransferKind::Ethless(contract.clone()),
			amount: loan_amount,
			block: System::block_number(),
			from: lender_address_id.clone(),
			to: debtor_address_id.clone(),
			order_id: OrderId::Deal(deal_order_id.clone()),
			processed: false,
			sighash: lender.clone(),
			tx: tx_hash.as_bytes().into_bounded(),
		};

		roll_by_with_ocw(1);

		let tx = pool.write().transactions.pop().unwrap();
		assert!(pool.read().transactions.is_empty());
		let verify_tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(
			verify_tx.call,
			Call::Creditcoin(crate::Call::verify_transfer { transfer: expected_transfer })
		);
	});
}

#[test]
fn add_ask_order_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let TestInfo { origin_account_id, address_id, loan_terms, blockchain } =
			prepare_test("myacct");
		let guid = B("testguid").into_bounded();
		let expiration_block = 1_000;

		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(origin_account_id.clone()),
			address_id.clone(),
			loan_terms.clone().into(),
			expiration_block.clone(),
			guid.clone()
		));

		let ask_order_id = crate::AskOrderId::new::<Test>(expiration_block.clone(), &guid);
		let new_ask_order = Creditcoin::ask_orders(expiration_block.clone(), ask_order_id.hash());
		let block = new_ask_order.clone().unwrap().block;

		let ask_order = crate::AskOrder {
			blockchain,
			lender_address_id: address_id,
			terms: loan_terms.into(),
			expiration_block,
			block,
			lender: origin_account_id,
		};

		assert_eq!(new_ask_order, Some(ask_order));
	});
}

#[test]
fn add_ask_order_pre_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let TestInfo { origin_account_id, address_id, loan_terms, .. } = prepare_test("myacct");

		let guid = B("testguid").into_bounded();
		let expiration_block = 1_000;

		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(origin_account_id.clone()),
			address_id.clone(),
			loan_terms.clone().into(),
			expiration_block.clone(),
			guid.clone()
		));

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(origin_account_id.clone()),
				address_id,
				loan_terms.into(),
				expiration_block,
				guid
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}
