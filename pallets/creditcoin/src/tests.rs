use crate::{
	helpers::{
		extensions::{HexToAddress, IntoBounded},
		non_paying_error, EVMAddress, PublicToAddress,
	},
	mock::{RuntimeOrigin as Origin, *},
	types::{DoubleMapExt, OwnershipProof},
	AddressId, AskOrder, AskOrderId, BidOrder, BidOrderId, Blockchain, DealOrder, DealOrderId,
	DealOrders, Duration, ExternalAddress, ExternalAmount, Guid, Id, LegacySighash, LoanTerms,
	Offer, OfferId, OrderId, Transfer, TransferId, TransferKind, Transfers, WeightInfo,
};
use assert_matches::assert_matches;
use bstr::B;
use ethereum_types::{BigEndianHash, H256, U256};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use frame_system::RawOrigin;
use pallet_offchain_task_scheduler::authority::AuthorityController;
use parity_scale_codec::Encode;
use sp_core::Pair;
use sp_runtime::{
	offchain::storage::StorageValueRef,
	traits::{BadOrigin, IdentifyAccount},
	AccountId32, MultiSigner,
};
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisteredAddress {
	pub(crate) address_id: AddressId<H256>,
	pub(crate) account_id: AccountId,
}
impl RegisteredAddress {
	pub fn from_pubkey_distinct_owner(
		owners_account_id: AccountId,
		blockchain: Blockchain,
		address_key: impl Into<MultiSigner>,
		signature: sp_core::ecdsa::Signature,
	) -> RegisteredAddress {
		let signer = address_key.into();
		let address = if let MultiSigner::Ecdsa(pkey) = signer {
			EVMAddress::from_public(&pkey)
		} else {
			unimplemented!();
		};

		let address_id = AddressId::new::<Test>(&blockchain, &address);

		assert_ok!(Creditcoin::register_address(
			Origin::signed(owners_account_id.clone()),
			blockchain,
			address,
			signature
		));
		RegisteredAddress { account_id: owners_account_id, address_id }
	}
	pub fn from_pubkey(
		public_key: impl Into<MultiSigner>,
		blockchain: Blockchain,
		signature: sp_core::ecdsa::Signature,
	) -> RegisteredAddress {
		let signer = public_key.into();
		let address = if let MultiSigner::Ecdsa(pkey) = signer {
			EVMAddress::from_public(&pkey)
		} else {
			unimplemented!();
		};
		let account_id = signer.into_account();
		let address_id = AddressId::new::<Test>(&blockchain, &address);

		assert_ok!(Creditcoin::register_address(
			Origin::signed(account_id.clone()),
			blockchain,
			address,
			signature
		));
		RegisteredAddress { account_id, address_id }
	}
	pub fn new(seed: &str, blockchain: Blockchain) -> RegisteredAddress {
		let (who, address, ownership_proof, _) = generate_address_with_proof(seed);
		let address_id = AddressId::new::<Test>(&blockchain, &address);
		assert_ok!(Creditcoin::register_address(
			Origin::signed(who.clone()),
			blockchain,
			address,
			ownership_proof
		));

		RegisteredAddress { account_id: who, address_id }
	}
}

pub(crate) fn generate_keypair_from_seed(seed: &str) -> sp_core::ecdsa::Pair {
	let seed = seed.bytes().cycle().take(32).collect::<Vec<_>>();
	sp_core::ecdsa::Pair::from_seed_slice(seed.as_slice()).unwrap()
}

/// Generates an external EVM address from an ecdsa keypair
pub(crate) fn external_address_from_keypair(key_pair: sp_core::ecdsa::Pair) -> ExternalAddress {
	EVMAddress::from_public(&key_pair.public())
}

/// Generates an account from an ecdsa keypair
pub(crate) fn account_from_keypair(key_pair: sp_core::ecdsa::Pair) -> AccountId {
	let signer: MultiSigner = key_pair.public().into();
	signer.into_account()
}

/// Generates proof of ownership for given account and external address
pub(crate) fn build_proof_of_ownership(
	who: AccountId32,
	external_keypair: sp_core::ecdsa::Pair,
) -> sp_core::ecdsa::Signature {
	let message = get_register_address_message(who);
	external_keypair.sign(message.as_slice())
}

/// Generates an account, an external address, and proof of account ownership
/// using the **same keypair** for the external address, and cc account.
pub(crate) fn generate_address_with_proof(
	seed: &str,
) -> (AccountId, ExternalAddress, sp_core::ecdsa::Signature, sp_core::ecdsa::Pair) {
	let key_pair = generate_keypair_from_seed(seed);
	let external_address = external_address_from_keypair(key_pair.clone());
	let who = account_from_keypair(key_pair.clone());
	let ownership_proof = build_proof_of_ownership(who.clone(), key_pair.clone());
	(who, external_address, ownership_proof, key_pair)
}

type TestAskOrder = (AskOrderId<BlockNumber, Hash>, AskOrder<AccountId, BlockNumber, Hash>);
type TestBidOrder = (BidOrderId<BlockNumber, Hash>, BidOrder<AccountId, BlockNumber, Hash>);
type TestOffer = (OfferId<BlockNumber, Hash>, Offer<AccountId, BlockNumber, Hash>);
type TestDealOrderId = DealOrderId<BlockNumber, Hash>;
type TestDealOrder =
	(DealOrderId<BlockNumber, Hash>, DealOrder<AccountId, BlockNumber, Hash, Moment>);
pub(crate) type TestTransfer = (TransferId<Hash>, Transfer<AccountId, BlockNumber, Hash, Moment>);

#[derive(Clone, Debug)]
pub struct TestInfo {
	pub(crate) blockchain: Blockchain,
	pub(crate) loan_terms: LoanTerms,
	pub(crate) lender: RegisteredAddress,
	pub(crate) borrower: RegisteredAddress,
	pub(crate) ask_guid: Guid,
	pub(crate) bid_guid: Guid,
	pub(crate) expiration_block: u64,
}

impl Default for TestInfo {
	fn default() -> Self {
		let lender = RegisteredAddress::new("lender", Blockchain::Rinkeby);
		let borrower = RegisteredAddress::new("borrower", Blockchain::Rinkeby);
		let blockchain = Blockchain::Rinkeby;

		let loan_terms =
			LoanTerms { amount: ExternalAmount::from(10_000_000_u64), ..Default::default() };

		let ask_guid = "ask_guid".as_bytes().into_bounded();
		let bid_guid = "bid_guid".as_bytes().into_bounded();
		let expiration_block = 1_000;
		TestInfo { blockchain, lender, borrower, loan_terms, ask_guid, bid_guid, expiration_block }
	}
}

impl TestInfo {
	pub fn new_defaults() -> TestInfo {
		TestInfo::default()
	}

	pub fn create_ask_order(&self) -> TestAskOrder {
		let TestInfo { lender, loan_terms, expiration_block, ask_guid, .. } = self;
		let RegisteredAddress { address_id, account_id } = lender;

		assert_ok!(Creditcoin::add_ask_order(
			Origin::signed(account_id.clone()),
			address_id.clone(),
			loan_terms.clone(),
			*expiration_block,
			ask_guid.clone()
		));

		let ask_order_id = AskOrderId::new::<Test>(*expiration_block, ask_guid);

		(
			ask_order_id.clone(),
			Creditcoin::ask_orders(expiration_block, ask_order_id.hash()).unwrap(),
		)
	}

	pub fn create_bid_order(&self) -> TestBidOrder {
		let TestInfo { borrower, loan_terms, expiration_block, bid_guid, .. } = self;
		let RegisteredAddress { address_id, account_id } = borrower;

		assert_ok!(Creditcoin::add_bid_order(
			Origin::signed(account_id.clone()),
			address_id.clone(),
			loan_terms.clone(),
			*expiration_block,
			bid_guid.clone()
		));

		let bid_order_id = BidOrderId::new::<Test>(*expiration_block, bid_guid);
		(
			bid_order_id.clone(),
			Creditcoin::bid_orders(*expiration_block, bid_order_id.hash()).unwrap(),
		)
	}

