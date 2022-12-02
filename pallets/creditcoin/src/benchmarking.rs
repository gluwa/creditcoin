#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::benchmarking::alloc::format;
use crate::helpers::{EVMAddress, HexToAddress, PublicToAddress, RefSliceOfTExt};
use crate::ocw::errors::VerificationFailureCause as Cause;
use crate::ocw::tasks::collect_coins::testing_constants::CHAIN;
use crate::types::Blockchain;
use crate::Duration;
#[allow(unused)]
use crate::Pallet as Creditcoin;
use crate::{types::Currency::Evm as CurrencyEvm, EvmTransferKind};
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
use sp_runtime::traits::One;
use sp_runtime::traits::{IdentifyAccount, UniqueSaturatedFrom};

#[derive(Clone, Copy, PartialEq, Eq)]
enum DealKind {
	Funded,
	Unfunded,
}

benchmarks! {
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
		//insert u unverifiedtransfers
		let u in 0..255;
		//insert c unverifiedcollectedcoins
		let c in 0..255;

		<Timestamp<T>>::set_timestamp(1u32.into());

		let lender = lender_account::<T>(false);
		let borrower = borrower_account::<T>(false);

		let terms = loan_terms::<T>();

		let expiration_block = T::BlockNumber::one();
		//generate this many filler asks
		for i in 0..a {
			insert_fake_ask::<T>(&borrower, expiration_block, None, i);
		}
		//generate this many filler bids
		for i in 0..b {
			insert_fake_bid::<T>(&lender, expiration_block, None, i);
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

		for i in 0..u {
			insert_fake_unverified_transfer::<T>(&lender, expiration_block, i);
		}

		let deadline = expiration_block;

		for i in 0..c {
			let collector: T::AccountId = lender_account::<T>(true);
			let evm_address = format!("{:03x}",i).as_bytes() .into_bounded();
			let address_id = AddressId::new::<T>(&CHAIN, &evm_address);
			let entry = Address { blockchain: CHAIN, value: evm_address.clone(), owner: collector.clone() };
			<Addresses<T>>::insert(address_id, entry);

			let tx_id = format!("{:03x}",i) .as_bytes() .into_bounded();
			let collected_coins_id = CollectedCoinsId::new::<T>(&CHAIN, &tx_id);

			let pending = types::UnverifiedCollectedCoins { to: evm_address.clone(), tx_id: tx_id.clone() , contract: Default::default()};

			crate::PendingTasks::<T>::insert(deadline, crate::TaskId::from(collected_coins_id), crate::Task::from(pending));
		}

	}: { Creditcoin::<T>::on_initialize(deadline) }
	verify {}

	register_address {
		let who: T::AccountId = lender_account::<T>(false);
		let ktypeid = KeyTypeId(*b"dumy");
		let seed = "//who".as_bytes().to_vec();
		let pkey = ecdsa_generate(ktypeid, Some(seed));
		let address = EVMAddress::from_public(&pkey);
		let message = sp_io::hashing::sha2_256(who.encode().as_slice());
		let signature = ecdsa_sign(ktypeid, &pkey, &message).expect("ecdsa signature");

	}: _(RawOrigin::Signed(who), Blockchain::ETHEREUM, address,signature)

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
		let terms = loan_terms::<T>();
		let expiration_block = T::BlockNumber::one();

		let (address_id,ask_id,guid) = generate_ask::<T>(&who,&terms,&expiration_block,false,0).unwrap();

	}: _(RawOrigin::Signed(who),address_id,terms,expiration_block,guid.into_bounded())

	add_bid_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let who:T::AccountId = borrower_account::<T>(true);

		let loan_terms = loan_terms::<T>();

		let expiration_block = T::BlockNumber::one();

		let (address_id,bid_id,guid) = generate_bid::<T>(&who,&loan_terms,&expiration_block,false,0).unwrap();

	}:_(RawOrigin::Signed(who),address_id,loan_terms,expiration_block,guid.into_bounded())

	add_offer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let loan_terms = loan_terms::<T>();
		let expiration_block = T::BlockNumber::one();

		let (_, ask_id, bid_id) = generate_offer::<T>(&lender,&loan_terms,&expiration_block,false,0u8).unwrap();

	}: _(RawOrigin::Signed(lender), ask_id, bid_id, expiration_block)

	add_deal_order {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender = lender_account::<T>(false);
		let borrower= borrower_account::<T>(true);
		let loan_terms = loan_terms::<T>();
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
		let transfer = generate_transfer::<T>(deal_id,false,false,true,0u8);
		let task_output = crate::TaskOutput::from(transfer);
	}: persist_task_output(RawOrigin::Signed(authority), deadline, task_output)

	fail_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let authority = authority_account::<T>(true);
		<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (transfer_id, _)= generate_transfer::<T>(deal_id,false,false,true,0u8);
		let cause = crate::ocw::VerificationFailureCause::TaskFailed;
		let deadline = T::BlockNumber::one();
		let task_id = crate::TaskId::from(transfer_id);
	}: fail_task(RawOrigin::Signed(authority), deadline, task_id, cause)

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

	register_funding_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (_,transfer) = generate_transfer::<T>(deal_id.clone(),false,false,true,0u8);
	}: register_funding_transfer(RawOrigin::Signed(lender),transfer.kind,deal_id,transfer.tx_id)

	register_repayment_transfer {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower: T::AccountId = borrower_account::<T>(true);
		let repayment_amount = ExternalAmount::from(1);
		let deal_id = generate_deal::<T>(true,0u8).unwrap();
		let (_,transfer) = generate_transfer::<T>(deal_id.clone(),false,true,true,0u8);
	}: register_repayment_transfer(RawOrigin::Signed(borrower),transfer.kind,repayment_amount,deal_id,transfer.tx_id)

	register_funding_transfer_legacy {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let lender: T::AccountId = lender_account::<T>(true);
		loan_terms::<T>();
		let deal_id = generate_deal_legacy::<T>(true,1u8).unwrap();
		let tx = format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620{:02x}", 0u8).as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".hex_to_address();
	}: register_funding_transfer_legacy(RawOrigin::Signed(lender),LegacyTransferKind::Ethless(contract),deal_id,tx)

	register_repayment_transfer_legacy {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let borrower: T::AccountId = borrower_account::<T>(true);
		loan_terms::<T>();
		let repayment_amount = ExternalAmount::from(1);
		let deal_id = generate_deal_legacy::<T>(true,0u8).unwrap();
		let tx = format!("0xcb13b65dd4d9d7f3cb8fcddeb442dfdf767403f8a9e5fe8587859225f8a620{:02x}", 0u8).as_bytes().into_bounded();
		let contract = "0x0ad1439a0e0bfdcd49939f9722866651a4aa9b3c".hex_to_address();
	}: register_repayment_transfer_legacy(RawOrigin::Signed(borrower),LegacyTransferKind::Ethless(contract),repayment_amount,deal_id,tx)

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
		let terms = loan_terms::<T>();
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

	request_collect_coins {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let collector: T::AccountId = lender_account::<T>(true);
		let collector_addr_id = register_eth_addr::<T>(&collector, "collector");
		let address = Creditcoin::<T>::addresses(&collector_addr_id).unwrap();
		let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab"
			.as_bytes()
			.into_bounded();
	}: _( RawOrigin::Signed(collector), address.value, tx_id)

	fail_collect_coins {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let authority = authority_account::<T>(true);
		<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
		let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab".as_bytes();
		let collected_coins_id = crate::CollectedCoinsId::new::<T>(&CHAIN, tx_id);
		let deadline = System::<T>::block_number() + <<T as crate::Config>::UnverifiedTaskTimeout as Get<T::BlockNumber>>::get();
		let task_id = crate::TaskId::from(collected_coins_id);
	}: fail_task(RawOrigin::Signed(authority), deadline, task_id, Cause::AbiMismatch)

	persist_collect_coins {
		<Timestamp<T>>::set_timestamp(1u32.into());
		let authority = authority_account::<T>(true);
		<Creditcoin<T>>::add_authority(RawOrigin::Root.into(), authority.clone()).unwrap();
		let collector: T::AccountId = lender_account::<T>(true);
		let collector_addr_id = register_eth_addr::<T>(&collector, "collector");
		let tx_id = "40be73b6ea10ef3da3ab33a2d5184c8126c5b64b21ae1e083ee005f18e3f5fab"
			.as_bytes()
			.into_bounded();
		let collected_coins_id = crate::CollectedCoinsId::new::<T>(&CHAIN, &tx_id);
		let amount = T::Balance::unique_saturated_from(Balances::<T>::minimum_balance());
		let collected_coins =
			crate::types::CollectedCoins::<T::Hash, T::Balance> { to: collector_addr_id, amount, tx_id };
		let deadline = System::<T>::block_number() + <<T as crate::Config>::UnverifiedTaskTimeout as Get<T::BlockNumber>>::get();
		let task_output = crate::TaskOutput::from((collected_coins_id, collected_coins));
	}: persist_task_output(RawOrigin::Signed(authority), deadline, task_output)

	remove_authority {
		let root = RawOrigin::Root;
		let who = authority_account::<T>(false);
		<Creditcoin<T>>::add_authority(root.clone().into(), who.clone()).unwrap();
	}: _(root, who)

	register_currency {
		let root = RawOrigin::Root;
		let currency = CurrencyEvm(
			crate::EvmCurrencyType::SmartContract(
				"0x0000000000000000000000000000000000000000".hex_to_address(),
				[EvmTransferKind::Ethless].into_bounded(),
			),
			EvmInfo { chain_id: 0.into() },
		);
	}: _(root, currency)

	set_collect_coins_contract {
		let root = RawOrigin::Root;
		let contract = GCreContract::default();
	}: _(root, contract)
}

