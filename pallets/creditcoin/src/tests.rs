use crate::{
	mock::*, types::DoubleMapExt, AddressId, AskOrder, AskOrderId, Authorities, BidOrder,
	BidOrderId, Blockchain, DealOrder, DealOrderId, DealOrders, ExternalAddress, ExternalAmount,
	Guid, Id, LegacySighash, LoanTerms, Offer, OfferId, OrderId, TransferId, TransferKind,
	Transfers,
};
use bstr::B;
use codec::{Decode, Encode};
use ethereum_types::H256;
use frame_support::{assert_noop, assert_ok, traits::Get, BoundedVec};
use frame_system::RawOrigin;

use sp_core::Pair;
use sp_runtime::{
	offchain::storage::StorageValueRef,
	traits::{BadOrigin, IdentifyAccount},
	MultiSigner,
};
use std::{
	collections::HashMap,
	convert::{TryFrom, TryInto},
};

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
	pub fn from_pubkey(
		public_key: impl Into<MultiSigner>,
		blockchain: Blockchain,
	) -> RegisteredAddress {
		let account_id = public_key.into().into_account();
		let address = "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB".as_bytes().into_bounded();
		let address_id = AddressId::new::<Test>(&blockchain, &address);
		assert_ok!(Creditcoin::register_address(
			Origin::signed(account_id.clone()),
			blockchain,
			address
		));
		RegisteredAddress { account_id, address_id }
	}
	pub fn new(address: ExternalAddress, i: u8, blockchain: Blockchain) -> RegisteredAddress {
		let account_id = AccountId::new([i; 32]);
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
		let address1 = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".as_bytes().into_bounded();
		let address2 = "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359".as_bytes().into_bounded();
		let lender = RegisteredAddress::new(address1, 1, Blockchain::Rinkeby);
		let borrower = RegisteredAddress::new(address2, 2, Blockchain::Rinkeby);
		let blockchain = Blockchain::Rinkeby;

		let loan_terms = LoanTerms {
			amount: ExternalAmount::from(1_000_0000u64),
			interest_rate: 1,
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

	pub fn get_register_deal_msg(&self) -> Vec<u8> {
		self.expiration_block
			.encode()
			.into_iter()
			.chain(self.ask_guid.encode())
			.chain(self.bid_guid.encode())
			.chain(self.loan_terms.encode())
			.collect::<Vec<u8>>()
	}
}

#[test]
fn register_address_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);
		let blockchain = Blockchain::Rinkeby;
		let value = B("0x52908400098527886E0F7030069857D2E4169EE7").into_bounded();
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
		let address = B("0x52908400098527886E0F7030069857D2E4169EE7").into_bounded();
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
			terms: loan_terms.try_into().unwrap(),
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
#[should_panic]
fn add_add_ask_order_rejects_zero_maturity() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo {
			loan_terms: LoanTerms { amount: 0u64.into(), interest_rate: 0, maturity: 0 },
			..TestInfo::new_defaults()
		};
		let _ = test_info.create_ask_order();
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
			terms: loan_terms.try_into().unwrap(),
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
#[should_panic]
fn add_bid_ask_order_rejects_zero_maturity() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo {
			loan_terms: LoanTerms { amount: 0u64.into(), interest_rate: 0, maturity: 0 },
			..TestInfo::new_defaults()
		};
		let _ = test_info.create_bid_order();
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
fn lock_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, deal_order_id) = test_info.create_deal_order();

		assert_noop!(Creditcoin::lock_deal_order(Origin::none(), deal_order_id), BadOrigin);
	});
}

#[test]
fn lock_deal_order_should_error_for_non_existent_deal_order() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, _) = test_info.create_deal_order();
		let DealOrder { borrower, offer_id, .. } = deal_order.clone();
		// expiration_block set to 0
		let deal_order_id = DealOrderId::new::<Test>(0, &offer_id);

		assert_noop!(
			Creditcoin::lock_deal_order(Origin::signed(borrower), deal_order_id),
			crate::Error::<Test>::NonExistentDealOrder
		);
	});
}

#[test]
fn lock_deal_order_should_error_when_not_funded() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, deal_order_id) = test_info.create_deal_order();

		assert_noop!(
			Creditcoin::lock_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id
			),
			crate::Error::<Test>::DealNotFunded
		);
	});
}