	pub fn create_offer(&self) -> TestOffer {
		let RegisteredAddress { account_id, .. } = &self.lender;

		let (ask_order_id, _) = self.create_ask_order();
		let (bid_order_id, _) = self.create_bid_order();
		let expiration_block = 1_000;
		assert_ok!(Creditcoin::add_offer(
			Origin::signed(account_id.clone()),
			ask_order_id.clone(),
			bid_order_id.clone(),
			expiration_block,
		));
		let offer_id = OfferId::new::<Test>(expiration_block, &ask_order_id, &bid_order_id);
		(offer_id.clone(), Creditcoin::offers(expiration_block, offer_id.hash()).unwrap())
	}

	pub fn create_deal_order(&self) -> TestDealOrder {
		let RegisteredAddress { account_id, .. } = &self.borrower;
		let (offer_id, _) = self.create_offer();
		let expiration_block = 1_000;

		assert_ok!(Creditcoin::add_deal_order(
			Origin::signed(account_id.clone()),
			offer_id.clone(),
			expiration_block,
		));

		let deal_order_id = DealOrderId::new::<Test>(expiration_block, &offer_id);

		(
			deal_order_id.clone(),
			Creditcoin::deal_orders(expiration_block, deal_order_id.hash()).unwrap(),
		)
	}

	pub fn create_funding_transfer(&self, deal_order_id: &TestDealOrderId) -> TestTransfer {
		let deal_order =
			Creditcoin::deal_orders(deal_order_id.expiration(), deal_order_id.hash()).unwrap();
		let tx = "0xfafafa";
		assert_ok!(Creditcoin::register_funding_transfer(
			Origin::signed(self.lender.account_id.clone()),
			TransferKind::Native,
			deal_order_id.clone(),
			tx.as_bytes().into_bounded()
		));
		self.mock_transfer(&self.lender, &self.borrower, deal_order.terms.amount, deal_order_id, tx)
	}

	pub fn create_repayment_transfer(
		&self,
		deal_order_id: &TestDealOrderId,
		amount: impl Into<ExternalAmount>,
	) -> TestTransfer {
		let tx = "0xafafaf";
		let amount = amount.into();
		assert_ok!(Creditcoin::register_repayment_transfer(
			Origin::signed(self.borrower.account_id.clone()),
			TransferKind::Native,
			amount,
			deal_order_id.clone(),
			tx.as_bytes().into_bounded()
		));

		self.mock_transfer(&self.borrower, &self.lender, amount, deal_order_id, tx)
	}

	pub fn make_transfer(
		&self,
		from: &RegisteredAddress,
		to: &RegisteredAddress,
		amount: impl Into<ExternalAmount>,
		deal_order_id: &TestDealOrderId,
		blockchain_tx_id: impl AsRef<[u8]>,
		transfer_kind: impl Into<Option<TransferKind>>,
	) -> TestTransfer {
		let blockchain_tx_id = blockchain_tx_id.as_ref();
		let tx = if blockchain_tx_id.starts_with(b"0x") {
			core::str::from_utf8(blockchain_tx_id).unwrap().hex_to_address()
		} else {
			blockchain_tx_id.into_bounded()
		};
		let id = TransferId::new::<Test>(&Blockchain::Rinkeby, &tx);
		(
			id,
			Transfer {
				blockchain: self.blockchain.clone(),
				kind: transfer_kind.into().unwrap_or(TransferKind::Native),
				from: from.address_id.clone(),
				to: to.address_id.clone(),
				order_id: OrderId::Deal(deal_order_id.clone()),
				amount: amount.into(),
				tx_id: tx,
				block: System::block_number(),
				is_processed: false,
				account_id: from.account_id.clone(),
				timestamp: None,
			},
		)
	}

	pub fn mock_transfer(
		&self,
		from: &RegisteredAddress,
		to: &RegisteredAddress,
		amount: impl Into<ExternalAmount>,
		deal_order_id: &TestDealOrderId,
		blockchain_tx_id: impl AsRef<[u8]>,
	) -> TestTransfer {
		let (id, transfer) =
			self.make_transfer(from, to, amount, deal_order_id, blockchain_tx_id, None);
		Transfers::<Test>::insert(&id, &transfer);
		(id, transfer)
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

pub fn get_register_address_message(who: AccountId) -> [u8; 32] {
	sp_io::hashing::sha2_256(who.encode().as_slice())
}

#[test]
fn register_address_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;
		assert_ok!(Creditcoin::register_address(
			Origin::signed(who.clone()),
			blockchain.clone(),
			address.clone(),
			ownership_proof
		));
		let address_id = crate::AddressId::new::<Test>(&blockchain, &address);
		let address = crate::Address { blockchain, value: address, owner: who };
		assert_eq!(Creditcoin::addresses(address_id.clone()), Some(address.clone()));

		let event = <frame_system::Pallet<Test>>::events().pop().expect("an event").event;

		assert_matches!(
			event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::AddressRegistered(registered_address_id, registered_address)) => {
				assert_eq!(registered_address_id, address_id);
				assert_eq!(registered_address, address);
			}
		);
	});
}

#[test]
fn register_address_should_fail_to_reregister_external_address_to_same_account() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, external_address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;
		// Register external address to account
		assert_ok!(Creditcoin::register_address(
			Origin::signed(who.clone()),
			blockchain.clone(),
			external_address.clone(),
			ownership_proof.clone()
		));
		// Try registering again to same account and fail
		assert_noop!(
			Creditcoin::register_address(
				Origin::signed(who),
				blockchain,
				external_address,
				ownership_proof
			),
			crate::Error::<Test>::AddressAlreadyRegisteredByCaller
		);
	})
}

#[test]
fn register_address_should_fail_to_reregister_external_address_to_new_account() {
	ExtBuilder::default().build_and_execute(|| {
		let who_first = account_from_keypair(generate_keypair_from_seed("first account"));
		let who_second = account_from_keypair(generate_keypair_from_seed("second account"));
		let external_keypair = generate_keypair_from_seed("external account");
		let external_address = external_address_from_keypair(external_keypair.clone());
		let blockchain = Blockchain::Rinkeby;

		// Register external address to first account
		let first_ownership_proof =
			build_proof_of_ownership(who_first.clone(), external_keypair.clone());
		assert_ok!(Creditcoin::register_address(
			Origin::signed(who_first),
			blockchain.clone(),
			external_address.clone(),
			first_ownership_proof
		));
		// Try registering to a second account and fail
		let second_ownership_proof = build_proof_of_ownership(who_second.clone(), external_keypair);
		assert_noop!(
			Creditcoin::register_address(
				Origin::signed(who_second),
				blockchain,
				external_address,
				second_ownership_proof
			),
			crate::Error::<Test>::AddressAlreadyRegistered
		);
	})
}

#[test]
fn register_address_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let (_who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;

		assert_noop!(
			Creditcoin::register_address(Origin::none(), blockchain, address, ownership_proof),
			BadOrigin,
		);
	})
}

#[test]
fn register_address_should_error_when_using_wrong_ownership_proof() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, _ownership_proof, _) = generate_address_with_proof("owner");
		let (_who2, _address2, ownership_proof2, _) = generate_address_with_proof("bogus");

		let blockchain = Blockchain::Rinkeby;
		assert_noop!(
			Creditcoin::register_address(
				Origin::signed(who),
				blockchain,
				address,
				ownership_proof2
			),
			crate::Error::<Test>::OwnershipNotSatisfied
		);
	})
}

#[test]
fn register_address_should_error_when_address_too_long() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let address = format!("0xff{}", hex::encode(address)).hex_to_address();
		let blockchain = Blockchain::Rinkeby;
		assert_noop!(
			Creditcoin::register_address(Origin::signed(who), blockchain, address, ownership_proof),
			crate::Error::<Test>::AddressFormatNotSupported
		);
	})
}