//impl_benchmark_test_suite!(Creditcoin, crate::mock::new_test_ext(), crate::mock::Test);
fn loan_terms<T: Config>() -> LoanTerms<T::Hash> {
	let currency = register_fake_currency::<T>();
	LoanTerms {
		amount: 10u64.into(),
		interest_rate: InterestRate {
			rate_per_period: 1,
			decimals: 1,
			period: Duration::from_millis(100),
			interest_type: InterestType::Simple,
		},
		term_length: Duration::new(1u64, 0u32),
		currency,
	}
}

fn loan_terms_legacy<T: Config>() -> LoanTerms<T::Hash> {
	LoanTerms {
		amount: 10u64.into(),
		interest_rate: InterestRate {
			rate_per_period: 1,
			decimals: 1,
			period: Duration::from_millis(100),
			interest_type: InterestType::Simple,
		},
		term_length: Duration::new(1u64, 0u32),
		currency: CurrencyId::placeholder(),
	}
}

fn register_fake_currency<T: Config>() -> CurrencyId<T::Hash> {
	let currency = CurrencyEvm(
		EvmCurrencyType::SmartContract(
			"0x0000000000000000000000000000000000000000".hex_to_address(),
			[EvmTransferKind::Ethless].into_bounded(),
		),
		EvmInfo { chain_id: 1.into() },
	);
	let id = CurrencyId::new::<T>(&currency);
	crate::Currencies::<T>::insert(&id, currency);
	id
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
	let transfer_id = TransferId::new::<T>(&Blockchain::ETHEREUM, &tx);

	if swap_sender {
		Creditcoin::<T>::register_repayment_transfer(
			RawOrigin::Signed(who).into(),
			TransferKind::Evm(EvmTransferKind::Ethless),
			gain.into(),
			deal_id,
			tx,
		)
		.unwrap();
	} else {
		Creditcoin::<T>::register_funding_transfer(
			RawOrigin::Signed(who).into(),
			TransferKind::Evm(EvmTransferKind::Ethless),
			deal_id,
			tx,
		)
		.unwrap();
	}

	let transfer = PendingTasks::<T>::iter_values()
		.find_map(|task| match task {
			crate::Task::VerifyTransfer(ut) => {
				let transfer = &ut.transfer;
				let seek_id = TransferId::new::<T>(&transfer.blockchain, &transfer.tx_id);
				if transfer_id == seek_id {
					Some(ut)
				} else {
					None
				}
			},
			_ => None,
		})
		.unwrap()
		.transfer;
	if kill_unverified {
		let to_remove: Vec<_> = PendingTasks::<T>::iter_keys()
			.filter(|(_, id)| matches!(id, crate::TaskId::VerifyTransfer(..)))
			.collect();
		for (deadline, id) in to_remove {
			PendingTasks::<T>::remove(deadline, id);
		}
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

fn generate_deal_legacy<T: Config>(
	insert: bool,
	seed: u8,
) -> Result<DealOrderId<T::BlockNumber, T::Hash>, crate::Error<T>> {
	let terms = loan_terms_legacy::<T>();
	let expiration_block = T::BlockNumber::one();

	let borrower = borrower_account::<T>(false);
	let origin = RawOrigin::Signed(borrower).into();
	let (offer_id, _, _) = generate_offer_legacy::<T>(&terms, &expiration_block, true, seed)?;

	let deal_id = DealOrderId::new::<T>(expiration_block, &offer_id);

	if insert {
		Creditcoin::<T>::add_deal_order(origin, offer_id, expiration_block).unwrap();
	}

	Ok(deal_id)
}

fn generate_deal<T: Config>(
	insert: bool,
	seed: u8,
) -> Result<DealOrderId<T::BlockNumber, T::Hash>, crate::Error<T>> {
	let lender = lender_account::<T>(true);
	let terms = loan_terms::<T>();
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

fn generate_offer_legacy<T: Config>(
	loan_terms: &LoanTerms<T::Hash>,
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
	let lender = lender_account::<T>(false);

	let origin = RawOrigin::Signed(lender.clone());

	let ask_id =
		insert_fake_ask::<T>(&lender, *expiration_block, Some(loan_terms.clone()), seed.into());
	let borrower: T::AccountId = borrower_account::<T>(false);
	let bid_id = insert_fake_bid::<T>(
		&borrower,
		*expiration_block,
		Some(loan_terms.clone()),
		u32::from(seed) + 1u32,
	);

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

fn generate_offer<T: Config>(
	who: &T::AccountId,
	loan_terms: &LoanTerms<T::Hash>,
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
	let address_id = crate::AddressId::new::<T>(&Blockchain::ETHEREUM, &address);

	let message = sp_io::hashing::sha2_256(who.encode().as_slice());
	let signature = ecdsa_sign(ktypeid, &pkey, &message).expect("ecdsa signature");

	let origin = RawOrigin::Signed(who.clone());
	Creditcoin::<T>::register_address(origin.into(), Blockchain::ETHEREUM, address, signature)
		.unwrap();

	address_id
}

fn generate_ask<T: Config>(
	who: &T::AccountId,
	loan_terms: &LoanTerms<T::Hash>,
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
	loan_terms: &LoanTerms<T::Hash>,
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

fn fake_address_id<T: Config>(seed: u32) -> AddressId<T::Hash> {
	let address = format!("somefakeaddress{}", seed);
	crate::AddressId::new::<T>(&Blockchain::ETHEREUM, address.as_bytes())
}

fn insert_fake_address<T: Config>(owner: T::AccountId, seed: u32) -> AddressId<T::Hash> {
	let addr = format!("somefakeaddress{}", seed);
	let id = crate::AddressId::new::<T>(&Blockchain::ETHEREUM, addr.as_bytes());

	let address = crate::Address {
		blockchain: Blockchain::ETHEREUM,
		value: addr.as_bytes().into_bounded(),
		owner,
	};

	crate::Addresses::<T>::insert(&id, address);

	id
}

fn fake_ask_id<T: Config>(
	seed: u32,
	expiration_block: BlockNumberFor<T>,
) -> AskOrderId<T::BlockNumber, T::Hash> {
	let guid = format!("somefakeaskguid{}", seed);
	crate::AskOrderId::new::<T>(expiration_block, guid.as_bytes())
}

fn insert_fake_ask<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	terms: Option<LoanTerms<T::Hash>>,
	seed: u32,
) -> AskOrderId<T::BlockNumber, T::Hash> {
	let address_id = insert_fake_address::<T>(who.clone(), seed);
	let ask_id = fake_ask_id::<T>(seed, expiration_block);
	let ask = crate::AskOrder {
		block: System::<T>::block_number(),
		expiration_block,
		lender: who.clone(),
		lender_address_id: address_id,
		terms: AskTerms::try_from(terms.unwrap_or_else(|| loan_terms::<T>())).unwrap(),
	};

	crate::AskOrders::<T>::insert_id(&ask_id, ask);
	ask_id
}

fn fake_bid_id<T: Config>(
	seed: u32,
	expiration_block: BlockNumberFor<T>,
) -> BidOrderId<T::BlockNumber, T::Hash> {
	let guid = format!("somefakebidguid{}", seed);
	crate::BidOrderId::new::<T>(expiration_block, guid.as_bytes())
}

fn insert_fake_bid<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	terms: Option<LoanTerms<T::Hash>>,
	seed: u32,
) -> BidOrderId<T::BlockNumber, T::Hash> {
	let address_id = insert_fake_address::<T>(who.clone(), seed);
	let bid_id = fake_bid_id::<T>(seed, expiration_block);
	let bid = crate::BidOrder {
		block: System::<T>::block_number(),
		expiration_block,
		borrower: who.clone(),
		borrower_address_id: address_id,
		terms: BidTerms::try_from(terms.unwrap_or_else(|| loan_terms::<T>())).unwrap(),
	};

	crate::BidOrders::<T>::insert_id(&bid_id, bid);
	bid_id
}

fn fake_offer_id<T: Config>(
	expiration_block: BlockNumberFor<T>,
	ask_id: &AskOrderId<T::BlockNumber, T::Hash>,
	bid_id: &BidOrderId<T::BlockNumber, T::Hash>,
) -> OfferId<T::BlockNumber, T::Hash> {
	OfferId::new::<T>(expiration_block, ask_id, bid_id)
}

fn insert_fake_offer<T: Config>(
	who: &T::AccountId,
	expiration_block: BlockNumberFor<T>,
	seed: u32,
) {
	let ask_id = fake_ask_id::<T>(seed, expiration_block);
	let bid_id = fake_bid_id::<T>(seed, expiration_block);

	let offer_id = fake_offer_id::<T>(expiration_block, &ask_id, &bid_id);
	let offer = crate::Offer {
		ask_id,
		bid_id,
		block: System::<T>::block_number(),
		expiration_block,
		lender: who.clone(),
	};

	crate::Offers::<T>::insert_id(offer_id, offer);
}

fn fake_deal_id<T: Config>(
	expiration_block: BlockNumberFor<T>,
	offer_id: &OfferId<T::BlockNumber, T::Hash>,
) -> DealOrderId<T::BlockNumber, T::Hash> {
	DealOrderId::new::<T>(expiration_block, offer_id)
}

fn fake_transfer_id<T: Config>(seed: u32) -> TransferId<T::Hash> {
	let tx_id = format!("somefaketransfertxid{}", seed);
	crate::TransferId::new::<T>(&Blockchain::ETHEREUM, tx_id.as_bytes())
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
		terms: loan_terms::<T>(),
		timestamp: pallet_timestamp::Pallet::<T>::now(),
	};

	crate::DealOrders::<T>::insert_id(deal_id, deal);
}

fn insert_fake_unverified_transfer<T: Config>(
	who: &T::AccountId,
	deadline: BlockNumberFor<T>,
	seed: u32,
) {
	let from_external = format!("somefakefromext{}", seed).as_bytes().into_bounded();
	let to_external = format!("somefaketoext{}", seed).as_bytes().into_bounded();
	let transfer_id = fake_transfer_id::<T>(seed);
	let transfer = crate::UnverifiedTransfer {
		deadline,
		from_external,
		to_external,
		transfer: crate::Transfer {
			account_id: who.clone(),
			amount: ExternalAmount::from(1),
			block: System::<T>::block_number(),
			blockchain: Blockchain::ETHEREUM,
			from: fake_address_id::<T>(seed - 1),
			to: fake_address_id::<T>(seed),
			is_processed: false,
			kind: crate::TransferKind::Evm(crate::EvmTransferKind::Ethless),
			deal_order_id: fake_deal_id::<T>(
				deadline,
				&fake_offer_id::<T>(
					deadline,
					&fake_ask_id::<T>(seed, deadline),
					&fake_bid_id::<T>(seed, deadline),
				),
			),
			tx_id: format!("{:03x}", seed).as_bytes().into_bounded(),
			timestamp: None,
		},
		currency_to_check: crate::CurrencyOrLegacyTransferKind::TransferKind(
			LegacyTransferKind::Native,
		),
	};

	let task_id = TaskId::VerifyTransfer(transfer_id);
	let task = Task::VerifyTransfer(transfer);
	crate::PendingTasks::<T>::insert(deadline, task_id, task)
}
