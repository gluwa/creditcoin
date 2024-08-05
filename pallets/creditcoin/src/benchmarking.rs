#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::benchmarking::alloc::format;
use crate::helpers::{extensions::IntoBounded, EVMAddress, PublicToAddress};
use crate::migrations::Migrate;
use crate::test_utils::{
	fake_address_id, fake_ask_id, fake_bid_id, fake_loan_terms, fake_offer_id, insert_fake_ask,
	insert_fake_bid, insert_fake_offer,
};
use crate::types::{Blockchain, OwnershipProof};
use crate::Pallet as Creditcoin;
use crate::{AskOrderId, LoanTerms};
use frame_benchmarking::{account, benchmarks, whitelist_account, Zero};
use frame_support::{pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use frame_system::Config as SystemConfig;
use frame_system::Pallet as System;
use frame_system::RawOrigin;
use pallet_balances::Pallet as Balances;
use pallet_timestamp::Config as TimestampConfig;
use pallet_timestamp::Pallet as Timestamp;
use sp_core::ecdsa;
use sp_io::crypto::{ecdsa_generate, ecdsa_sign};
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::One;
use sp_runtime::KeyTypeId;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DealKind {
	Funded,
	Unfunded,
}

benchmarks! {
	migration_v7 {
		let t in 0..1024;

		for t in 0..t {
			let account:T::AccountId = account("Authority",1,t);
			crate::migrations::v7::Authorities::<T>::insert(account,());
		}

		let m = crate::migrations::v7::Migration::<T>::new();

	 }: {m.migrate()}

	on_initialize {
		//insert a askorders
		let a in 0..255;
		//insert b bidorders
		let b in 0..255;
		//insert o offers
		let o in 0..255;
		//insert d dealorders
		let d in 0..255;
		//insert f fundedorders
		let f in 0..255;

		<Timestamp<T>>::set_timestamp(1u32.into());

		let lender = lender_account::<T>(false);
		let borrower = borrower_account::<T>(false);

		let terms = fake_loan_terms();

		let expiration_block = T::BlockNumber::one();
		//generate this many filler asks
		for i in 0..a {
			insert_fake_ask::<T>(&borrower, expiration_block, i);
		}
		//generate this many filler bids
		for i in 0..b {
			insert_fake_bid::<T>(&lender, expiration_block, i);
		}
		//generate this many matching offers,bids,asks
		for i in 0..o {
			insert_fake_offer::<T>(&lender, expiration_block, i);
		}
		//generate this many matching deals,offers,asks,bids
		for i in 0..d {
			insert_fake_deal::<T>(&lender, expiration_block, DealKind::Unfunded, i);
		}
		//generate this many matching funded_deals with its deal,transfer,offer,ask and bid.
		for i in d..d + f{
			insert_fake_deal::<T>(&lender, expiration_block, DealKind::Funded, i);
		}

	}: { Creditcoin::<T>::on_initialize(expiration_block) }
	verify {}

	register_address {
		let who: T::AccountId = lender_account::<T>(false);
		let ktypeid = KeyTypeId(*b"dumy");
		let seed = "//who".as_bytes().to_vec();
		let pkey = ecdsa_generate(ktypeid, Some(seed));
		let address = EVMAddress::from_public(&pkey);
		let message = sp_io::hashing::sha2_256(who.encode().as_slice());
		let signature = ecdsa_sign(ktypeid, &pkey, &message).expect("ecdsa signature");

	}: _(RawOrigin::Signed(who), Blockchain::Ethereum, address,signature)

	claim_legacy_wallet {
		let pubkey = {
			let raw_key:[u8;33]= hex::decode("0399d6e7c784494fd7edc26fc9ca460a68c97cc64c49c85dfbb68148f0607893bf").unwrap().try_into().unwrap();
			ecdsa::Public::from_raw(raw_key)
		};

		let claimer = T::Signer::from(pubkey).into_account();
		whitelist_account!(claimer);

		let sighash = LegacySighash::from(&pubkey);
		let cash = <Balances<T> as Currency<T::AccountId>>::minimum_balance();
		LegacyWallets::<T>::insert(sighash, cash);

		let keeper: T::AccountId = account("keeper", 1, 1);
		<Balances<T> as Currency<T::AccountId>>::make_free_balance_be(&keeper,cash);

		LegacyBalanceKeeper::<T>::put(keeper.clone());

	}: _(RawOrigin::Signed(claimer.clone()), pubkey)
	verify {
		assert!(Balances::<T>::free_balance(&keeper).is_zero());
		assert_eq!(Balances::<T>::free_balance(&claimer),cash);
	}

	add_ask_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let who:T::AccountId = lender_account::<T>(true);
		let terms = fake_loan_terms();
		let expiration_block = T::BlockNumber::one();

		let (address_id,ask_id,guid) = generate_ask::<T>(&who,&terms,&expiration_block,false,0).unwrap();

	}: _(RawOrigin::Signed(who),address_id,terms,expiration_block,guid.into_bounded())

	add_bid_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let who:T::AccountId = borrower_account::<T>(true);

		let loan_terms = fake_loan_terms();

		let expiration_block = T::BlockNumber::one();

		let (address_id,bid_id,guid) = generate_bid::<T>(&who,&loan_terms,&expiration_block,false,0).unwrap();

	}:_(RawOrigin::Signed(who),address_id,loan_terms,expiration_block,guid.into_bounded())

	add_offer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let loan_terms = fake_loan_terms();
		let expiration_block = T::BlockNumber::one();

		let (_, ask_id, bid_id) = generate_offer::<T>(&lender,&loan_terms,&expiration_block,false,0u8).unwrap();

	}: _(RawOrigin::Signed(lender), ask_id, bid_id, expiration_block)

	add_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender = lender_account::<T>(false);
		let borrower= borrower_account::<T>(true);
		let loan_terms = fake_loan_terms();
		let expiration_block = T::BlockNumber::one();

		let (offer_id,ask_id,bid_id) = generate_offer::<T>(&lender, &loan_terms, &expiration_block, true,0u8).unwrap();

	}: _(RawOrigin::Signed(borrower), offer_id, expiration_block)

	add_authority {
		let root = RawOrigin::Root;
		let who = authority_account::<T>(false);
	}: _(root, who)

	persist_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let authority = authority_account::<T>(true);
		<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let deadline = T::BlockNumber::one();
		// pending task does not matter
		let transfer = generate_transfer::<T>(deal_id,false,false,0u8);
		let task_output = crate::TaskOutput::from(transfer);
	}: persist_task_output(RawOrigin::Signed(authority), deadline, task_output)

	fail_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let authority = authority_account::<T>(true);
		<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (transfer_id, _)= generate_transfer::<T>(deal_id,false,true,0u8);
		let cause = crate::ocw::VerificationFailureCause::TaskFailed;
		let deadline = T::BlockNumber::one();
		let task_id = crate::TaskId::from(transfer_id);
	}: fail_task(RawOrigin::Signed(authority), deadline, task_id, cause)

	fund_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (transfer_id,_) = generate_transfer::<T>(deal_id.clone(),true,false,0u8);

	}: _(RawOrigin::Signed(lender), deal_id, transfer_id)

	lock_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower:T::AccountId = borrower_account::<T>(true);

		let (deal_id,_) = generate_funded_deal::<T>(true,0u8).unwrap();

	}: _(RawOrigin::Signed(borrower), deal_id)

	register_funding_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (_,transfer) = generate_transfer::<T>(deal_id.clone(),false,false,0u8);
	}: _(RawOrigin::Signed(lender),transfer.kind,deal_id,transfer.tx_id)

	register_repayment_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower: T::AccountId = borrower_account::<T>(true);
		let repayment_amount = ExternalAmount::from(1);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (_,transfer) = generate_transfer::<T>(deal_id.clone(),false,true,0u8);
	}: _(RawOrigin::Signed(borrower),transfer.kind,repayment_amount,deal_id,transfer.tx_id)

	close_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower: T::AccountId = borrower_account::<T>(true);
		let (deal_id,_) = generate_locked_deal::<T>(true).unwrap();
		let (transfer_id, _) = generate_transfer::<T>(deal_id.clone(),true,true,0u8);

	}: _(RawOrigin::Signed(borrower),deal_id,transfer_id)

	exempt {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
	}:_(RawOrigin::Signed(lender),deal_id)

	register_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender:T::AccountId = lender_account::<T>(true);
		let lender_addr_id = register_eth_addr::<T>(&lender,"lender");
		let terms = fake_loan_terms();
		let expiry = T::BlockNumber::one();
		let ask_guid = "ask_guid".as_bytes();
		let bid_guid = "bid_guid".as_bytes();
		let payload = {
			expiry.encode().into_iter()
				.chain(ask_guid.encode())
				.chain(bid_guid.encode())
				.chain(terms.encode())
				.collect::<Vec<u8>>()
		};

		let ktypeid = KeyTypeId(*b"dumy");
		let pkey = ecdsa_generate(ktypeid, None);
		let borrower = T::Signer::from(pkey).into_account();

		let borrower_addr_id = register_eth_addr::<T>(&borrower,"borrower");
		let signature = ecdsa_sign(ktypeid, &pkey, &payload[..]).expect("ecdsa signature");


	}: _(RawOrigin::Signed(lender),lender_addr_id,borrower_addr_id,terms,expiry,ask_guid.into_bounded(),bid_guid.into_bounded(),pkey.into(),signature.into())

	remove_authority {
		let root = RawOrigin::Root;
		let who = authority_account::<T>(false);
		<Creditcoin<T>>::add_authority(root.clone().into(), who.clone()).unwrap();
	}: _(root, who)

	register_address_v2 {
		let who: T::AccountId = lender_account::<T>(false);
		let ktypeid = KeyTypeId(*b"dumy");
		let seed = "//who".as_bytes().to_vec();
		let pkey = ecdsa_generate(ktypeid, Some(seed));
		let address = EVMAddress::from_public(&pkey);
		let message = sp_io::hashing::sha2_256(who.encode().as_slice());
		let signature = ecdsa_sign(ktypeid, &pkey, &message).expect("ecdsa signature");
		let proof = OwnershipProof::EthSign(signature);
	}: _(RawOrigin::Signed(who), Blockchain::Ethereum, address, proof)

	set_gate_contract {
		let root = RawOrigin::Root;
		let contract = DeployedContract::default();
	}: _(root, contract)

	set_gate_faucet {
		let root = RawOrigin::Root;
		let addr: T::AccountId = lender_account::<T>(false);
	}: _(root, addr)
}

