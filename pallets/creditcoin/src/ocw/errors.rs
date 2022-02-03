use core::str::Utf8Error;

use super::rpc::errors::RpcError;
use alloc::string::FromUtf8Error;
use sp_runtime::offchain::storage::StorageRetrievalError;

#[derive(Debug)]
pub enum OffchainError {
	InvalidTransfer(&'static str),
	NoRpcUrl(RpcUrlError),
	RpcError(RpcError),
}

#[derive(Debug)]
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
	RpcError => RpcError
);
impl_from_error!(
	RpcUrlError,
	StorageRetrievalError => StorageFailure,
	FromUtf8Error => InvalidUrl,
);
