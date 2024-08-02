use super::{ocw::RuntimePublicOf, Config, Pallet};
use crate::authority::AuthorityController;
use sp_runtime::traits::IdentifyAccount;

pub trait Authorship {
	type RuntimePublic: Clone;
	type Public: From<Self::RuntimePublic> + IdentifyAccount<AccountId = Self::AccountId>;
	type AccountId;
	fn find_authorized<'a>(
		mut keys: impl Iterator<Item = &'a Self::RuntimePublic>,
	) -> Option<Self::RuntimePublic>
	where
		Self::RuntimePublic: 'a,
	{
		keys.find(|&pkey| {
			let auth = Self::Public::from(pkey.clone()).into_account();
			Self::is_authorized(&auth)
		})
		.cloned()
	}

	fn is_authorized(who: &Self::AccountId) -> bool;
}

impl<T: Config> Authorship for Pallet<T>
where
	RuntimePublicOf<T>: Clone,
	T::Public: From<RuntimePublicOf<T>>,
{
	type RuntimePublic = RuntimePublicOf<T>;
	type AccountId = T::AccountId;
	type Public = T::Public;

	fn is_authorized(who: &Self::AccountId) -> bool {
		Self::is_authority(who)
	}
}
