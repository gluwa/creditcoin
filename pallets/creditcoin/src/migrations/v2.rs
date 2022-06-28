// `block` added to `DealOrder` and `timestamp` added to `Transfer`

use crate::{
	AddressId, Blockchain, Config, ExternalAmount, ExternalTxId, OfferId, OrderId, TransferId,
	TransferKind,
};
use frame_support::{generate_storage_alias, pallet_prelude::*, Identity, Twox64Concat};

use super::v1::DealOrder as OldDealOrder;
use super::v1::LoanTerms;

use crate::Transfer;

#[derive(Encode, Decode)]
struct OldTransfer<AccountId, BlockNum, Hash> {
	blockchain: Blockchain,
	kind: TransferKind,
	from: AddressId<Hash>,
	to: AddressId<Hash>,
	order_id: OrderId<BlockNum, Hash>,
	amount: ExternalAmount,
	tx: ExternalTxId,
	block: BlockNum,
	processed: bool,
	sighash: AccountId,
}

#[derive(Encode, Decode)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct DealOrder<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub offer_id: OfferId<BlockNum, Hash>,
	pub lender_address_id: AddressId<Hash>,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: LoanTerms,
	pub expiration_block: BlockNum,
	pub timestamp: Moment,
	pub block: Option<BlockNum>,
	pub funding_transfer_id: Option<TransferId<Hash>>,
	pub repayment_transfer_id: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub borrower: AccountId,
}

generate_storage_alias!(
	Creditcoin,
	DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

generate_storage_alias!(
	Creditcoin,
	Transfers<T: Config> => Map<(Identity, TransferId<T::Hash>), Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	DealOrders::<T>::translate::<OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_exp, _hash, deal| {
			weight = weight.saturating_add(weight_each);
			Some(DealOrder {
				blockchain: deal.blockchain,
				offer_id: deal.offer_id,
				lender_address_id: deal.lender_address_id,
				borrower_address_id: deal.borrower_address_id,
				terms: deal.terms,
				expiration_block: deal.expiration_block,
				timestamp: deal.timestamp,
				funding_transfer_id: deal.funding_transfer_id,
				lock: deal.lock,
				borrower: deal.borrower,
				repayment_transfer_id: deal.repayment_transfer_id,
				block: None,
			})
		},
	);

	Transfers::<T>::translate::<OldTransfer<T::AccountId, T::BlockNumber, T::Hash>, _>(
		|_id, transfer| {
			weight = weight.saturating_add(weight_each);
			Some(Transfer {
				blockchain: transfer.blockchain,
				kind: transfer.kind,
				from: transfer.from,
				to: transfer.to,
				order_id: transfer.order_id,
				amount: transfer.amount,
				tx_id: transfer.tx,
				block: transfer.block,
				is_processed: transfer.processed,
				account_id: transfer.sighash,
				timestamp: None,
			})
		},
	);

	weight
}

#[cfg(test)]
mod test {
	use core::convert::TryInto;

	use super::{Config, DealOrder, Identity, OldDealOrder, OldTransfer, Transfer, Twox64Concat};
	use crate::{
		mock::{ExtBuilder, Test},
		tests::TestInfo,
		Blockchain, DealOrderId, DoubleMapExt, Duration, OfferId, OrderId, TransferId,
		TransferKind,
	};
	use frame_support::generate_storage_alias;

	generate_storage_alias!(
		Creditcoin,
		DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
	);

	type OldDealOrders = DealOrders<Test>;

	generate_storage_alias!(
		Creditcoin,
		Transfers<T: Config> => Map<(Identity, TransferId<T::Hash>), OldTransfer<T::AccountId, T::BlockNumber, T::Hash>>
	);

	type OldTransfers = Transfers<Test>;

	#[test]
	fn deal_order_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();

			let deal_id = DealOrderId::with_expiration_hash::<Test>(100, [0u8; 32].into());
			let offer_id = OfferId::with_expiration_hash::<Test>(100, [1u8; 32].into());

			let old_deal = OldDealOrder {
				blockchain: Blockchain::Ethereum,
				offer_id,
				lender_address_id: test_info.lender.address_id,
				borrower_address_id: test_info.borrower.address_id,
				terms: super::LoanTerms {
					amount: 100u64.into(),
					interest_rate: super::super::v1::InterestRate {
						rate_per_period: 100,
						decimals: 4,
						period: Duration::from_millis(2000),
					},
					term_length: Duration::from_millis(10000),
				},
				expiration_block: 100,
				timestamp: 0,
				funding_transfer_id: None,
				repayment_transfer_id: None,
				lock: None,
				borrower: test_info.borrower.account_id,
			};

			OldDealOrders::insert_id(&deal_id, &old_deal);

			super::migrate::<Test>();

			let deal = super::DealOrders::<Test>::try_get_id(&deal_id).unwrap();

			assert_eq!(
				deal,
				DealOrder {
					blockchain: old_deal.blockchain,
					offer_id: old_deal.offer_id,
					lender_address_id: old_deal.lender_address_id,
					borrower_address_id: old_deal.borrower_address_id,
					terms: old_deal.terms,
					expiration_block: old_deal.expiration_block,
					timestamp: old_deal.timestamp,
					funding_transfer_id: old_deal.funding_transfer_id,
					repayment_transfer_id: old_deal.repayment_transfer_id,
					lock: old_deal.lock,
					borrower: old_deal.borrower,
					block: None,
				}
			);
		});
	}

	#[test]
	fn transfer_migrates() {
		ExtBuilder::default().build_and_execute(|| {
			let test_info = TestInfo::new_defaults();
			let blockchain = Blockchain::Ethereum;
			let transfer_id = TransferId::new::<Test>(&blockchain, &[0]);
			let old_transfer = OldTransfer {
				blockchain: Blockchain::Ethereum,
				kind: TransferKind::Native,
				from: test_info.lender.address_id,
				to: test_info.borrower.address_id,
				order_id: OrderId::Deal(DealOrderId::dummy()),
				amount: 100u64.into(),
				tx: vec![0u8; 32].try_into().unwrap(),
				block: 1,
				processed: false,
				sighash: test_info.borrower.account_id,
			};

			OldTransfers::insert(&transfer_id, &old_transfer);

			super::migrate::<Test>();

			let transfer = super::Transfers::<Test>::try_get(&transfer_id).unwrap();

			assert_eq!(
				transfer,
				Transfer {
					blockchain: old_transfer.blockchain,
					kind: old_transfer.kind,
					from: old_transfer.from,
					to: old_transfer.to,
					order_id: old_transfer.order_id,
					amount: old_transfer.amount,
					tx_id: old_transfer.tx,
					block: old_transfer.block,
					is_processed: old_transfer.processed,
					account_id: old_transfer.sighash,
					timestamp: None,
				}
			);
		});
	}
}