#[test]
fn register_address_should_error_when_signature_is_invalid() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, _ownership_proof, _) = generate_address_with_proof("owner");

		// NOTE: No checking goes on to ensure this is a real signature! See
		// https://docs.rs/sp-core/2.0.0-rc4/sp_core/ecdsa/struct.Signature.html#method.from_raw
		let ownership_proof = sp_core::ecdsa::Signature::from_raw([0; 65]);

		let blockchain = Blockchain::Rinkeby;
		assert_noop!(
			Creditcoin::register_address(Origin::signed(who), blockchain, address, ownership_proof),
			crate::Error::<Test>::InvalidSignature
		);
	})
}

#[test]
fn verify_ethless_transfer() {
	ExtBuilder::default().build_offchain_and_execute_with_state(|state, _| {
		let dummy_url = "dummy";
		let tx_hash = get_mock_tx_hash();
		let contract = get_mock_contract().hex_to_address();
		let tx_block_num = get_mock_tx_block_num();
		let rpc_url_storage = StorageValueRef::persistent(B("rinkeby-rpc-uri"));
		rpc_url_storage.set(&dummy_url);

		MockedRpcRequests::new(dummy_url, &tx_hash, &tx_block_num, &ETHLESS_RESPONSES)
			.mock_all(&mut state.write());

		let from = get_mock_from_address().hex_to_address();
		let to = get_mock_to_address().hex_to_address();
		let order_id = crate::OrderId::Deal(crate::DealOrderId::with_expiration_hash::<Test>(
			10000,
			H256::from_uint(&get_mock_nonce()),
		));
		let amount = get_mock_amount();
		let tx_id = tx_hash.hex_to_address();

		assert_ok!(Creditcoin::verify_ethless_transfer(
			&Blockchain::Rinkeby,
			&contract,
			&from,
			&to,
			&order_id,
			&amount,
			&tx_id,
		));
	});
}

pub(crate) fn adjust_deal_order_to_nonce(
	deal_order_id: &TestDealOrderId,
	nonce: U256,
) -> TestDealOrderId {
	let deal_id_hash = H256::from_uint(&nonce);
	let deal = crate::DealOrders::<Test>::try_get_id(&deal_order_id).unwrap();
	crate::DealOrders::<Test>::remove(deal_order_id.expiration(), deal_order_id.hash());
	let fake_deal_order_id =
		crate::DealOrderId::with_expiration_hash::<Test>(deal_order_id.expiration(), deal_id_hash);
	crate::DealOrders::<Test>::insert_id(fake_deal_order_id.clone(), deal);
	fake_deal_order_id
}

#[test]
fn add_ask_order_basic() {
	let (mut ext, _, _) = ExtBuilder::default().build_offchain();

	ext.execute_with(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, blockchain, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;
		let (_, ask_order) = test_info.create_ask_order();
		let AskOrder { block, expiration_block, .. } = ask_order;

		let new_ask_order = crate::AskOrder {
			blockchain,
			lender_address_id: address_id,
			terms: loan_terms.try_into().unwrap(),
			expiration_block,
			block,
			lender: account_id,
		};

		assert_eq!(ask_order, new_ask_order);
	});
}

#[test]
fn add_ask_order_expired() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, ask_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _ask_order = test_info.create_ask_order();
		let expiration_block = 1_500;
		System::set_block_number(expiration_block + 1);

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
				ask_guid
			),
			crate::Error::<Test>::AskOrderExpired
		);
	});
}

#[test]
fn add_ask_order_used_guid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, ask_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _ask_order = test_info.create_ask_order();
		assert_eq!(crate::Pallet::<Test>::used_guids(ask_guid.clone()), Some(()));
		let expiration_block = 1_500;

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
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

		let (_, ask_order) = test_info.create_ask_order();
		let AskOrder { expiration_block, .. } = ask_order;
		let existing_ask_order_id = AskOrderId::new::<Test>(expiration_block, &ask_guid);
		assert_eq!(
			crate::Pallet::<Test>::ask_orders(expiration_block, existing_ask_order_id.hash()),
			Some(ask_order)
		);

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
				ask_guid
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
#[should_panic]
fn add_add_ask_order_rejects_zero_term_length_ms() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo {
			loan_terms: LoanTerms {
				amount: 0u64.into(),
				interest_rate: Default::default(),
				term_length: Duration::from_millis(0),
			},
			..TestInfo::new_defaults()
		};
		let _ = test_info.create_ask_order();
	});
}

#[test]
fn add_bid_order_basic() {
	let (mut ext, _, _) = ExtBuilder::default().build_offchain();

	ext.execute_with(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { borrower, loan_terms, blockchain, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = borrower;

		let (_, bid_order) = test_info.create_bid_order();
		let BidOrder { expiration_block, block, .. } = bid_order;

		let new_bid_order = crate::BidOrder {
			blockchain,
			borrower_address_id: address_id,
			terms: loan_terms.try_into().unwrap(),
			expiration_block,
			block,
			borrower: account_id,
		};

		assert_eq!(new_bid_order, bid_order);
	});
}

#[test]
fn add_bid_order_expired() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, bid_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _bid_order = test_info.create_bid_order();
		let expiration_block = 1_500;
		System::set_block_number(expiration_block + 1);

		assert_noop!(
			Creditcoin::add_bid_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
				bid_guid
			),
			crate::Error::<Test>::BidOrderExpired
		);
	});
}

#[test]
fn add_bid_order_used_guid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let TestInfo { lender, loan_terms, bid_guid, .. } = test_info.clone();
		let RegisteredAddress { address_id, account_id } = lender;

		let _bid_order = test_info.create_bid_order();
		assert_eq!(crate::Pallet::<Test>::used_guids(bid_guid.clone()), Some(()));
		let expiration_block = 1_500;

		assert_noop!(
			Creditcoin::add_ask_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
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

		let (_, bid_order) = test_info.create_bid_order();
		let BidOrder { expiration_block, .. } = bid_order;
		let existing_bid_order_id = BidOrderId::new::<Test>(expiration_block, &bid_guid);
		assert_eq!(
			crate::Pallet::<Test>::bid_orders(expiration_block, existing_bid_order_id.hash()),
			Some(bid_order)
		);

		assert_noop!(
			Creditcoin::add_bid_order(
				Origin::signed(account_id),
				address_id,
				loan_terms,
				expiration_block,
				bid_guid
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
#[should_panic]
fn add_bid_ask_order_rejects_zero_term_length_ms() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo {
			loan_terms: LoanTerms { term_length: Duration::from_millis(0), ..Default::default() },
			..TestInfo::new_defaults()
		};
		let _ = test_info.create_bid_order();
	});
}

#[test]
fn add_offer_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, offer) = test_info.create_offer();
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

		let (offer_id, offer) = test_info.create_offer();
		let Offer { expiration_block, ask_id, bid_id, lender, .. } = offer.clone();
		assert_eq!(crate::Pallet::<Test>::offers(expiration_block, offer_id.hash()), Some(offer));

		assert_noop!(
			Creditcoin::add_offer(Origin::signed(lender), ask_id, bid_id, expiration_block,),
			crate::Error::<Test>::DuplicateOffer
		);
	})
}

#[test]
fn add_offer_should_error_when_blockchain_differs_between_ask_and_bid_order() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, offer) = test_info.create_offer();
		let Offer { expiration_block, ask_id, bid_id, lender, .. } = offer;

		// simulate deal transfer
		crate::AskOrders::<Test>::mutate(ask_id.expiration(), ask_id.hash(), |ask_order_storage| {
			ask_order_storage.as_mut().unwrap().blockchain = Blockchain::Bitcoin;
		});

		assert_noop!(
			Creditcoin::add_offer(Origin::signed(lender), ask_id, bid_id, expiration_block,),
			crate::Error::<Test>::AddressBlockchainMismatch
		);
	})
}

