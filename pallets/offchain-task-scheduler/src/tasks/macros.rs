#[macro_export]
macro_rules! impl_enum_from_variant {
	($self_ty: ident, $err_ty: path, $variant: ident) => {
		impl From<$err_ty> for $self_ty {
			fn from(err: $err_ty) -> Self {
				$self_ty::$variant(err)
			}
		}
	};
	($self_ty: ident, $($err_ty: path => $variant: ident),+ $(,)?) => {
        $(
            impl_enum_from_variant!($self_ty, $err_ty, $variant);
        )+
    };
}