#[test]
fn lock_deal_order_should_fail_for_non_borrower() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().funding_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"12345678"));
			},
		);

		assert_noop!(
			Creditcoin::lock_deal_order(Origin::signed(test_info.lender.account_id), deal_order_id),
			crate::Error::<Test>::NotBorrower
		);
	});
}

#[test]
fn lock_deal_order_should_fail_if_already_locked() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().funding_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"12345678"));
			},
		);

		assert_ok!(Creditcoin::lock_deal_order(
			Origin::signed(test_info.borrower.account_id.clone()),
			deal_order_id.clone()
		));

		assert_noop!(
			Creditcoin::lock_deal_order(
				Origin::signed(test_info.borrower.account_id.clone()),
				deal_order_id.clone()
			),
			crate::Error::<Test>::DealOrderAlreadyLocked
		);
	});
}

#[test]
fn lock_deal_order_locks_by_borrower() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().funding_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"12345678"));
			},
		);

		assert_ok!(Creditcoin::lock_deal_order(
			Origin::signed(test_info.borrower.account_id.clone()),
			deal_order_id.clone()
		));
		let locked_deal_order =
			Creditcoin::deal_orders(deal_order.expiration_block, deal_order_id.hash()).unwrap();
		assert_eq!(locked_deal_order.lock, Some(test_info.borrower.account_id.clone()));
	});
}

#[test]
fn fund_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(
			Creditcoin::fund_deal_order(Origin::none(), deal_order_id, transfer_id),
			BadOrigin
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_address_not_registered() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with an address that isn't registered
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				let blockchain = Blockchain::Rinkeby;

				deal_order_storage.as_mut().unwrap().lender_address_id =
					AddressId::new::<Test>(&blockchain, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::NonExistentAddress
		);
	});
}

#[test]
fn fund_deal_order_should_error_for_non_lender() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::NotLender
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_timestamp_is_in_the_future() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with a timestamp in the future
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().timestamp = Creditcoin::timestamp() + 99999;
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::MalformedDealOrder
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_deal_is_funded() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate a funded deal
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().funding_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"12345678"));
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::DealOrderAlreadyFunded
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_deal_has_expired() {
	ExtBuilder::default().build_and_execute(|| {
		roll_to(4); // advance head so we have something to compare to

		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate an expired deal by setting expiration_block < head
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().expiration_block = 0;
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::DealOrderExpired
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_order_id_doesnt_match_deal_order_id() {
	ExtBuilder::default().build_and_execute(|| {
		// this is the primary deal_order
		let test_info = TestInfo::new_defaults();
		let (_, deal_order_id) = test_info.create_deal_order();
		let address1 = "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB".as_bytes().into_bounded();
		let address2 = "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb".as_bytes().into_bounded();

		// this is a deal_order from another person
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new(address1, 100, Blockchain::Rinkeby),
			borrower: RegisteredAddress::new(address2, 200, Blockchain::Rinkeby),
			blockchain: Blockchain::Rinkeby,
			loan_terms: LoanTerms {
				amount: 2_000_000u64.into(),
				interest_rate: 0,
				maturity: 1_000_000,
			},
			ask_guid: "second-ask-guid".as_bytes().into_bounded(),
			bid_guid: "second-bid-guid".as_bytes().into_bounded(),
			expiration_block: 3_333,
		};

		let (bogus_deal_order, bogus_deal_order_id) = second_test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(second_test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(bogus_deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&bogus_deal_order.blockchain, &tx_hash.clone());

		// try funding DealOrder from Person1 with the transfer from Person2,
		// which points to a different order_id
		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id.clone()),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_amount_doesnt_match() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify deal amount in order to cause transfer mismatch
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				// note: the transfer above has amount of 0 b/c it is an exemption!
				deal_order_storage.as_mut().unwrap().terms.amount = ExternalAmount::from(4444);
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id.clone()),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_sighash_doesnt_match_lender() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.sighash = AccountId::new([4; 32]);
		});

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id.clone()),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_has_been_processed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause an error
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.processed = true;
		});

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id.clone()),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferAlreadyProcessed
		);
	});
}