#[test]
fn add_deal_order_basic() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, deal_order) = test_info.create_deal_order();
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
			block: Some(Creditcoin::block_number()),
		};

		assert_eq!(new_deal_order, deal_order);
	})
}

#[test]
fn add_deal_order_existing() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, deal_order) = test_info.create_deal_order();
		let DealOrder { expiration_block, borrower, offer_id, .. } = deal_order;

		assert_noop!(
			Creditcoin::add_deal_order(Origin::signed(borrower), offer_id, expiration_block),
			crate::Error::<Test>::DuplicateDealOrder
		);
	});
}

#[test]
fn lock_deal_order_should_emit_deal_order_locked_event() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);
		let test_info = TestInfo::new_defaults();

		let (deal_order_id, deal_order) = test_info.create_deal_order();

		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().funding_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"12345678"));
			},
		);

		assert_ok!(Creditcoin::lock_deal_order(
			Origin::signed(deal_order.borrower),
			deal_order_id.clone()
		));
		let event = <frame_system::Pallet<Test>>::events().pop().expect("expected an event").event;

		assert_matches!(event, crate::mock::RuntimeEvent::Creditcoin(crate::Event::DealOrderLocked(id))=>{
			assert_eq!(id,deal_order_id);
		});
	});
}

#[test]
fn lock_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order_id, _) = test_info.create_deal_order();

		assert_noop!(Creditcoin::lock_deal_order(Origin::none(), deal_order_id), BadOrigin);
	});
}

#[test]
fn lock_deal_order_should_error_for_non_existent_deal_order() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (_, deal_order) = test_info.create_deal_order();
		let DealOrder { borrower, offer_id, .. } = deal_order;
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

		let (deal_order_id, _) = test_info.create_deal_order();

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

		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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

		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
				Origin::signed(test_info.borrower.account_id),
				deal_order_id
			),
			crate::Error::<Test>::DealOrderAlreadyLocked
		);
	});
}

#[test]
fn lock_deal_order_locks_by_borrower() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// simulate deal transfer
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
			crate::Pallet::<Test>::deal_orders(deal_order.expiration_block, deal_order_id.hash())
				.unwrap();
		assert_eq!(locked_deal_order.lock, Some(test_info.borrower.account_id));
	});
}

#[test]
fn fund_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with an address that isn't registered
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with a timestamp in the future
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate a funded deal
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate an expired deal by setting expiration_block < head
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, _) = test_info.create_deal_order();

		// this is a deal_order from another person
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new("lender2", Blockchain::Rinkeby),
			borrower: RegisteredAddress::new("borrower2", Blockchain::Rinkeby),
			blockchain: Blockchain::Rinkeby,
			loan_terms: LoanTerms {
				amount: 2_000_000u64.into(),
				interest_rate: Default::default(),
				term_length: Duration::from_millis(1_000_000),
			},
			ask_guid: "second-ask-guid".as_bytes().into_bounded(),
			bid_guid: "second-bid-guid".as_bytes().into_bounded(),
			expiration_block: 3_333,
		};

		let (bogus_deal_order_id, _) = second_test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_funding_transfer(
			Origin::signed(second_test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract),
			bogus_deal_order_id.clone(),
			tx_hash
		));
		let (transfer_id, _) = second_test_info.create_funding_transfer(&bogus_deal_order_id);

		// try funding DealOrder from Person1 with the transfer from Person2,
		// which points to a different order_id
		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferDealOrderMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_amount_doesnt_match() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_funding_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract),
			deal_order_id.clone(),
			tx_hash
		));

		let (transfer_id, _) = test_info.create_funding_transfer(&deal_order_id);

		// modify deal amount in order to cause transfer mismatch
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				// note: the transfer above has amount of 0 b/c it is an exemption!
				deal_order_storage.as_mut().unwrap().terms.amount = ExternalAmount::from(4444u64);
			},
		);

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferAmountMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_sighash_doesnt_match_lender() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_funding_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract),
			deal_order_id.clone(),
			tx_hash
		));

		let (transfer_id, _) = test_info.create_funding_transfer(&deal_order_id);

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.account_id = AccountId::new([4; 32]);
		});

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
				deal_order_id,
				transfer_id
			),
			crate::Error::<Test>::TransferAccountMismatch
		);
	});
}

#[test]
fn fund_deal_order_should_error_when_transfer_has_been_processed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		let (transfer_id, _) = test_info.create_funding_transfer(&deal_order_id);

		// modify transfer in order to cause an error
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.amount = deal_order.terms.amount;
			ts.is_processed = true;
		});

		assert_noop!(
			Creditcoin::fund_deal_order(
				Origin::signed(test_info.lender.account_id),
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
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_funding_transfer(
			Origin::signed(test_info.lender.account_id.clone()),
			TransferKind::Ethless(contract),
			deal_order_id.clone(),
			tx_hash
		));

		let (transfer_id, _) = test_info.create_funding_transfer(&deal_order_id);

		// modify transfer b/c amount above is 0
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			transfer_storage.as_mut().unwrap().amount = deal_order.terms.amount;
		});

		assert_ok!(Creditcoin::fund_deal_order(
			Origin::signed(test_info.lender.account_id),
			deal_order_id.clone(),
			transfer_id.clone()
		));

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();

		let event2 = all_events.pop().expect("Second EventRecord").event;
		assert_matches!(
			event2,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::TransferProcessed(id)) =>{
				assert_eq!(transfer_id, id)

			}
		);

		let event1 = all_events.pop().expect("First EventRecord").event;
		assert_matches!(
			event1,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::DealOrderFunded(id))=>{
				assert_eq!(deal_order_id, id)
			}
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
	let pub_key = sp_core::ecdsa::Public::from_full(
		&hex::decode("0399d6e7c784494fd7edc26fc9ca460a68c97cc64c49c85dfbb68148f0607893bf").unwrap(),
	)
	.unwrap();
	let claimer = MultiSigner::from(pub_key).into_account();

	let mut ext = ExtBuilder::default();
	ext.fund(keeper.clone(), legacy_amount)
		.legacy_balance_keeper(keeper)
		.legacy_wallets(vec![(sighash, legacy_amount)]);

	ext.build_and_execute(|| {
		System::set_block_number(1);

		assert_ok!(Creditcoin::claim_legacy_wallet(Origin::signed(claimer.clone()), pub_key));
		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::LegacyWalletClaimed(..))
		));
		assert_eq!(frame_system::pallet::Account::<Test>::get(&claimer).data.free, 1000000);
	});
}

#[test]
fn add_authority_errors_for_non_root() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);

		assert_noop!(Creditcoin::add_authority(Origin::signed(acct.clone()), acct), BadOrigin);
	});
}

#[test]
fn add_authority_should_fail_when_authority_already_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let root = RawOrigin::Root;
		let acct: AccountId = AccountId::new([0; 32]);

		assert_ok!(Creditcoin::add_authority(
			crate::mock::RuntimeOrigin::from(root.clone()),
			acct.clone(),
		));

		// try again
		assert_noop!(
			Creditcoin::add_authority(crate::mock::RuntimeOrigin::from(root), acct,),
			crate::Error::<Test>::AlreadyAuthority,
		);
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
		let compliance_proof = key_pair.sign(&message);

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
				compliance_proof.into(),
			),
			crate::Error::<Test>::NotAddressOwner
		);
	});
}

#[test]
fn register_deal_order_should_error_when_lender_address_doesnt_match_sender() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(
				key_pair.public(),
				Blockchain::Rinkeby,
				ownership_proof,
			),
			..TestInfo::new_defaults()
		};

		let lender = RegisteredAddress::new("lender2", Blockchain::Rinkeby);
		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				key_pair.public().into(),
				compliance_proof.into(),
			),
			crate::Error::<Test>::NotAddressOwner
		);
	});
}

