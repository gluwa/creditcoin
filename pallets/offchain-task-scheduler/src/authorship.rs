use super::{ocw::RuntimePlubicOf, pallet::Authorities, Config, Pallet};
use crate::authority::AuthorityController;
use sp_runtime::traits::IdentifyAccount;

pub trait Authorship {
	type RuntimePublic: Clone;
	type Public: From<Self::RuntimePublic> + IdentifyAccount<AccountId = Self::AccountId>;
	type AccountId;
	fn find_authorized<'a>(
		keys: impl Iterator<Item = &'a Self::RuntimePublic>,
	) -> Option<Self::RuntimePublic>
	where
		Self::RuntimePublic: 'a,
	{
		keys.cloned().find(|pkey| {
			let auth = Self::Public::from(pkey.clone()).into_account();
			Self::is_authorized(&auth)
		})
	}

	fn is_authorized(who: &Self::AccountId) -> bool;
}

impl<T: Config> Authorship for Pallet<T>
where
	RuntimePlubicOf<T>: Clone,
	T::Public: From<RuntimePlubicOf<T>>,
{
	type RuntimePublic = RuntimePlubicOf<T>;
	type AccountId = T::AccountId;
	type Public = T::Public;

	fn is_authorized(who: &Self::AccountId) -> bool {
		T::is_authority(who)
	}
}