#[test]
fn fund_deal_order_works() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer b/c amount above is 0
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			transfer_storage.as_mut().unwrap().amount = deal_order.terms.amount;
		});

		assert_ok!(Creditcoin::fund_deal_order(
			Origin::signed(test_info.lender.account_id.clone()),
			deal_order_id,
			transfer_id
		));
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

#[test]
fn add_authority_errors_for_non_root() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);

		assert_noop!(
			Creditcoin::add_authority(Origin::signed(acct.clone()), acct.clone()),
			BadOrigin
		);
	});
}

#[test]
fn add_authority_works_for_root() {
	ExtBuilder::default().build_and_execute(|| {
		let root = RawOrigin::Root;
		let acct: AccountId = AccountId::new([0; 32]);

		assert_ok!(Creditcoin::add_authority(
			crate::mock::Origin::from(root.clone()),
			acct.clone(),
		));

		let value = Authorities::<Test>::take(acct.clone());
		assert_eq!(value, Some(()))
	});
}

#[test]
fn register_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::none(),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			BadOrigin
		);
	});
}

#[test]
fn register_deal_order_should_error_when_signature_is_invalid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let (wrong_key, _) = sp_core::ecdsa::Pair::generate();
		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				wrong_key.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::InvalidSignature
		);
	});
}

#[test]
fn register_deal_order_should_error_when_borrower_address_doesnt_match_signature() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id.clone(),
				test_info.lender.address_id.clone(), // <-- bogus address
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::NotAddressOwner
		);
	});
}

#[test]
fn register_deal_order_should_error_when_lender_address_doesnt_match_sender() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new(
				"0x8617E340B3D01FA5F11F306F4090FD50E238070D".as_bytes().into_bounded(),
				111,
				Blockchain::Rinkeby,
			),
			..test_info.clone()
		};
		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				second_test_info.lender.address_id, // <-- bogus
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::NotAddressOwner
		);
	});
}

#[test]
fn register_deal_order_should_error_when_lender_and_borrower_are_on_different_chains() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			lender: RegisteredAddress::new(
				"0x8617E340B3D01FA5F11F306F4090FD50E238070D".as_bytes().into_bounded(),
				111,
				Blockchain::Ethereum,
			),
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::AddressPlatformMismatch
		);
	});
}

#[test]
fn register_deal_order_should_error_when_ask_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};
		// create AskOrder which will use-up the default ID
		test_info.create_ask_order();

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn register_deal_order_should_error_when_bid_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};
		// create BidOrder which will use-up the default ID
		test_info.create_bid_order();

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn register_deal_order_should_error_when_offer_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		// create Offer w/o creating AskOrder & BidOrder to avoid
		// erroring out when checking for their existence
		let ask_order_id = AskOrderId::new::<Test>(test_info.expiration_block, &test_info.ask_guid);
		let bid_order_id = BidOrderId::new::<Test>(test_info.expiration_block, &test_info.bid_guid);
		let offer_id =
			OfferId::new::<Test>(test_info.expiration_block, &ask_order_id, &bid_order_id);
		let current_block = Creditcoin::block_number();
		let offer = Offer {
			ask_id: ask_order_id.clone(),
			bid_id: bid_order_id.clone(),
			block: current_block,
			blockchain: test_info.blockchain.clone(),
			expiration_block: test_info.expiration_block.clone(),
			lender: test_info.lender.account_id.clone(),
		};
		// insert this offer into storage which will use-up the ID
		// register_deal_order() will reconstruct the same ID later
		crate::Offers::<Test>::insert_id(offer_id, offer);

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::DuplicateOffer
		);
	});
}

#[test]
fn register_deal_order_should_error_when_deal_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		// create DealOrder w/o creating AskOrder, BidOrder & Offer to avoid
		// erroring out when checking for their existence
		let ask_order_id = AskOrderId::new::<Test>(test_info.expiration_block, &test_info.ask_guid);
		let bid_order_id = BidOrderId::new::<Test>(test_info.expiration_block, &test_info.bid_guid);
		let offer_id =
			OfferId::new::<Test>(test_info.expiration_block, &ask_order_id, &bid_order_id);
		let deal_order_id = DealOrderId::new::<Test>(test_info.expiration_block, &offer_id);

		let deal_order = DealOrder {
			blockchain: test_info.blockchain,
			offer_id,
			lender_address_id: test_info.lender.address_id.clone(),
			borrower_address_id: test_info.borrower.address_id.clone(),
			terms: test_info.loan_terms.clone(),
			expiration_block: test_info.expiration_block,
			timestamp: Creditcoin::timestamp(),
			borrower: test_info.borrower.account_id,
			funding_transfer_id: None,
			lock: None,
			repayment_transfer_id: None,
		};

		// insert this DealOrder into storage which will use-up the ID
		// register_deal_order() will reconstruct the same ID later
		crate::DealOrders::<Test>::insert_id(deal_order_id, deal_order);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				signature.into(),
			),
			crate::Error::<Test>::DuplicateDealOrder
		);
	});
}