#[test]
fn register_deal_order_should_error_when_lender_and_borrower_are_on_different_chains() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			lender: RegisteredAddress::new("lender2", Blockchain::Ethereum),
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				pub_key.into(),
				compliance_proof.into(),
			),
			crate::Error::<Test>::AddressBlockchainMismatch
		);
	});
}

#[test]
fn register_deal_order_should_error_when_ask_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
			..TestInfo::new_defaults()
		};
		// create AskOrder which will use-up the default ID
		test_info.create_ask_order();

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				pub_key.into(),
				compliance_proof.into(),
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn register_deal_order_should_error_when_bid_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
			..TestInfo::new_defaults()
		};

		// create BidOrder which will use-up the default ID
		test_info.create_bid_order();

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

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
				compliance_proof.into(),
			),
			crate::Error::<Test>::DuplicateId
		);
	});
}

#[test]
fn register_deal_order_should_error_when_offer_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
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
			ask_id: ask_order_id,
			bid_id: bid_order_id,
			block: current_block,
			blockchain: test_info.blockchain.clone(),
			expiration_block: test_info.expiration_block,
			lender: test_info.lender.account_id.clone(),
		};
		// insert this offer into storage which will use-up the ID
		// register_deal_order() will reconstruct the same ID later
		crate::Offers::<Test>::insert_id(offer_id, offer);

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

		assert_noop!(
			Creditcoin::register_deal_order(
				Origin::signed(test_info.lender.account_id),
				test_info.lender.address_id,
				test_info.borrower.address_id,
				test_info.loan_terms,
				test_info.expiration_block,
				test_info.ask_guid,
				test_info.bid_guid,
				pub_key.into(),
				compliance_proof.into(),
			),
			crate::Error::<Test>::DuplicateOffer
		);
	});
}

#[test]
fn register_deal_order_should_error_when_deal_order_id_exists() {
	ExtBuilder::default().build_and_execute(|| {
		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

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
			block: Some(Creditcoin::block_number()),
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
				pub_key.into(),
				compliance_proof.into(),
			),
			crate::Error::<Test>::DuplicateDealOrder
		);
	});
}

#[test]
fn register_deal_order_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (_, _, ownership_proof, key_pair) = generate_address_with_proof("borrower2");
		let pub_key = key_pair.public();

		let test_info = TestInfo {
			borrower: RegisteredAddress::from_pubkey(pub_key, Blockchain::Rinkeby, ownership_proof),
			..TestInfo::new_defaults()
		};

		let message = test_info.get_register_deal_msg();
		let compliance_proof = key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			pub_key.into(),
			compliance_proof.into(),
		));

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event4 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event4,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::DealOrderAdded(..))
		));

		let event3 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event3,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::OfferAdded(..))
		));

		let event2 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event2,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::BidOrderAdded(..))
		));

		let event1 = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert!(matches!(
			event1,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::AskOrderAdded(..))
		));
	});
}

#[test]
fn register_deal_order_accepts_sr25519() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (owners_key_pair, _) = sp_core::sr25519::Pair::generate();
		let o_pubkey = owners_key_pair.public();
		let o_signer: MultiSigner = o_pubkey.into();
		let owners_account = o_signer.into_account();

		let test_info = {
			let (b_key_pair, _) = sp_core::ecdsa::Pair::generate();
			let b_pubkey = b_key_pair.public();
			let message = get_register_address_message(owners_account.clone());
			let ownership_proof = b_key_pair.sign(message.as_slice());
			TestInfo {
				borrower: RegisteredAddress::from_pubkey_distinct_owner(
					owners_account,
					Blockchain::Rinkeby,
					b_pubkey,
					ownership_proof,
				),
				..TestInfo::new_defaults()
			}
		};

		let message = test_info.get_register_deal_msg();
		let compliance_proof = owners_key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			o_pubkey.into(),
			compliance_proof.into(),
		));
	});
}

#[test]
fn register_deal_order_accepts_ed25519() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (owners_key_pair, _) = sp_core::ed25519::Pair::generate();
		let o_pubkey = owners_key_pair.public();
		let o_signer: MultiSigner = o_pubkey.into();
		let owners_account = o_signer.clone().into_account();

		let test_info = {
			let (b_key_pair, _) = sp_core::ecdsa::Pair::generate();
			let b_pubkey = b_key_pair.public();
			let message = get_register_address_message(owners_account.clone());
			let ownership_proof = b_key_pair.sign(message.as_slice());
			TestInfo {
				borrower: RegisteredAddress::from_pubkey_distinct_owner(
					owners_account,
					Blockchain::Rinkeby,
					b_pubkey,
					ownership_proof,
				),
				..TestInfo::new_defaults()
			}
		};

		let message = test_info.get_register_deal_msg();
		let compliance_proof = owners_key_pair.sign(&message);

		assert_ok!(Creditcoin::register_deal_order(
			Origin::signed(test_info.lender.account_id),
			test_info.lender.address_id,
			test_info.borrower.address_id,
			test_info.loan_terms,
			test_info.expiration_block,
			test_info.ask_guid,
			test_info.bid_guid,
			o_signer,
			compliance_proof.into(),
		));
	});
}

#[test]
fn close_deal_order_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with an address that isn't registered
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal with a timestamp in the future
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate DealOrder which has been repaid
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"12345678");

		// simulate deal which is not locked
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
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
		let (deal_order_id, _) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);
		// this is a deal_order from another person
		let second_test_info = TestInfo {
			lender: RegisteredAddress::new("lender2", Blockchain::Rinkeby),
			borrower: RegisteredAddress::new("borrower2", Blockchain::Rinkeby),
			blockchain: Blockchain::Rinkeby,
			loan_terms: LoanTerms {
				amount: 2_000_000u64.into(),
				interest_rate: Default::default(),
				term_length: Duration::from_millis(1_000_000),
			},
			ask_guid: "second-ask-guid".as_bytes().into_bounded(),
			bid_guid: "second-bid-guid".as_bytes().into_bounded(),
			expiration_block: 3_333,
		};

		let (bogus_deal_order_id, _) = second_test_info.create_deal_order();

		let (transfer_id, _) =
			second_test_info.create_repayment_transfer(&bogus_deal_order_id, 33u64);

		// Person1 tries closing the deal by using the transfer made by Person2
		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferDealOrderMismatch
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_block_is_greater_than_current_block() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		let (transfer_id, _) =
			test_info.create_repayment_transfer(&deal_order_id, deal_order.terms.amount);

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
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		let (transfer_id, _) =
			test_info.create_repayment_transfer(&deal_order_id, deal_order.terms.amount);

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			ts.account_id = AccountId::new([44; 32]);
		});

		assert_noop!(
			Creditcoin::close_deal_order(
				Origin::signed(test_info.borrower.account_id),
				deal_order_id,
				transfer_id,
			),
			crate::Error::<Test>::TransferAccountMismatch
		);
	});
}

#[test]
fn close_deal_order_should_error_when_transfer_has_already_been_processed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		let (transfer_id, _) =
			test_info.create_repayment_transfer(&deal_order_id, deal_order.terms.amount);

		// modify transfer in order to cause transfer mismatch
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();
			// b/c amount above is 0
			ts.is_processed = true;
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
fn close_deal_order_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// lock DealOrder
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().lock =
					Some(test_info.borrower.account_id.clone());
			},
		);

		//  insert as exemption to bypass transfer verification
		let tx_hash = "0".as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

		assert_ok!(Creditcoin::register_transfer_internal(
			test_info.borrower.account_id.clone(),
			test_info.borrower.address_id.clone(),
			test_info.lender.address_id.clone(),
			TransferKind::Ethless(contract),
			33u64.into(),
			OrderId::Deal(deal_order_id.clone()),
			tx_hash
		));

		let (transfer_id, _) =
			test_info.create_repayment_transfer(&deal_order_id, deal_order.terms.amount + 1u64);

		// modify transfer to make sure we have transfered enough funds
		crate::Transfers::<Test>::mutate(&transfer_id, |transfer_storage| {
			let mut ts = transfer_storage.as_mut().unwrap();

			ts.amount = deal_order.terms.amount + 1u64;
		});

		assert_ok!(Creditcoin::close_deal_order(
			Origin::signed(test_info.borrower.account_id),
			deal_order_id.clone(),
			transfer_id.clone(),
		));

		// assert field values were updated in storage
		let saved_deal_order = DealOrders::<Test>::try_get_id(&deal_order_id).unwrap();
		assert_eq!(saved_deal_order.repayment_transfer_id, Some(transfer_id.clone()));

		let saved_transfer = crate::Pallet::<Test>::transfers(&transfer_id).unwrap();
		assert!(saved_transfer.is_processed);

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event2 = all_events.pop().expect("Second EventRecord").event;
		assert_matches!(
			event2,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::TransferProcessed(id)) =>{
				assert_eq!(id,transfer_id);
			}
		);

		let event1 = all_events.pop().expect("First EventRecord").event;
		assert_matches!(
			event1,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::DealOrderClosed(id)) =>{
				assert_eq!(id,deal_order_id);
			}
		);
	});
}