fn generate_funded_deal<T: Config>(
	fund: bool,
	seed: u8,
) -> Result<(DealOrderId<T::BlockNumber, T::Hash>, TransferId<T::Hash>), Error<T>> {
	let deal_id = generate_deal::<T>(true, seed).unwrap();
	let (transfer_id, _) = generate_transfer::<T>(deal_id.clone(), true, false, seed);
	let lender: T::AccountId = lender_account::<T>(true);

	if fund {
		Creditcoin::<T>::fund_deal_order(
			RawOrigin::Signed(lender).into(),
			deal_id.clone(),
			transfer_id.clone(),
		)
		.unwrap();
	}
	Ok((deal_id, transfer_id))
}

fn generate_transfer<T: Config>(
	deal_id: DealOrderId<T::BlockNumber, T::Hash>,
	insert: bool,
	swap_sender: bool,
	seed: u8,
) -> (TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>) {
	let (raw_tx, gain, who) = if swap_sender {
		(
			format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620{seed:02x}"),
			1u64,
			borrower_account::<T>(true),
		)
	} else {
		(
			format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a621{seed:02x}"),
			0u64,
			lender_account::<T>(true),
		)
	};

	let tx = raw_tx.as_bytes().into_bounded();
	let transfer_id = TransferId::new::<T>(&Blockchain::Ethereum, &tx);

	let order = try_get_id!(DealOrders<T>, &deal_id, NonExistentDealOrder).unwrap();

	let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

	let (transfer, _) = if swap_sender {
		Creditcoin::<T>::generate_transfer(
			who,
			order.borrower_address_id,
			order.lender_address_id,
			TransferKind::Ethless(contract),
			gain.into(),
			deal_id.into(),
			tx,
		)
		.unwrap()
	} else {
		Creditcoin::<T>::generate_transfer(
			who,
			order.lender_address_id,
			order.borrower_address_id,
			TransferKind::Ethless(contract),
			order.terms.amount,
			deal_id.into(),
			tx,
		)
		.unwrap()
	};

	if insert {
		Transfers::<T>::insert(&transfer_id, transfer.clone());
	}

	(transfer_id, transfer)
}

