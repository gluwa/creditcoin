use super::v5;
use crate::Config;
use frame_support::pallet_prelude::*;
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

pub(crate) fn migrate<T: Config>() -> Weight {
	todo!()
}