#[test]
fn exempt_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();

		assert_noop!(Creditcoin::exempt(Origin::none(), deal_order_id), BadOrigin);
	});
}

#[test]
fn exempt_should_error_when_deal_order_has_already_been_repaid() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// simulate DealOrder which has been repaid
		crate::DealOrders::<Test>::mutate(
			deal_order_id.expiration(),
			deal_order_id.hash(),
			|deal_order_storage| {
				deal_order_storage.as_mut().unwrap().repayment_transfer_id =
					Some(TransferId::new::<Test>(&deal_order.blockchain, b"4444"));
			},
		);

		assert_noop!(
			Creditcoin::exempt(Origin::signed(test_info.lender.account_id), deal_order_id),
			crate::Error::<Test>::DealOrderAlreadyClosed
		);
	});
}

#[test]
fn exempt_should_error_for_non_lender() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();

		assert_noop!(
			Creditcoin::exempt(Origin::signed(test_info.borrower.account_id), deal_order_id),
			crate::Error::<Test>::NotLender
		);
	});
}

#[test]
fn exempt_should_succeed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();

		assert_ok!(Creditcoin::exempt(
			Origin::signed(test_info.lender.account_id),
			deal_order_id.clone()
		));

		let transfer_id = TransferId::new::<Test>(&deal_order.blockchain, b"0");

		// assert field values were updated in storage
		let saved_deal_order = DealOrders::<Test>::try_get_id(&deal_order_id).unwrap();
		assert_eq!(saved_deal_order.repayment_transfer_id, Some(transfer_id));

		// assert events in reversed order
		let mut all_events = <frame_system::Pallet<Test>>::events();
		let event = all_events.pop().expect("Expected at least one EventRecord to be found").event;
		assert_eq!(
			event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::LoanExempted(deal_order_id))
		);
	});
}

#[test]
fn verify_transfer_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		let (transfer_id, transfer) = test_info.create_funding_transfer(&deal_order_id);
		let deadline = Test::unverified_transfer_deadline();
		assert_noop!(
			Creditcoin::persist_task_output(
				Origin::none(),
				deadline,
				(transfer_id, transfer).into()
			),
			BadOrigin
		);
	});
}

#[test]
fn verify_transfer_should_error_when_signer_not_authorized() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		let (transfer_id, transfer) = test_info.create_funding_transfer(&deal_order_id);
		let deadline = Test::unverified_transfer_deadline();
		assert_noop!(
			Creditcoin::persist_task_output(
				Origin::signed(test_info.lender.account_id),
				deadline,
				(transfer_id, transfer).into(),
			),
			crate::Error::<Test>::InsufficientAuthority,
		);
	});
}

#[test]
fn verify_transfer_should_error_when_transfer_has_already_been_registered() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();

		// authorize lender
		let root = RawOrigin::Root;
		assert_ok!(Creditcoin::add_authority(
			crate::mock::RuntimeOrigin::from(root),
			test_info.lender.account_id.clone(),
		));

		let (deal_order_id, _) = test_info.create_deal_order();
		let (transfer_id, transfer) = test_info.create_funding_transfer(&deal_order_id);
		let deadline = Test::unverified_transfer_deadline();

		assert_noop!(
			Creditcoin::persist_task_output(
				Origin::signed(test_info.lender.account_id),
				deadline,
				(transfer_id, transfer).into(),
			),
			non_paying_error(crate::Error::<Test>::TransferAlreadyRegistered),
		);
	});
}

#[test]
fn verify_transfer_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();

		// authorize lender
		let root = RawOrigin::Root;
		assert_ok!(Creditcoin::add_authority(
			crate::mock::RuntimeOrigin::from(root),
			test_info.lender.account_id.clone(),
		));

		let (deal_order_id, deal_order) = test_info.create_deal_order();

		// create a transfer but don't add it into storage
		let tx = "0xafafaf".as_bytes().into_bounded();
		let transfer_id = TransferId::new::<Test>(&Blockchain::Rinkeby, &tx);
		let transfer = Transfer {
			blockchain: test_info.blockchain.clone(),
			kind: TransferKind::Native,
			from: test_info.lender.address_id.clone(),
			to: test_info.borrower.address_id.clone(),
			order_id: OrderId::Deal(deal_order_id),
			amount: deal_order.terms.amount,
			tx_id: tx,
			block: System::block_number(),
			is_processed: false,
			account_id: test_info.lender.account_id.clone(),
			timestamp: None,
		};
		let deadline = Test::unverified_transfer_deadline();

		assert_ok!(Creditcoin::persist_task_output(
			Origin::signed(test_info.lender.account_id),
			deadline,
			(transfer_id.clone(), transfer.clone()).into(),
		));

		let mut all_events = <frame_system::Pallet<Test>>::events();

		// assert events in reversed order
		let last_event = all_events.pop().expect("At least one EventRecord").event;
		assert_matches!(
			last_event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::TransferVerified(id)) => {
				assert_eq!(transfer_id, id)
			}
		);

		assert_eq!(Transfers::<Test>::get(&transfer_id), Some(transfer));
	});
}

#[test]
fn fail_transfer_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();

		let root = RawOrigin::Root;
		assert_ok!(Creditcoin::add_authority(
			crate::mock::RuntimeOrigin::from(root),
			test_info.lender.account_id.clone(),
		));

		let _ = test_info.create_deal_order();

		let tx = "0xafafaf".hex_to_address();
		let transfer_id = TransferId::new::<Test>(&Blockchain::Rinkeby, &tx);

		let failure_cause = crate::ocw::errors::VerificationFailureCause::TaskFailed;
		let deadline = Test::unverified_transfer_deadline();

		assert_ok!(Creditcoin::fail_task(
			Origin::signed(test_info.lender.account_id),
			deadline,
			transfer_id.clone().into(),
			failure_cause
		));

		let mut all_events = System::events();

		assert_matches!(
			all_events.pop().unwrap().event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::TransferFailedVerification(id, cause)) => {
				assert_eq!(id, transfer_id);
				assert_eq!(cause, failure_cause);
			}
		);
	})
}

#[test]
fn fail_transfer_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();

		let _ = test_info.create_deal_order();

		let tx = "0xafafaf".hex_to_address();
		let transfer_id = TransferId::new::<Test>(&Blockchain::Rinkeby, &tx);

		let failure_cause = crate::ocw::errors::VerificationFailureCause::TaskFailed;
		let deadline = Test::unverified_transfer_deadline();

		assert_noop!(
			Creditcoin::fail_task(Origin::none(), deadline, transfer_id.into(), failure_cause),
			BadOrigin
		);
	})
}