fn generate_locked_deal<T: Config>(
	lock: bool,
) -> Result<(DealOrderId<T::BlockNumber, T::Hash>, TransferId<T::Hash>), Error<T>> {
	let (deal_id, transfer_id) = generate_funded_deal::<T>(true, 0u8).unwrap();

	let borrower: T::AccountId = borrower_account::<T>(false);

	if lock {
		Creditcoin::<T>::lock_deal_order(RawOrigin::Signed(borrower).into(), deal_id.clone())
			.unwrap();
	}
	Ok((deal_id, transfer_id))
}

fn borrower_account<T: Config>(whitelist: bool) -> T::AccountId {
	let borrower = account("borrower", 1, 1);
	if whitelist {
		whitelist_account!(borrower);
	}
	borrower
}

fn authority_account<T: Config>(whitelist: bool) -> T::AccountId {
	let authority = account("authority", 1, 1);
	if whitelist {
		whitelist_account!(authority);
	}
	authority
}

fn lender_account<T: Config>(whitelist: bool) -> T::AccountId {
	let lender = account("lender", 1, 1);
	if whitelist {
		whitelist_account!(lender);
	}
	lender
}

fn generate_deal<T: Config>(
	insert: bool,
	seed: u8,
) -> Result<DealOrderId<T::BlockNumber, T::Hash>, crate::Error<T>> {
	let lender = lender_account::<T>(true);
	let terms = fake_loan_terms();
	let expiration_block = T::BlockNumber::one();

	let borrower = borrower_account::<T>(false);
	let origin = RawOrigin::Signed(borrower).into();
	let (offer_id, _, _) = generate_offer::<T>(&lender, &terms, &expiration_block, true, seed)?;

	let deal_id = DealOrderId::new::<T>(expiration_block, &offer_id);

	if insert {
		Creditcoin::<T>::add_deal_order(origin, offer_id, expiration_block).unwrap();
	}

	Ok(deal_id)
}