#[test]
fn register_deal_order_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (key_pair, _) = sp_core::ecdsa::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			key_pair.public().into(),
			signature.into(),
		));

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event4 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(event4, crate::mock::Event::Creditcoin(crate::Event::DealOrderAdded(..))));

		let event3 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(event3, crate::mock::Event::Creditcoin(crate::Event::OfferAdded(..))));

		let event2 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(event2, crate::mock::Event::Creditcoin(crate::Event::BidOrderAdded(..))));

		let event1 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(event1, crate::mock::Event::Creditcoin(crate::Event::AskOrderAdded(..))));
	});
}

#[test]
fn register_deal_order_accepts_sr25519() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (key_pair, _) = sp_core::sr25519::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			key_pair.public().into(),
			signature.into(),
		));
	});
}

#[test]
fn register_deal_order_accepts_ed25519() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (key_pair, _) = sp_core::ed25519::Pair::generate();
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(key_pair.public(), Blockchain::Rinkeby),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let signature = key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			key_pair.public().into(),
			signature.into(),
		));
	});
}

#[test]
fn close_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(
			Creditcoin::close_deal_order(Origin::none(), deal_order_id, transfer_id,),
			BadOrigin
		);
	});
}

#[test]
fn close_deal_order_should_error_when_borrower_address_is_not_registered() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with an address that isn't registered
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				let blockchain = Blockchain::Rinkeby;

				deal_order_storage.as_mut().unwrap().borrower_address_id =
					AddressId::new::<Test>(&blockchain, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
			},
		);

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::NonExistentAddress
		);
	});
}

#[test]
fn close_deal_order_should_error_when_not_signed_by_borrower() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(
			Creditcoin::close_deal_order(
				// bogus signature --------v
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::NotBorrower
		);
	});
}

#[test]
fn close_deal_order_should_error_when_deal_timestamp_is_in_the_future() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with a timestamp in the future
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().timestamp = Creditcoin::timestamp() + 99999;
			},
		);

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::MalformedDealOrder
		);
	});
}

#[test]
fn close_deal_order_should_error_when_deal_order_has_already_been_repaid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate DealOrder which has been repaid
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().repayment_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"4444"));
			},
		);

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::DealOrderAlreadyClosed
		);
	});
}

#[test]
fn close_deal_order_should_error_when_deal_isnt_locked() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal which is not locked
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock = None;
			},
		);

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::DealOrderMustBeLocked
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_order_id_doesnt_match_deal_order_id() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (_, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		let address1 = "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB".as_bytes().into_bounded();
		let address2 = "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb".as_bytes().into_bounded();
		// this is a deal_order from another person
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new(address1, 100, Blockchain::Rinkeby),
			borrower: RegisteredAddress::new(address2, 200, Blockchain::Rinkeby),
			blockchain: Blockchain::Rinkeby,
			loan_terms: LoanTerms {
				amount: 2_000_000u64.into(),
				interest_rate: 0,
				maturity: 1_000_000,
			},
			ask_guid: "second-ask-guid".as_bytes().into_bounded(),
			bid_guid: "second-bid-guid".as_bytes().into_bounded(),
			expiration_block: 3_333,
		};

		let (bogus_deal_order, bogus_deal_order_id) = second_test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(second_test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(bogus_deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&bogus_deal_order.blockchain, &tx_hash.clone());

		// Person1 tries closing the deal by using the transfer made by Person2
		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_block_is_greater_than_current_block() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.block = Creditcoin::block_number() + 1;
		});

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::MalformedTransfer
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_sighash_doesnt_match_borrower() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.sighash = AccountId::new([44; 32]);
		});

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_has_already_been_processed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.processed = true;
		});

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferAlreadyProcessed
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_amount_is_not_enough() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// NOTE: amount not enough to repay loan + interest
			ts.amount = ExternalAmount::from(1u64);
		});

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferAmountInsufficient
		);
	});
}

