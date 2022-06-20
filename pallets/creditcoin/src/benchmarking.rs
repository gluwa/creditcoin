#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::benchmarking::alloc::format;
use crate::helpers::{EVMAddress, PublicToAddress};
use crate::ocw::tasks::collect_coins::CONTRACT_CHAIN;
use crate::types::Blockchain;
use crate::Duration;
#[allow(unused)]
use crate::Pallet as Creditcoin;
use crate::{AskOrderId, InterestRate, InterestType, LoanTerms};
use frame_benchmarking::{account, benchmarks, whitelist_account, Zero};
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, Get},
};
use frame_system::pallet_prelude::*;
use frame_system::Pallet as System;
use frame_system::RawOrigin;
use pallet_balances::Pallet as Balances;
use pallet_timestamp::Pallet as Timestamp;
use sp_core::ecdsa;
use sp_io::crypto::{ecdsa_generate, ecdsa_sign};
use sp_runtime::traits::IdentifyAccount;
use sp_runtime::traits::One;

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

benchmarks! {
	on_initialize {
		//insert a askorders
		let a in 128..255;
		//insert b bidorders
		let b in 128..255;
		//insert o offers
		let o in 64..128;
		//insert d dealorders
		let d in 32..64;
		//insert f fundedorders
		let f in 16..32;
		//insert u unverifiedtransfers
		let u in 0..16;
		//insert c unverifiedcollectedcoins
		let c in 0..64;

		<Timestamp<T>>::set_timestamp(1u32.into());

		let lender = lender_account::<T>(false);

		let terms = get_all_fit_terms::<T>();

		let expiration_block = T::BlockNumber::one();
		//generate this many filler asks
		for i in o..a {
			let _ = generate_ask::<T>(&lender, &terms, &expiration_block, true,i as u8).unwrap();
		}
		let borrower = borrower_account::<T>(false);
		//generate this many filler bids
		for i in o..b {
			let _ = generate_bid::<T>(&borrower, &terms, &expiration_block, true, i as u8).unwrap();
		}
		//generate this many matching offers,bids,asks
		for i in d..o {
			let _ = generate_offer::<T>(&lender, &terms, &expiration_block, true,i as u8).unwrap();
		}
		//generate this many matching deals,offers,asks,bids
		for i in f..d{
			let _ = generate_deal::<T>(true,i as u8).unwrap();
		}
		//generate this many matching funded_deals with its deal,transfer,offer,ask and bid.
		for i in 0..f{
			let (deal_id, _) = generate_funded_deal::<T>(true, i as u8).unwrap();
			//generate this many unverified transfers
			if i < u {
				generate_transfer::<T>(deal_id, false, true, false, i as u8);
			}
		}

		let deadline = T::BlockNumber::one();

		for i in 0..c {
			let collector: T::AccountId = lender_account::<T>(true);
			let evm_address = format!("{:03x}",i).as_bytes() .into_bounded();
			let address_id = AddressId::new::<T>(&CONTRACT_CHAIN, &evm_address);
			let entry = Address { blockchain: CONTRACT_CHAIN, value: evm_address.clone(), owner: collector.clone() };
			<Addresses<T>>::insert(address_id, entry);

			let tx_id = format!("{:03x}",i) .as_bytes() .into_bounded();
			let collected_coins_id = CollectedCoinsId::new::<T>(&tx_id);

			let pending = types::UnverifiedCollectedCoins { to: evm_address.clone(), tx_id: tx_id.clone() };

			crate::PendingTasks::<T>::insert(deadline, crate::TaskId::from(collected_coins_id), crate::Task::from(pending));
		}

	}:{ Creditcoin::<T>::on_initialize(deadline) }
	verify {
	}

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

		let claimer = T::Signer::from(pubkey.clone()).into_account();
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
		let terms = get_all_fit_terms::<T>();
		let expiration_block = T::BlockNumber::one();

		let (address_id,ask_id,guid) = generate_ask::<T>(&who,&terms,&expiration_block,false,0).unwrap();

	}: _(RawOrigin::Signed(who),address_id,terms,expiration_block,guid.into_bounded())

	add_bid_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let who:T::AccountId = borrower_account::<T>(true);

		let loan_terms = get_all_fit_terms::<T>();

		let expiration_block = T::BlockNumber::one();

		let (address_id,bid_id,guid) = generate_bid::<T>(&who,&loan_terms,&expiration_block,false,0).unwrap();

	}:_(RawOrigin::Signed(who),address_id,loan_terms,expiration_block,guid.into_bounded())

	add_offer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let loan_terms = get_all_fit_terms::<T>();
		let expiration_block = T::BlockNumber::one();

		let (_, ask_id, bid_id) = generate_offer::<T>(&lender,&loan_terms,&expiration_block,false,0u8).unwrap();

	}: _(RawOrigin::Signed(lender), ask_id, bid_id, expiration_block)

	add_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender = lender_account::<T>(false);
		let borrower= borrower_account::<T>(true);
		let loan_terms = get_all_fit_terms::<T>();
		let expiration_block = T::BlockNumber::one();

		let (offer_id,ask_id,bid_id) = generate_offer::<T>(&lender, &loan_terms, &expiration_block, true,0u8).unwrap();

	}: _(RawOrigin::Signed(borrower), offer_id, expiration_block)

	add_authority {
		let root = RawOrigin::Root;
		let who = authority_account::<T>(false);
	}: _(root, who)

	// verify_transfer {
	// 	<Timestamp<T>>::set_timestamp(1u32.into());
	// 	let authority = authority_account::<T>(true);
	// 	<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
	// 	let deal_id = generate_deal::<T>(true,0u8).unwrap();
	// 	let expiry = T::BlockNumber::one();
	// 	let (_, transfer)= generate_transfer::<T>(deal_id,false,false,true,0u8);

	// }: _(RawOrigin::Signed(authority), expiry, transfer)

	// fail_transfer {
	// 	<Timestamp<T>>::set_timestamp(1u32.into());
	// 	let authority = authority_account::<T>(true);
	// 	<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
	// 	let deal_id = generate_deal::<T>(true,0u8).unwrap();
	// 	let (transfer_id, _)= generate_transfer::<T>(deal_id,false,false,true,0u8);
	// 	let cause = crate::ocw::VerificationFailureCause::TaskFailed;
	// 	let expiry = T::BlockNumber::one();
	// }: _(RawOrigin::Signed(authority), expiry, transfer_id, cause)

	fund_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (transfer_id,_) = generate_transfer::<T>(deal_id.clone(),true,false,true,0u8);

	}: _(RawOrigin::Signed(lender), deal_id, transfer_id)

	lock_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower:T::AccountId = borrower_account::<T>(true);

		let (deal_id,_) = generate_funded_deal::<T>(true,0u8).unwrap();

	}: _(RawOrigin::Signed(borrower), deal_id)

	register_transfer_ocw {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (_,transfer) = generate_transfer::<T>(deal_id.clone(),false,false,true,0u8);
	}: register_funding_transfer(RawOrigin::Signed(lender),transfer.kind,deal_id,transfer.tx_id)

	close_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower: T::AccountId = borrower_account::<T>(true);
		let (deal_id,_) = generate_locked_deal::<T>(true).unwrap();
		let (transfer_id, _) = generate_transfer::<T>(deal_id.clone(),true,true,true,0u8);

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
		let terms = get_all_fit_terms::<T>();
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
		let borrower = T::Signer::from(pkey.clone()).into_account();

		let borrower_addr_id = register_eth_addr::<T>(&borrower,"borrower");
		let signature = ecdsa_sign(ktypeid, &pkey, &payload[..]).expect("ecdsa signature");


	}: _(RawOrigin::Signed(lender),lender_addr_id,borrower_addr_id,terms,expiry,ask_guid.into_bounded(),bid_guid.into_bounded(),pkey.into(),signature.into())

	request_collect_coins{
		<Timestamp<T>>::set_timestamp(1u32.into());
		let collector: T::AccountId = lender_account::<T>(true);
		let collector_addr_id = register_eth_addr::<T>(&collector, "collector");
		let address = Creditcoin::<T>::addresses(&collector_addr_id).unwrap();
		let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab"
			.as_bytes()
			.into_bounded();
	}: _( RawOrigin::Signed(collector), address.value, tx_id)

	// fail_collect_coins{
	// 	<Timestamp<T>>::set_timestamp(1u32.into());
	// 	let authority = authority_account::<T>(true);
	// 	<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
	// 	let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab".as_bytes();
	// 	let collected_coins_id = crate::CollectedCoinsId::new::<T>(tx_id);
	// 	let deadline = System::<T>::block_number() + <<T as crate::Config>::UnverifiedTaskTimeout as Get<T::BlockNumber>>::get();

	// }: _(RawOrigin::Signed(authority), collected_coins_id, Cause::AbiMismatch, deadline)

	// persist_collect_coins{
	// 	<Timestamp<T>>::set_timestamp(1u32.into());
	// 	let authority = authority_account::<T>(true);
	// 	<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
	// 	let collector: T::AccountId = lender_account::<T>(true);
	// 	let collector_addr_id = register_eth_addr::<T>(&collector, "collector");
	// 	let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab"
	// 		.as_bytes()
	// 		.into_bounded();
	// 	let collected_coins_id = crate::CollectedCoinsId::new::<T>(&tx_id);

	// 	let collected_coins =
	// 		crate::types::CollectedCoins::<T::Hash, T::Balance> { to: collector_addr_id, amount: T::Balance::unique_saturated_from(1u32), tx_id };

	// 	let deadline = System::<T>::block_number() + <<T as crate::Config>::UnverifiedTaskTimeout as Get<T::BlockNumber>>::get();
	// }: _(RawOrigin::Signed(authority), collected_coins, deadline)

}