#[test]
fn fail_transfer_should_error_when_not_authority() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();

		let _ = test_info.create_deal_order();

		let tx = "0xafafaf".hex_to_address();
		let transfer_id = TransferId::new::<Test>(&Blockchain::Rinkeby, &tx);

		let failure_cause = crate::ocw::errors::VerificationFailureCause::TaskFailed;
		let deadline = Test::unverified_transfer_deadline();

		assert_noop!(
			Creditcoin::fail_task(
				Origin::signed(test_info.lender.account_id),
				deadline,
				transfer_id.into(),
				failure_cause
			),
			crate::Error::<Test>::InsufficientAuthority
		);
	})
}

#[test]
fn fail_transfer_should_error_when_transfer_registered() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let test_info = TestInfo::new_defaults();

		let (deal_order_id, _) = test_info.create_deal_order();

		let (transfer_id, _) = test_info.create_funding_transfer(&deal_order_id);

		let root = RawOrigin::Root;
		assert_ok!(Creditcoin::add_authority(
			crate::mock::RuntimeOrigin::from(root),
			test_info.lender.account_id.clone(),
		));

		let failure_cause = crate::ocw::errors::VerificationFailureCause::TaskFailed;
		let deadline = Test::unverified_transfer_deadline();

		assert_noop!(
			Creditcoin::fail_task(
				Origin::signed(test_info.lender.account_id),
				deadline,
				transfer_id.into(),
				failure_cause
			),
			crate::Error::<Test>::TransferAlreadyRegistered
		);
	})
}

#[test]
fn on_initialize_removes_expired_deals_without_transfers() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		System::set_block_number(1);
		let mut expected_deal_orders = Vec::new();

		let now = System::block_number();
		for expiration_block in now..=20 {
			let seed1 = format!("{:02}0", expiration_block.clone());
			let seed2 = format!("{:02}1", expiration_block.clone());

			let test_info = TestInfo {
				lender: RegisteredAddress::new(&seed1, Blockchain::Rinkeby),
				borrower: RegisteredAddress::new(&seed2, Blockchain::Rinkeby),
				blockchain: Blockchain::Rinkeby,
				loan_terms: LoanTerms {
					amount: 2_000_000u64.into(),
					interest_rate: Default::default(),
					term_length: Duration::from_millis(1_000_000),
				},
				ask_guid: format!("{:?}-ask-guid", expiration_block.clone())
					.as_bytes()
					.into_bounded(),
				bid_guid: format!("{:?}-bid-guid", expiration_block.clone())
					.as_bytes()
					.into_bounded(),
				expiration_block,
			};

			let (offer_id, _) = test_info.create_offer();
			assert_ok!(Creditcoin::add_deal_order(
				Origin::signed(test_info.borrower.account_id.clone()),
				offer_id.clone(),
				expiration_block,
			));
			let deal_order_id = DealOrderId::new::<Test>(expiration_block, &offer_id);
			let deal_order =
				Creditcoin::deal_orders(deal_order_id.expiration(), deal_order_id.hash()).unwrap();

			// fund only deal orders which expire at even blocks
			if expiration_block % 2 == 0 {
				let tx = format!("0xfafafa{:02}", expiration_block.clone());
				assert_ok!(Creditcoin::register_funding_transfer(
					Origin::signed(test_info.lender.account_id.clone()),
					TransferKind::Native,
					deal_order_id.clone(),
					tx.as_bytes().into_bounded()
				));
				let (transfer_id, _) = test_info.mock_transfer(
					&test_info.lender,
					&test_info.borrower,
					deal_order.terms.amount,
					&deal_order_id,
					tx,
				);

				// attach transfer to deal order
				assert_ok!(Creditcoin::fund_deal_order(
					Origin::signed(test_info.lender.account_id.clone()),
					deal_order_id.clone(),
					transfer_id.clone()
				));
				// it's funded so it should be kept
				expected_deal_orders.push(deal_order_id.clone());
			} else if expiration_block > 15 {
				// still hasn't expired so should be kept regardless
				expected_deal_orders.push(deal_order_id.clone());
			}
		}

		// advance blocks, will perform housekeeping
		roll_to(15);

		for expected_order_id in expected_deal_orders.iter() {
			let _order = DealOrders::<Test>::try_get_id(&expected_order_id).unwrap();
		}
	});
}

#[test]
fn register_funding_transfer_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		let tx = "0xabcabcabc";

		assert_noop!(
			Creditcoin::register_funding_transfer(
				Origin::none(),
				TransferKind::Native,
				deal_order_id,
				tx.as_bytes().into_bounded()
			),
			BadOrigin
		);
	})
}

#[test]
fn register_funding_transfer_should_error_when_not_deal_order_not_found() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (_, deal_order) = test_info.create_deal_order();
		let DealOrder { offer_id, .. } = deal_order;
		// expiration_block set to 0
		let deal_order_id = DealOrderId::new::<Test>(0, &offer_id);

		let tx = "0xabcabcabc";

		assert_noop!(
			Creditcoin::register_funding_transfer(
				Origin::signed(test_info.lender.account_id),
				TransferKind::Native,
				deal_order_id,
				tx.as_bytes().into_bounded()
			),
			crate::Error::<Test>::NonExistentDealOrder
		);
	})
}

#[test]
fn register_repayment_transfer_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		let tx = "0xabcabcabc";

		assert_noop!(
			Creditcoin::register_repayment_transfer(
				Origin::none(),
				TransferKind::Native,
				21u64.into(),
				deal_order_id,
				tx.as_bytes().into_bounded()
			),
			BadOrigin
		);
	})
}

#[test]
fn register_repayment_transfer_should_error_when_not_deal_order_not_found() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (_, deal_order) = test_info.create_deal_order();
		let DealOrder { offer_id, .. } = deal_order;
		// expiration_block set to 0
		let deal_order_id = DealOrderId::new::<Test>(0, &offer_id);

		let tx = "0xabcabcabc";

		assert_noop!(
			Creditcoin::register_repayment_transfer(
				Origin::signed(test_info.borrower.account_id),
				TransferKind::Native,
				21u64.into(),
				deal_order_id,
				tx.as_bytes().into_bounded()
			),
			crate::Error::<Test>::NonExistentDealOrder
		);
	})
}

#[test]
fn register_transfer_internal_should_error_with_non_existent_lender_address() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let tx = "0xabcabcabc";
		let bogus_address =
			AddressId::new::<Test>(&Blockchain::Rinkeby, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

		let result = Creditcoin::register_transfer_internal(
			test_info.lender.account_id,
			bogus_address,
			deal_order.borrower_address_id,
			TransferKind::Native,
			deal_order.terms.amount,
			OrderId::Deal(deal_order_id),
			tx.as_bytes().into_bounded(),
		)
		.unwrap_err();

		assert_eq!(result, crate::Error::<Test>::NonExistentAddress);
	})
}

#[test]
fn register_transfer_internal_should_error_when_addresses_are_not_on_the_same_blockchain() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let second_borrower = RegisteredAddress::new("borrower2", Blockchain::Luniverse);
		let tx = "0xabcabcabc";

		let result = Creditcoin::register_transfer_internal(
			test_info.lender.account_id,
			deal_order.lender_address_id,
			second_borrower.address_id,
			TransferKind::Native,
			deal_order.terms.amount,
			OrderId::Deal(deal_order_id),
			tx.as_bytes().into_bounded(),
		)
		.unwrap_err();

		assert_eq!(result, crate::Error::<Test>::AddressBlockchainMismatch);
	})
}

#[test]
fn register_transfer_internal_should_error_when_transfer_kind_is_not_supported() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let tx = "0xabcabcabc";

		let result = Creditcoin::register_transfer_internal(
			test_info.lender.account_id,
			deal_order.lender_address_id,
			deal_order.borrower_address_id,
			// not supported on Blockchain::Rinkeby
			TransferKind::Other(BoundedVec::try_from(b"other".to_vec()).unwrap()),
			deal_order.terms.amount,
			OrderId::Deal(deal_order_id),
			tx.as_bytes().into_bounded(),
		)
		.unwrap_err();

		assert_eq!(result, crate::Error::<Test>::UnsupportedTransferKind);
	})
}

