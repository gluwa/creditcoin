use core::str::Utf8Error;

use super::rpc::errors::RpcError;
use alloc::string::FromUtf8Error;
use pallet_offchain_task_scheduler::impl_enum_from_variant;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::offchain::storage::StorageRetrievalError;

#[derive(Debug)]
pub enum OffchainError {
	InvalidTask(VerificationFailureCause),
	NoRpcUrl(RpcUrlError),
	RpcError(RpcError),
	IncorrectChainId,
}

#[derive(Debug)]
pub enum SchedulerError {
	NoRpcUrl(RpcUrlError),
	RpcError(RpcError),
	IncorrectChainId,
}

pub type VerificationResult<T> = Result<T, OffchainError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum VerificationFailureCause {
	TaskNonexistent,
	TaskFailed,
	TaskPending,
	TaskUnconfirmed,
	TaskInFuture,
	IncorrectContract,
	MissingReceiver,
	MissingSender,
	AbiMismatch,
	IncorrectInputLength,
	EmptyInput,
	IncorrectInputType,
	IncorrectAmount,
	IncorrectNonce,
	IncorrectReceiver,
	IncorrectSender,
	InvalidAddress,
	UnsupportedMethod,
	TransactionNotFound,
}

impl VerificationFailureCause {
	pub fn is_fatal(self) -> bool {
		use VerificationFailureCause::*;
		match self {
			TaskFailed | IncorrectContract | MissingSender | MissingReceiver | AbiMismatch
			| IncorrectInputLength | IncorrectInputType | IncorrectAmount | IncorrectNonce
			| InvalidAddress | UnsupportedMethod | TaskInFuture | IncorrectSender | EmptyInput
			| IncorrectReceiver | TaskNonexistent | TransactionNotFound => true,
			TaskPending | TaskUnconfirmed => false,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum RpcUrlError {
	StorageFailure(StorageRetrievalError),
	InvalidUrl(FromUtf8Error),
	InvalidChain(Utf8Error),
	NoValue,
}

impl_enum_from_variant!(
	OffchainError,
	RpcUrlError => NoRpcUrl,
	RpcError => RpcError,
	VerificationFailureCause => InvalidTask,
);
impl_enum_from_variant!(
	RpcUrlError,
	StorageRetrievalError => StorageFailure,
	FromUtf8Error => InvalidUrl,
);

impl_enum_from_variant!(
	SchedulerError,
	RpcUrlError => NoRpcUrl,
	RpcError => RpcError,
);