fn generate_offer<T: Config>(
	who: &T::AccountId,
	loan_terms: &LoanTerms,
	expiration_block: &T::BlockNumber,
	call: bool,
	seed: u8,
) -> Result<
	(
		OfferId<T::BlockNumber, T::Hash>,
		AskOrderId<T::BlockNumber, T::Hash>,
		BidOrderId<T::BlockNumber, T::Hash>,
	),
	crate::Error<T>,
> {
	let origin = RawOrigin::Signed(who.clone());

	let (_, ask_id, _) = generate_ask::<T>(who, loan_terms, expiration_block, true, seed)?;
	let borrower: T::AccountId = borrower_account::<T>(false);
	let (_, bid_id, _) = generate_bid::<T>(&borrower, loan_terms, expiration_block, true, seed)?;

	let offer_id = OfferId::new::<T>(*expiration_block, &ask_id, &bid_id);

	if call {
		Creditcoin::<T>::add_offer(
			origin.into(),
			ask_id.clone(),
			bid_id.clone(),
			*expiration_block,
		)
		.unwrap();
	}

	Ok((offer_id, ask_id, bid_id))
}

fn register_eth_addr<T: Config>(who: &T::AccountId, seed: &str) -> AddressId<<T>::Hash> {
	let ktypeid = KeyTypeId(*b"dumy");
	let pkey = ecdsa_generate(ktypeid, Some(format!("//{seed}").as_bytes().to_vec()));
	let address = EVMAddress::from_public(&pkey);
	let address_id = crate::AddressId::new::<T>(&Blockchain::Ethereum, &address);

	let message = sp_io::hashing::sha2_256(who.encode().as_slice());
	let signature = ecdsa_sign(ktypeid, &pkey, &message).expect("ecdsa signature");

	let origin = RawOrigin::Signed(who.clone());
	Creditcoin::<T>::register_address(origin.into(), Blockchain::Ethereum, address, signature)
		.unwrap();

	address_id
}

fn generate_ask<T: Config>(
	who: &T::AccountId,
	loan_terms: &LoanTerms,
	expiration_block: &T::BlockNumber,
	call: bool,
	seed: u8,
) -> Result<(AddressId<<T>::Hash>, AskOrderId<T::BlockNumber, T::Hash>, Vec<u8>), crate::Error<T>> {
	let secretkey = &format!("lender{seed:02x}")[..];
	let address_id = register_eth_addr::<T>(who, secretkey);
	let guid = format!("ask_guid{seed:02x}");
	let guid = guid.as_bytes();

	let ask_order_id = AskOrderId::new::<T>(*expiration_block, guid);
	let origin = RawOrigin::Signed(who.clone());
	if call {
		Creditcoin::<T>::add_ask_order(
			origin.into(),
			address_id.clone(),
			loan_terms.clone(),
			*expiration_block,
			guid.into_bounded(),
		)
		.unwrap();
	}

	Ok((address_id, ask_order_id, guid.to_vec()))
}