//impl_benchmark_test_suite!(Creditcoin, crate::mock::new_test_ext(), crate::mock::Test);
fn get_all_fit_terms<T: Config>() -> LoanTerms {
	LoanTerms {
		amount: 10u64.into(),
		interest_rate: InterestRate {
			rate_per_period: 1,
			decimals: 1,
			period: Duration::from_millis(100),
			interest_type: InterestType::Simple,
		},
		term_length: Duration::new(1u64, 0u32),
	}
}

fn generate_funded_deal<T: Config>(
	fund: bool,
	seed: u8,
) -> Result<(DealOrderId<T::BlockNumber, T::Hash>, TransferId<T::Hash>), Error<T>> {
	let deal_id = generate_deal::<T>(true, seed).unwrap();
	let (transfer_id, _) = generate_transfer::<T>(deal_id.clone(), true, false, true, seed);
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
	kill_unverified: bool,
	seed: u8,
) -> (TransferId<T::Hash>, Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>) {
	let (raw_tx, gain, who) = if swap_sender {
		(
			format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620{:02x}", seed),
			1u64,
			borrower_account::<T>(true),
		)
	} else {
		(
			format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a621{:02x}", seed),
			0u64,
			lender_account::<T>(true),
		)
	};
	let tx = raw_tx.as_bytes().into_bounded();
	let transfer_id = TransferId::new::<T>(&Blockchain::Ethereum, &tx);

	let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".as_bytes().into_bounded();

	if swap_sender {
		Creditcoin::<T>::register_repayment_transfer(
			RawOrigin::Signed(who).into(),
			TransferKind::Ethless(contract),
			gain.into(),
			deal_id,
			tx,
		)
		.unwrap();
	} else {
		Creditcoin::<T>::register_funding_transfer(
			RawOrigin::Signed(who).into(),
			TransferKind::Ethless(contract),
			deal_id,
			tx,
		)
		.unwrap();
	}

	let transfer = PendingTasks::<T>::iter_values()
		.filter_map(|task| match task {
			crate::Task::VerifyTransfer(transfer) => Some(transfer),
			_ => None,
		})
		.find(|ut| {
			let transfer = &ut.transfer;
			let seek_id = TransferId::new::<T>(&transfer.blockchain, &transfer.tx_id);
			transfer_id == seek_id
		})
		.unwrap()
		.transfer;
	if kill_unverified {
		PendingTasks::<T>::remove_all(None);
	}

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
	let terms = get_all_fit_terms::<T>();
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
	let pkey = ecdsa_generate(ktypeid, Some(format!("//{}", seed).as_bytes().to_vec()));
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
	let secretkey = &format!("lender{:02x}", seed)[..];
	let address_id = register_eth_addr::<T>(who, secretkey);
	let guid = format!("ask_guid{:02x}", seed);
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
	let secretkey = &format!("borrower{:02x}", seed)[..];
	let address_id = register_eth_addr::<T>(who, secretkey);
	let guid = format!("bid_guid{:02x}", seed);
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
