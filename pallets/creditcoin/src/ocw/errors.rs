use core::str::Utf8Error;

use super::rpc::errors::RpcError;
use alloc::string::FromUtf8Error;
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

macro_rules! _impl_from_error {
	($self_ty: ident, $err_ty: path, $variant: ident) => {
		impl From<$err_ty> for $self_ty {
			fn from(err: $err_ty) -> Self {
				$self_ty::$variant(err)
			}
		}
	};
	($self_ty: ident, $($err_ty: path => $variant: ident),+ $(,)?) => {
        $(
            impl_from_error!($self_ty, $err_ty, $variant);
        )+
    };
}

pub(crate) use _impl_from_error as impl_from_error;

impl_from_error!(
	OffchainError,
	RpcUrlError => NoRpcUrl,
	RpcError => RpcError,
	VerificationFailureCause => InvalidTask,
);
impl_from_error!(
	RpcUrlError,
	StorageRetrievalError => StorageFailure,
	FromUtf8Error => InvalidUrl,
);

use pallet_offchain_task_scheduler::impl_enum_from_variant;

impl_enum_from_variant!(
	SchedulerError,
	RpcUrlError => NoRpcUrl,
	RpcError => RpcError,
);