fn generate_bid<T: Config>(
	who: &T::AccountId,
	loan_terms: &LoanTerms,
	expiration_block: &T::BlockNumber,
	call: bool,
	seed: u8,
) -> Result<(AddressId<<T>::Hash>, BidOrderId<T::BlockNumber, T::Hash>, Vec<u8>), crate::Error<T>> {
	let secretkey = &format!("borrower{seed:02x}")[..];
	let address_id = register_eth_addr::<T>(who, secretkey);
	let guid = format!("bid_guid{seed:02x}");
	let guid = guid.as_bytes();

	let bid_order_id = BidOrderId::new::<T>(*expiration_block, guid);
	let origin = RawOrigin::Signed(who.clone());

	if call {
		Creditcoin::<T>::add_bid_order(
			origin.into(),
			address_id.clone(),
			loan_terms.clone(),
			*expiration_block,
			guid.into_bounded(),
		)
		.unwrap();
	}

	Ok((address_id, bid_order_id, guid.to_vec()))
}

pub(crate) fn fake_deal_id<T: SystemConfig>(
	expiration_block: BlockNumberFor<T>,
	offer_id: &OfferId<T::BlockNumber, T::Hash>,
) -> DealOrderId<T::BlockNumber, T::Hash> {
	DealOrderId::new::<T>(expiration_block, offer_id)
}

fn fake_transfer_id<T: Config>(seed: u32) -> TransferId<T::Hash> {
	let tx_id = format!("somefaketransfertxid{seed}");
	crate::TransferId::new::<T>(&Blockchain::Ethereum, tx_id.as_bytes())
}

fn insert_fake_deal<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	kind: DealKind,
	seed: u32,
) {
	let ask_id = fake_ask_id::<T>(seed, expiration_block);
	let bid_id = fake_bid_id::<T>(seed, expiration_block);
	let address_id = fake_address_id::<T>(seed);
	let offer_id = fake_offer_id::<T>(expiration_block, &ask_id, &bid_id);
	let deal_id = fake_deal_id::<T>(expiration_block, &offer_id);
	let deal = crate::DealOrder::<_, _, _, T::Moment> {
		block: None,
		blockchain: Blockchain::Ethereum,
		borrower: who.clone(),
		borrower_address_id: address_id.clone(),
		lender_address_id: address_id,
		expiration_block,
		lock: None,
		funding_transfer_id: match kind {
			DealKind::Funded => Some(fake_transfer_id::<T>(seed)),
			_ => None,
		},
		offer_id,
		repayment_transfer_id: None,
		terms: fake_loan_terms(),
		timestamp: pallet_timestamp::Pallet::<T>::now(),
	};

	crate::DealOrders::<T>::insert_id(deal_id, deal);
}

pub(crate) fn generate_fake_unverified_transfer<T: SystemConfig + TimestampConfig + Config>(
	who: &T::AccountId,
	deadline: BlockNumberFor<T>,
	seed: u32,
) -> crate::types::UnverifiedTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment> {
	let from_external = format!("somefakefromext{seed}").as_bytes().into_bounded();
	let to_external = format!("somefaketoext{seed}").as_bytes().into_bounded();
	crate::UnverifiedTransfer {
		deadline,
		from_external,
		to_external,
		transfer: crate::Transfer {
			account_id: who.clone(),
			amount: ExternalAmount::from(1),
			block: System::<T>::block_number(),
			blockchain: Blockchain::Ethereum,
			from: fake_address_id::<T>(seed - 1),
			to: fake_address_id::<T>(seed),
			is_processed: false,
			kind: TransferKind::Native,
			order_id: OrderId::Deal(fake_deal_id::<T>(
				deadline,
				&fake_offer_id::<T>(
					deadline,
					&fake_ask_id::<T>(seed, deadline),
					&fake_bid_id::<T>(seed, deadline),
				),
			)),
			tx_id: format!("{seed:03x}").as_bytes().into_bounded(),
			timestamp: None,
		},
	}
}