#[test]
fn close_deal_order_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.borrower.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer to make sure we have transfered enough funds
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();

			let interest = deal_order.terms.calc_interest();
			ts.amount = ExternalAmount::from(deal_order.terms.amount + interest + 1u64);
		});

		assert_ok!(Creditcoin::close_deal_order(
			Origin::signed(test_info.borrower.account_id),
			deal_order_id.clone(),
			transfer_id.clone(),
		));

		// assert field values were updated in storage
		let saved_deal_order = DealOrders::<Test>::try_get_id(&deal_order_id).unwrap();
		assert_eq!(saved_deal_order.repayment_transfer_id, Some(transfer_id.clone()));

		let saved_transfer = Transfers::<Test>::try_get(&transfer_id).unwrap();
		assert_eq!(saved_transfer.processed, true);

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event2 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event2,
			crate::mock::Event::Creditcoin(crate::Event::TransferProcessed(..))
		));

		let event1 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event1,
			crate::mock::Event::Creditcoin(crate::Event::DealOrderClosed(..))
		));
	});
}

#[test]
fn exempt_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(Creditcoin::exempt(Origin::none(), deal_order_id, transfer_id,), BadOrigin);
	});
}

#[test]
fn exempt_should_error_when_deal_order_has_already_been_repaid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate DealOrder which has been repaid
		crate::DealOrders::<Test>::mutate(
			&deal_order_id.expiration(),
			&deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().repayment_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"4444"));
			},
		);

		assert_noop!(
			Creditcoin::exempt(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::DealOrderAlreadyClosed
		);
	});
}

#[test]
fn exempt_should_error_for_non_lender() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		assert_noop!(
			Creditcoin::exempt(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::NotLender
		);
	});
}

#[test]
fn exempt_should_error_when_transfer_order_id_doesnt_match_deal_order_id() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (_, deal_order_id) = test_info.create_deal_order();

		let address1 = "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB".as_bytes().into_bounded();
		let address2 = "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb".as_bytes().into_bounded();
		// this is a deal_order from another person
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new(address1, 100, Blockchain::Rinkeby),
			borrower: RegisteredAddress::new(address2, 200, Blockchain::Rinkeby),
			blockchain: Blockchain::Rinkeby,
			loan_terms: LoanTerms {
				amount: 2_000_000u64.into(),
				interest_rate: 0,
				maturity: 1_000_000,
			},
			ask_guid: "second-ask-guid".as_bytes().into_bounded(),
			bid_guid: "second-bid-guid".as_bytes().into_bounded(),
			expiration_block: 3_333,
		};

		let (bogus_deal_order, bogus_deal_order_id) = second_test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(second_test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(bogus_deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&bogus_deal_order.blockchain, &tx_hash.clone());

		assert_noop!(
			Creditcoin::exempt(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferMismatch
		);
	});
}

#[test]
fn exempt_should_error_when_transfer_has_been_processed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		// modify transfer in order to cause an error
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.processed = true;
		});

		assert_noop!(
			Creditcoin::exempt(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferAlreadyProcessed
		);
	});
}

#[test]
fn exempt_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();
		let (deal_order, deal_order_id) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract.clone()),
			0u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, &tx_hash.clone());

		assert_ok!(Creditcoin::exempt(
			Origin::signed(test_info.lender.account_id),
			deal_order_id.clone(),
			transfer_id.clone(),
		));

		// assert field values were updated in storage
		let saved_deal_order = DealOrders::<Test>::try_get_id(&deal_order_id).unwrap();
		assert_eq!(saved_deal_order.repayment_transfer_id, Some(transfer_id.clone()));

		let saved_transfer = Transfers::<Test>::try_get(&transfer_id).unwrap();
		assert_eq!(saved_transfer.processed, true);

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert_eq!(
			event,
			crate::mock::Event::Creditcoin(crate::Event::LoanExempted(deal_order_id, transfer_id))
		);
	});
}
