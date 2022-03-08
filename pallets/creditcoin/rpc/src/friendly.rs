use pallet_creditcoin::*;
use serde::{Deserialize, Serialize};
use sp_core::Bytes;

type Hash = <creditcoin_node_runtime::Runtime as frame_system::Config>::Hash;
type AccountId = <creditcoin_node_runtime::Runtime as frame_system::Config>::AccountId;
type BlockNumber = <creditcoin_node_runtime::Runtime as frame_system::Config>::BlockNumber;
type Moment = <creditcoin_node_runtime::Runtime as pallet_timestamp::Config>::Moment;
type Balance = <creditcoin_node_runtime::Runtime as pallet_balances::Config>::Balance;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct _Address {
	pub id: Bytes,
	pub blockchain: String,
	pub value: String,
	pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Event {
	CtcTransfer {
		from: AccountId,
		to: AccountId,
		amount: String,
	},
	CtcDeposit {
		into: AccountId,
		amount: String,
	},
	CtcWithdraw {
		from: AccountId,
		amount: String,
	},
	RewardIssued {
		to: AccountId,
		amount: String,
	},
	AddressRegistered {
		address_id: AddressId<Hash>,
		address: Address<AccountId>,
	},

	TransferRegistered {
		transfer_id: TransferId<Hash>,
		transfer: Transfer<AccountId, BlockNumber, Hash>,
	},
	TransferVerified {
		transfer_id: TransferId<Hash>,
		transfer: Transfer<AccountId, BlockNumber, Hash>,
	},
	TransferProcessed {
		transfer_id: TransferId<Hash>,
		transfer: Transfer<AccountId, BlockNumber, Hash>,
	},

	AskOrderAdded {
		ask_id: AskOrderId<BlockNumber, Hash>,
		ask: AskOrder<AccountId, BlockNumber, Hash, Moment>,
	},

	BidOrderAdded {
		bid_id: BidOrderId<BlockNumber, Hash>,
		bid: BidOrder<AccountId, BlockNumber, Hash, Moment>,
	},

	OfferAdded {
		offer_id: OfferId<BlockNumber, Hash>,
		offer: Offer<AccountId, BlockNumber, Hash>,
	},

	DealOrderAdded {
		deal_id: DealOrderId<BlockNumber, Hash>,
		deal: DealOrder<AccountId, BlockNumber, Hash, Moment>,
	},
	DealOrderFunded {
		deal_id: DealOrderId<BlockNumber, Hash>,
		deal: DealOrder<AccountId, BlockNumber, Hash, Moment>,
	},
	DealOrderClosed {
		deal_id: DealOrderId<BlockNumber, Hash>,
		deal: DealOrder<AccountId, BlockNumber, Hash, Moment>,
	},

	LoanExempted {
		deal_id: DealOrderId<BlockNumber, Hash>,
		exempt_transfer_id: TransferId<Hash>,
	},

	LegacyWalletClaimed {
		new_account_id: AccountId,
		legacy_sighash: LegacySighash,
		claimed_balance: String,
	},
}

impl Event {
	pub fn from_runtime(event: creditcoin_node_runtime::Event) -> Option<Self> {
		Some(match event {
			creditcoin_node_runtime::Event::System(_) => None?,
			creditcoin_node_runtime::Event::Balances(e) => match e {
				pallet_balances::Event::Transfer { from, to, amount } => {
					Event::CtcTransfer { from, to, amount: amount.to_string() }
				},
				pallet_balances::Event::Deposit { who, amount } => {
					Event::CtcDeposit { into: who, amount: amount.to_string() }
				},
				pallet_balances::Event::Withdraw { who, amount } => {
					Event::CtcWithdraw { from: who, amount: amount.to_string() }
				},
				_ => None?,
			},
			creditcoin_node_runtime::Event::Rewards(e) => match e {
				pallet_rewards::Event::RewardIssued(to, amount) => {
					Event::RewardIssued { to, amount: amount.to_string() }
				},
				_ => None?,
			},
			creditcoin_node_runtime::Event::Sudo(_) => None?,
			creditcoin_node_runtime::Event::Creditcoin(e) => match e {
				pallet_creditcoin::Event::AddressRegistered(address_id, address) => {
					Event::AddressRegistered { address_id, address }
				},
				pallet_creditcoin::Event::TransferRegistered(transfer_id, transfer) => {
					Event::TransferRegistered { transfer_id, transfer }
				},
				pallet_creditcoin::Event::TransferVerified(transfer_id, transfer) => {
					Event::TransferVerified { transfer_id, transfer }
				},
				pallet_creditcoin::Event::TransferProcessed(transfer_id, transfer) => {
					Event::TransferProcessed { transfer_id, transfer }
				},
				pallet_creditcoin::Event::AskOrderAdded(ask_id, ask) => {
					Event::AskOrderAdded { ask_id, ask }
				},
				pallet_creditcoin::Event::BidOrderAdded(bid_id, bid) => {
					Event::BidOrderAdded { bid_id, bid }
				},
				pallet_creditcoin::Event::OfferAdded(offer_id, offer) => {
					Event::OfferAdded { offer_id, offer }
				},
				pallet_creditcoin::Event::DealOrderAdded(deal_id, deal) => {
					Event::DealOrderAdded { deal_id, deal }
				},
				pallet_creditcoin::Event::DealOrderFunded(deal_id, deal) => {
					Event::DealOrderFunded { deal_id, deal }
				},
				pallet_creditcoin::Event::DealOrderClosed(deal_id, deal) => {
					Event::DealOrderClosed { deal_id, deal }
				},
				pallet_creditcoin::Event::LoanExempted(deal_id, exempt_transfer_id) => {
					Event::LoanExempted { deal_id, exempt_transfer_id }
				},
				pallet_creditcoin::Event::LegacyWalletClaimed(
					new_account_id,
					legacy_sighash,
					claimed_balance,
				) => Event::LegacyWalletClaimed {
					new_account_id,
					legacy_sighash,
					claimed_balance: claimed_balance.to_string(),
				},
				_ => None?,
			},
		})
	}
}