#[test]
fn register_transfer_internal_should_error_when_transfer_is_already_registered() {
	ExtBuilder::default().build_and_execute(|| {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, deal_order) = test_info.create_deal_order();
		let (_, transfer) = test_info.create_funding_transfer(&deal_order_id);

		let result = Creditcoin::register_transfer_internal(
			test_info.lender.account_id,
			deal_order.lender_address_id,
			deal_order.borrower_address_id,
			TransferKind::Native,
			deal_order.terms.amount,
			OrderId::Deal(deal_order_id),
			transfer.tx_id,
		)
		.unwrap_err();

		assert_eq!(result, crate::Error::<Test>::TransferAlreadyRegistered);
	})
}

#[test]
fn exercise_weightinfo_functions() {
	let result = super::weights::WeightInfo::<Test>::register_address();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::claim_legacy_wallet();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::add_ask_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::add_bid_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::add_offer();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::add_deal_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::add_authority();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::persist_transfer();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::fail_transfer();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::fund_deal_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::lock_deal_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::register_funding_transfer();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::register_repayment_transfer();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::close_deal_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::exempt();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::register_deal_order();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::request_collect_coins();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::fail_collect_coins();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::persist_collect_coins();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::remove_authority();
	assert!(result.ref_time() > 0);

	let result = super::weights::WeightInfo::<Test>::set_collect_coins_contract();
	assert!(result.ref_time() > 0);
}

#[test]
fn remove_authority_errors_for_non_root() {
	ExtBuilder::default().build_and_execute(|| {
		let acct: AccountId = AccountId::new([0; 32]);

		assert_noop!(Creditcoin::remove_authority(Origin::signed(acct.clone()), acct), BadOrigin);
	});
}

#[test]
fn remove_authority_should_fail_when_authority_does_not_exist() {
	ExtBuilder::default().build_and_execute(|| {
		let root = RawOrigin::Root;
		let acct: AccountId = AccountId::new([0; 32]);

		assert_noop!(
			Creditcoin::remove_authority(crate::mock::RuntimeOrigin::from(root), acct),
			crate::Error::<Test>::NotAnAuthority,
		);
	});
}

#[test]
fn add_and_remove_authority_works_for_root() {
	ExtBuilder::default().build_and_execute(|| {
		let root = RawOrigin::Root;
		let account: AccountId = AccountId::new([0; 32]);

		assert!(!TaskScheduler::is_authority(&account));
		TaskScheduler::insert_authority(&account);
		assert!(TaskScheduler::is_authority(&account));

		assert_ok!(Creditcoin::remove_authority(Origin::from(root), account.clone()));
		assert!(!TaskScheduler::is_authority(&account));
	});
}

#[test]
fn register_address_v2_should_work() {
	ExtBuilder::default().build_and_execute(|| {
		System::set_block_number(1);

		let (who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;

		let proof = OwnershipProof::EthSign(ownership_proof);

		assert_ok!(Creditcoin::register_address_v2(
			Origin::signed(who.clone()),
			blockchain.clone(),
			address.clone(),
			proof,
		));
		let address_id = crate::AddressId::new::<Test>(&blockchain, &address);
		let address = crate::Address { blockchain, value: address, owner: who };
		assert_eq!(Creditcoin::addresses(address_id.clone()), Some(address.clone()));

		let event = <frame_system::Pallet<Test>>::events().pop().expect("an event").event;

		assert_matches!(
			event,
			crate::mock::RuntimeEvent::Creditcoin(crate::Event::<Test>::AddressRegistered(registered_address_id, registered_address)) => {
				assert_eq!(registered_address_id, address_id);
				assert_eq!(registered_address, address);
			}
		);
	});
}

#[test]
fn register_address_v2_should_fail_to_reregister_external_address_to_same_account() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, external_address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;
		// Register external address to account
		assert_ok!(Creditcoin::register_address(
			Origin::signed(who.clone()),
			blockchain.clone(),
			external_address.clone(),
			ownership_proof.clone()
		));

		let proof = OwnershipProof::EthSign(ownership_proof);

		// Try registering again to same account and fail
		assert_noop!(
			Creditcoin::register_address_v2(
				Origin::signed(who),
				blockchain,
				external_address,
				proof,
			),
			crate::Error::<Test>::AddressAlreadyRegisteredByCaller
		);
	})
}

#[test]
fn register_address_v2_should_fail_to_reregister_external_address_to_new_account() {
	ExtBuilder::default().build_and_execute(|| {
		let who_first = account_from_keypair(generate_keypair_from_seed("first account"));
		let who_second = account_from_keypair(generate_keypair_from_seed("second account"));
		let external_keypair = generate_keypair_from_seed("external account");
		let external_address = external_address_from_keypair(external_keypair.clone());
		let blockchain = Blockchain::Rinkeby;

		// Register external address to first account
		let first_ownership_proof =
			build_proof_of_ownership(who_first.clone(), external_keypair.clone());

		let proof = OwnershipProof::EthSign(first_ownership_proof);
		assert_ok!(Creditcoin::register_address_v2(
			Origin::signed(who_first),
			blockchain.clone(),
			external_address.clone(),
			proof,
		));
		// Try registering to a second account and fail
		let second_ownership_proof = build_proof_of_ownership(who_second.clone(), external_keypair);

		let second_proof = OwnershipProof::EthSign(second_ownership_proof);
		assert_noop!(
			Creditcoin::register_address_v2(
				Origin::signed(who_second),
				blockchain,
				external_address,
				second_proof
			),
			crate::Error::<Test>::AddressAlreadyRegistered
		);
	})
}

#[test]
fn register_address_v2_should_error_when_not_signed() {
	ExtBuilder::default().build_and_execute(|| {
		let (_who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let blockchain = Blockchain::Rinkeby;

		let proof = OwnershipProof::EthSign(ownership_proof);
		assert_noop!(
			Creditcoin::register_address_v2(Origin::none(), blockchain, address, proof),
			BadOrigin,
		);
	})
}

#[test]
fn register_address_v2_should_error_when_using_wrong_ownership_proof() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, _ownership_proof, _) = generate_address_with_proof("owner");
		let (_who2, _address2, ownership_proof2, _) = generate_address_with_proof("bogus");

		let blockchain = Blockchain::Rinkeby;
		let bad_proof = OwnershipProof::EthSign(ownership_proof2);
		assert_noop!(
			Creditcoin::register_address_v2(
				Origin::signed(who),
				blockchain,
				address,
				bad_proof,
			),
			crate::Error::<Test>::OwnershipNotSatisfied
		);
	})
}

#[test]
fn register_address_v2_should_error_when_address_too_long() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, ownership_proof, _) = generate_address_with_proof("owner");
		let address = format!("0xff{}", hex::encode(address)).hex_to_address();
		let blockchain = Blockchain::Rinkeby;
		let proof = OwnershipProof::EthSign(ownership_proof);
		assert_noop!(
			Creditcoin::register_address_v2(Origin::signed(who), blockchain, address, proof),
			crate::Error::<Test>::EthSignExternalAddressGenerationFailed
		);
	})
}

#[test]
fn register_address_v2_should_error_when_signature_is_invalid() {
	ExtBuilder::default().build_and_execute(|| {
		let (who, address, _ownership_proof, _) = generate_address_with_proof("owner");

		// NOTE: No checking goes on to ensure this is a real signature! See
		// https://docs.rs/sp-core/2.0.0-rc4/sp_core/ecdsa/struct.Signature.html#method.from_raw
		let ownership_proof = sp_core::ecdsa::Signature::from_raw([0; 65]);

		let bad_proof = OwnershipProof::EthSign(ownership_proof);

		let blockchain = Blockchain::Rinkeby;
		assert_noop!(
			Creditcoin::register_address_v2(Origin::signed(who), blockchain, address, bad_proof),
			crate::Error::<Test>::EthSignPublicKeyRecoveryFailed
		);
	})
}
