use crate::{
	mock::*, AddressId, AskOrder, AskOrderId, BidOrder, BidOrderId, Blockchain, DealOrder,
	DealOrderId, ExternalAmount, Guid, Id, LegacySighash, LoanTerms, Offer, OfferId, OrderId,
	TransferKind,
};
use bstr::B;
use codec::{Decode, Encode};
use ethereum_types::H256;
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};

use sp_runtime::{offchain::storage::StorageValueRef, traits::IdentifyAccount, MultiSigner};
use std::{collections::HashMap, convert::TryFrom};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisteredAddress {
	address_id: AddressId<H256>,
	account_id: AccountId,
}
impl RegisteredAddress {
	pub fn new(i: u8) -> RegisteredAddress {
		let account_id = AccountId::new([i; 32]);
		let address = i.to_string().as_bytes().into_bounded();
		let blockchain = Blockchain::Rinkeby;
		let address_id = AddressId::new::<Test>(&blockchain, &address);
		assert_ok!(Creditcoin::register_address(
			Origin::signed(account_id.clone()),
			blockchain,
			address
		));
		RegisteredAddress { account_id, address_id }
	}
}

type TestAskOrderId = AskOrderId<u64, H256>;
type TestBidOrderId = BidOrderId<u64, H256>;
type TestOfferId = OfferId<u64, H256>;
type TestDealOrderId = DealOrderId<u64, H256>;
type TestAskOrder = (AskOrder<AccountId, u64, H256, u64>, TestAskOrderId);
type TestBidOrder = (BidOrder<AccountId, u64, H256, u64>, TestBidOrderId);
type TestOffer = (Offer<AccountId, u64, H256>, TestOfferId);
type TestDealOrder = (DealOrder<AccountId, u64, H256, u64>, TestDealOrderId);

#[derive(Clone, Debug)]
pub struct TestInfo {
	blockchain: Blockchain,
	loan_terms: LoanTerms<u64>,
	lender: RegisteredAddress,
	borrower: RegisteredAddress,
	ask_guid: Guid,
	bid_guid: Guid,
	expiration_block: u64,
}

impl TestInfo {
	pub fn new_defaults() -> TestInfo {
		let lender = RegisteredAddress::new(0);
		let borrower = RegisteredAddress::new(1);
		let blockchain = Blockchain::Rinkeby;
		let loan_terms = LoanTerms {
			amount: ExternalAmount::from(1_000_0000u64),
			interest_rate: 0,
			maturity: 1_000_000,
		};
		let ask_guid = "ask_guid".as_bytes().into_bounded();
		let bid_guid = "bid_guid".as_bytes().into_bounded();
		let expiration_block = 1_000;
		TestInfo { blockchain, lender, borrower, loan_terms, ask_guid, bid_guid, expiration_block }
	}

	pub fn create_ask_order(&self) -> TestAskOrder {
		let TestInfo { lender, loan_terms, expiration_block, ask_guid, .. } = self;
		let RegisteredAddress { address_id, account_id } = lender;

		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(account_id.clone()),
			address_id.clone(),
			loan_terms.clone().into(),
			expiration_block.clone(),
			ask_guid.clone()
		));

		let ask_order_id = AskOrderId::new::<Test>(expiration_block.clone(), &ask_guid);

		(Creditcoin::ask_orders(expiration_block, ask_order_id.hash()).unwrap(), ask_order_id)
	}

	pub fn create_bid_order(&self) -> TestBidOrder {
		let TestInfo { borrower, loan_terms, expiration_block, bid_guid, .. } = self;
		let RegisteredAddress { address_id, account_id } = borrower;

		assert_ok!(Creditcoin::add_bid_order(
			Origin::signed(account_id.clone()),
			address_id.clone(),
			loan_terms.clone().into(),
			expiration_block.clone(),
			bid_guid.clone()
		));

		let bid_order_id = BidOrderId::new::<Test>(expiration_block.clone(), &bid_guid);
		(
			Creditcoin::bid_orders(expiration_block.clone(), bid_order_id.hash()).unwrap(),
			bid_order_id,
		)
	}

	pub fn create_offer(&self) -> TestOffer {
		let RegisteredAddress { account_id, .. } = &self.lender;

		let (_, ask_order_id) = self.create_ask_order();
		let (_, bid_order_id) = self.create_bid_order();
		let expiration_block = 1_000;
		assert_ok!(Creditcoin::add_offer(
			Origin::signed(account_id.clone()),
			ask_order_id.clone(),
			bid_order_id.clone(),
			expiration_block.clone(),
		));
		let offer_id = OfferId::new::<Test>(expiration_block.clone(), &ask_order_id, &bid_order_id);
		(Creditcoin::offers(expiration_block, offer_id.hash()).unwrap(), offer_id)
	}

	pub fn create_deal_order(&self) -> TestDealOrder {
		let RegisteredAddress { account_id, .. } = &self.borrower;
		let (_, offer_id) = self.create_offer();
		let expiration_block = 1_000;

		assert_ok!(Creditcoin::add_deal_order(
			Origin::signed(account_id.clone()),
			offer_id.clone(),
			expiration_block.clone(),
		));

		let deal_order_id = DealOrderId::new::<Test>(expiration_block.clone(), &offer_id);

		(Creditcoin::deal_orders(expiration_block, deal_order_id.hash()).unwrap(), deal_order_id)
	}
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
	let (mut ext, _, _) = ExtBuilder::default().build_offchain();

	let (ask_order, ask_guid) = ext.execute_with(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, blockchain, ask_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;
		let (ask_order, _) = test_info.create_ask_order();
		let AskOrder { block, expiration_block, .. } = ask_order.clone();

		let new_ask_order = crate::AskOrder {
			blockchain,
			lender_address_id: address_id,
			terms: loan_terms.into(),
			expiration_block,
			block,
			lender: account_id,
		};

		assert_eq!(ask_order, new_ask_order);
		(ask_order, ask_guid)
	});

	ext.persist_offchain_overlay();
	assert_eq!(ext.offchain_db().get(&ask_guid).unwrap(), ask_order.encode());
}

