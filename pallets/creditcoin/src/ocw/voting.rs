use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use pallet_voting_oracle::MakeProposal;
use scale_info::TypeInfo;

use crate::{Config, UnverifiedTransfer};

#[derive(Encode, Decode, PartialEq, Eq, Clone, RuntimeDebug, TypeInfo)]
pub enum BaseTaskProposal<T: Config> {
	Transfer(UnverifiedTransfer<T::AccountId, T::BlockNumber, T::Hash, T::Moment>),
}

#[derive(Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, RuntimeDebug, TypeInfo)]
pub enum OracleData<Moment> {
	Transfer(Option<Moment>),
}

impl<T: Config> MakeProposal<T> for BaseTaskProposal<T>
where
	T: pallet_voting_oracle::Config<ProposalExtraData = OracleData<T::Moment>>,
{
	fn make_proposal(
		self,
		extra_data: <T as pallet_voting_oracle::Config>::ProposalExtraData,
	) -> Result<<T as pallet_voting_oracle::Config>::Proposal, ()> {
		match (self, extra_data) {
			(BaseTaskProposal::Transfer(transfer), OracleData::Transfer(timestamp)) => {
				let call = <T as Config>::Call::from(crate::Call::verify_transfer {
					transfer: crate::Transfer { timestamp, ..transfer.transfer },
					deadline: transfer.deadline,
				});
				let prop: <T as pallet_voting_oracle::Config>::Proposal = call.into();
				Ok(prop)
			},
		}
	}

	fn make_failure_proposal(
		self,
		reason: <T as pallet_voting_oracle::Config>::DisagreementReason,
	) -> Result<<T as pallet_voting_oracle::Config>::Proposal, ()> {
		match self {
			BaseTaskProposal::Transfer(transfer) => {
				let transfer_id = crate::TransferId::new::<T>(
					&transfer.transfer.blockchain,
					&transfer.transfer.tx_id,
				);
				let call = <T as Config>::Call::from(crate::Call::fail_transfer {
					transfer_id,
					cause: reason,
					deadline: transfer.deadline,
				});
				let prop: <T as pallet_voting_oracle::Config>::Proposal = call.into();
				Ok(prop)
			},
		}
	}
}
