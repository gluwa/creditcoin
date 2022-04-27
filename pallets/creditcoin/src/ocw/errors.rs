use core::str::Utf8Error;

use super::rpc::errors::RpcError;
use alloc::string::FromUtf8Error;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::offchain::storage::StorageRetrievalError;

#[derive(Debug)]
pub enum OffchainError {
	InvalidTransfer(VerificationFailureCause),
	NoRpcUrl(RpcUrlError),
	RpcError(RpcError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult<Moment> {
	Success { timestamp: Option<Moment> },
	Failure(VerificationFailureCause),
}

impl<M> From<VerificationFailureCause> for VerificationResult<M> {
	fn from(failure: VerificationFailureCause) -> Self {
		VerificationResult::Failure(failure)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum VerificationFailureCause {
	TransferFailed,
	TransferPending,
	TransferUnconfirmed,
	TransferInFuture,
	IncorrectContract,
	MissingReceiver,
	AbiMismatch,
	IncorrectInputLength,
	IncorrectInputType,
	IncorrectAmount,
	IncorrectNonce,
	IncorrectReceiver,
	IncorrectSender,
	InvalidAddress,
	UnsupportedMethod,
}

impl VerificationFailureCause {
	pub fn is_fatal(self) -> bool {
		use VerificationFailureCause::*;
		match self {
			TransferFailed | IncorrectContract | MissingReceiver | AbiMismatch
			| IncorrectInputLength | IncorrectInputType | IncorrectAmount | IncorrectNonce
			| InvalidAddress | UnsupportedMethod | TransferInFuture | IncorrectSender
			| IncorrectReceiver => true,
			TransferPending | TransferUnconfirmed => false,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum RpcUrlError {
	StorageFailure(StorageRetrievalError),
	InvalidUrl(FromUtf8Error),
	InvalidChain(Utf8Error),
	NoValue,
}

macro_rules! _impl_from_error {
	($self_ty: ident, $err_ty: path, $variant: ident, $err: ident, $ret: expr) => {
		impl From<$err_ty> for $self_ty {
			fn from($err: $err_ty) -> Self {
				$ret
			}
		}
	};
	($self_ty: ident, $err_ty: path, $variant: ident) => {
		impl_from_error!($self_ty, $err_ty, $variant, err, $self_ty::$variant(err));
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
	VerificationFailureCause => InvalidTransfer,
);
impl_from_error!(
	RpcUrlError,
	StorageRetrievalError => StorageFailure,
	FromUtf8Error => InvalidUrl,
);