#[test]
fn add_ask_order_used_guid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, ask_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _ask_order = test_info.create_ask_order();
		let expiration_block = 1_500;

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id.clone()),
				address_id.clone(),
				loan_terms.into(),
				expiration_block.clone(),
				ask_guid
			),
			crate::Error::<Test>::GuidAlreadyUsed
		);
	});
}

#[test]
fn add_ask_order_pre_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, ask_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let (ask_order, _) = test_info.create_ask_order();
		let AskOrder { expiration_block, .. } = ask_order.clone();

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id.clone()),
				address_id.clone(),
				loan_terms.into(),
				expiration_block.clone(),
				ask_guid
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn add_bid_order_basic() {
	let (mut ext, _, _) = ExtBuilder::default().build_offchain();

	let (bid_order, bid_guid) = ext.execute_with(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { borrower, loan_terms, blockchain, bid_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = borrower;

		let (bid_order, _) = test_info.create_bid_order();
		let BidOrder { expiration_block, block, .. } = bid_order.clone();

		let new_bid_order = crate::BidOrder {
			blockchain,
			borrower_address_id: address_id,
			terms: loan_terms.into(),
			expiration_block,
			block,
			borrower: account_id,
		};

		assert_eq!(new_bid_order, bid_order);
		(bid_order, bid_guid)
	});

	ext.persist_offchain_overlay();
	assert_eq!(ext.offchain_db().get(&bid_guid).unwrap(), bid_order.encode());
}

#[test]
fn add_bid_order_used_guid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, bid_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _bid_order = test_info.create_bid_order();
		let expiration_block = 1_500;

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id.clone()),
				address_id.clone(),
				loan_terms.into(),
				expiration_block.clone(),
				bid_guid
			),
			crate::Error::<Test>::GuidAlreadyUsed
		);
	});
}

#[test]
fn add_bid_order_pre_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { borrower, loan_terms, bid_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = borrower;

		let (bid_order, _) = test_info.create_bid_order();
		let BidOrder { expiration_block, .. } = bid_order.clone();

		assert_noop!(
			Creditcoin::add_bid_order(
				Origin::signed(account_id.clone()),
				address_id.clone(),
				loan_terms.into(),
				expiration_block.clone(),
				bid_guid
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn add_offer_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (offer, _) = test_info.create_offer();
		let Offer { blockchain, expiration_block, block, ask_id, bid_id, lender, .. } =
			offer.clone();

		let new_offer = Offer { blockchain, expiration_block, block, ask_id, bid_id, lender };

		assert_eq!(new_offer, offer);
	});
}

#[test]
fn add_offer_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (offer, _) = test_info.create_offer();
		let Offer { expiration_block, ask_id, bid_id, lender, .. } = offer.clone();

		assert_noop!(
			Creditcoin::add_offer(Origin::signed(lender), ask_id, bid_id, expiration_block,),
			crate::Error::<Test>::DuplicateOffer
		);
	})
}

#[test]
fn add_deal_order_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, _) = test_info.create_deal_order();
		let DealOrder {
			blockchain,
			expiration_block,
			lender_address_id,
			borrower_address_id,
			terms,
			timestamp,
			borrower,
			offer_id,
			..
		} = deal_order.clone();

		let new_deal_order = DealOrder {
			blockchain,
			offer_id,
			lender_address_id,
			borrower_address_id,
			expiration_block,
			terms,
			timestamp,
			borrower,
			funding_transfer_id: None,
			lock: None,
			repayment_transfer_id: None,
		};

		assert_eq!(new_deal_order, deal_order);
	})
}

#[test]
fn add_deal_order_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, _) = test_info.create_deal_order();
		let DealOrder { expiration_block, borrower, offer_id, .. } = deal_order.clone();

		assert_noop!(
			Creditcoin::add_deal_order(Origin::signed(borrower), offer_id, expiration_block),
			crate::Error::<Test>::DuplicateDealOrder
		);
	});
}

#[test]
fn claim_legacy_wallet_works() {
	let keeper = AccountId::from([0; 32]);
	let legacy_amount = 1000000;
	let sighash =
		LegacySighash::try_from("f0bdc887e4d7928623081f30b1bc87b9e4443cca6b52c4364ce578cb6bf4")
			.unwrap();
	let pubkey = sp_core::ecdsa::Public::from_full(
		&hex::decode("0399d6e7c784494fd7edc26fc9ca460a68c97cc64c49c85dfbb68148f0607893bf").unwrap(),
	)
	.unwrap();
	let claimer = MultiSigner::from(pubkey.clone()).into_account();

	let mut ext = ExtBuilder::default();
	ext.fund(keeper.clone(), legacy_amount)
		.legacy_balance_keeper(keeper)
		.legacy_wallets(vec![(sighash, legacy_amount)]);

	ext.build_and_execute(|| {
		assert_ok!(Creditcoin::claim_legacy_wallet(Origin::signed(claimer.clone()), pubkey));

		assert_eq!(frame_system::pallet::Account::<Test>::get(&claimer).data.free, 1000000);
	});
}
