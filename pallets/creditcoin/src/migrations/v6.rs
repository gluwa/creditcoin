use super::v5;
use crate::Config;
use frame_support::generate_storage_alias;
use frame_support::pallet_prelude::*;

pub use v5::*;

pub use v5::Address as OldAddress;
pub use v5::AskOrder as OldAskOrder;
pub use v5::AskTerms as OldAskTerms;
pub use v5::BidOrder as OldBidOrder;
pub use v5::BidTerms as OldBidTerms;
pub use v5::Blockchain as OldBlockchain;
pub use v5::DealOrder as OldDealOrder;
pub use v5::LoanTerms as OldLoanTerms;
pub use v5::OrderId as OldOrderId;
pub use v5::Transfer as OldTransfer;
pub use v5::TransferKind as OldTransferKind;

use crate::Address;
use crate::AddressId;
use crate::Blockchain;
use crate::Transfer;
use crate::TransferId;

fn translate_blockchain(old: OldBlockchain) -> Option<Blockchain> {
	match old {
		OldBlockchain::Ethereum => Some(Blockchain::ETHEREUM),
		OldBlockchain::Rinkeby => Some(Blockchain::RINKEBY),
		// this assumes that Luniverse == mainnet luniverse, we may want to make the chain ID of the
		// old "Luniverse" variant on-chain-storage to make testnet work
		OldBlockchain::Luniverse => Some(Blockchain::LUNIVERSE),
		other => {
			log::warn!(
				"unexpected blockchain found on storage item: {:?}",
				core::str::from_utf8(other.as_bytes()).ok()
			);
			None
		},
	}
}

generate_storage_alias!(
	Creditcoin,
	Transfers<T: Config> => Map<(Identity, TransferId<T::Hash>), Transfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>>
);

generate_storage_alias!(
	Creditcoin,
	Addresses<T: Config> => Map<(Blake2_128Concat, AddressId<T::Hash>), Address<T::AccountId>>
);

#[allow(unreachable_code)]
pub(crate) fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let weight_each = T::DbWeight::get().reads_writes(1, 1);

	Transfers::<T>::translate::<OldTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>, _>(
		|_id, transfer| {
			weight = weight.saturating_add(weight_each);
			Some(Transfer {
				amount: transfer.amount,
				from: transfer.from,
				to: transfer.to,
				tx_id: transfer.tx_id,
				block: transfer.block,
				is_processed: transfer.is_processed,
				account_id: transfer.account_id,
				timestamp: transfer.timestamp,
				deal_order_id: match transfer.order_id {
					OldOrderId::Deal(id) => id,
					OldOrderId::Repayment(id) => {
						log::warn!(
							"Found unexpected repayment ID attached to a transfer: {:?}",
							id
						);
						return None;
					},
				},
				blockchain: translate_blockchain(transfer.blockchain)?,
				kind: todo!(),
			})
		},
	);

	Addresses::<T>::translate::<OldAddress<T::AccountId>, _>(|_id, address| {
		weight = weight.saturating_add(weight_each);
		Some(Address {
			blockchain: translate_blockchain(address.blockchain)?,
			value: address.value,
			owner: address.owner,
		})
	});

	weight
}
