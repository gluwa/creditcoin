use crate::{AddressId, Blockchain, Config, ExternalAmount, ExternalTxId, OrderId, TransferKind};
use frame_support::{generate_storage_alias, pallet_prelude::*, Identity, Twox64Concat};

use super::v1::DealOrder as OldDealOrder;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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

generate_storage_alias!(
	Creditcoin,
	DealOrders<T: Config> => DoubleMap<(Twox64Concat, T::BlockNumber), (Identity, T::Hash), crate::DealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

generate_storage_alias!(
	Creditcoin,
	Transfers<T: Config> => Map<(Identity, crate::TransferId<T::Hash>), crate::Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	DealOrders::<T>::translate::<OldDealOrder<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_exp, _hash, deal| {
			weight += weight_each;
			Some(crate::DealOrder {
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
			weight += weight_each;
			Some(crate::Transfer {
				blockchain: transfer.blockchain,
				kind: transfer.kind,
				from: transfer.from,
				to: transfer.to,
				order_id: transfer.order_id,
				amount: transfer.amount,
				tx_id: transfer.tx,
				block: transfer.block,
				processed: transfer.processed,
				sighash: transfer.sighash,
				timestamp: None,
			})
		},
	);

	weight
}
