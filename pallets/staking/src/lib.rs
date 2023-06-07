#![cfg_attr(not(feature = "std"), no_std)]

use frame_election_provider_support::{
	BoundedSupportsOf, ElectionDataProvider, ElectionProvider, ElectionProviderBase,
	SortedListProvider, Support,
};
use frame_support::traits::{ConstU32, DefensiveTruncateFrom};
use frame_support::{defensive, BoundedVec};
use frame_support::{traits::Defensive, RuntimeDebug};
use frame_system::offchain::SigningTypes;
use pallet_offchain_task_scheduler::ocw::RuntimePublicOf;
use pallet_offchain_task_scheduler::{authorship::Authorship, Config as TaskSchedulerConfig};
pub use pallet_staking_substrate as pallet;
pub use pallet_staking_substrate::weights;
#[cfg(feature = "std")]
pub use pallet_staking_substrate::GenesisConfig;
#[cfg(feature = "std")]
pub use pallet_staking_substrate::StakerStatus;
#[cfg(feature = "std")]
pub use pallet_staking_substrate::TestBenchmarkingConfig;
use pallet_staking_substrate::ValidatorPrefs;
pub use pallet_staking_substrate::{
	ActiveEra, ActiveEraInfo, Config, ErasStartSessionIndex, Error, Event, ForceEra, Forcing,
	Pallet, RewardDestination, UseValidatorsMap,
};
use parity_scale_codec::{Decode, Encode, EncodeLike};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{OpaqueKeys, Zero};
use sp_runtime::AccountId32;
pub use sp_staking::{EraIndex, StakingInterface};
use sp_std::{boxed::Box, fmt::Debug, marker::PhantomData, vec};

pub(crate) const LOG_TARGET: &str = "runtime::staking";

macro_rules! logger {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}
pub(crate) use logger;

pub struct EmptyList<T>(PhantomData<T>);
impl<T: Config> SortedListProvider<T::AccountId> for EmptyList<T> {
	type Error = ();
	type Score = u64;

	fn iter() -> Box<dyn Iterator<Item = T::AccountId>> {
		defensive!();
		Box::new(vec![].into_iter())
	}

	fn iter_from(
		_start: &T::AccountId,
	) -> Result<Box<dyn Iterator<Item = T::AccountId>>, Self::Error> {
		defensive!();
		Ok(Self::iter())
	}

	fn count() -> u32 {
		logger!(debug, "Faking EmptyList count");
		pallet_staking_substrate::Validators::<T>::count()
	}

	fn contains(_id: &T::AccountId) -> bool {
		false
	}

	fn on_insert(_id: T::AccountId, _score: Self::Score) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn on_update(_id: &T::AccountId, _score: Self::Score) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn get_score(_id: &T::AccountId) -> Result<Self::Score, Self::Error> {
		defensive!();
		Ok(Zero::zero())
	}

	fn on_remove(_id: &T::AccountId) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn unsafe_regenerate(
		_all: impl IntoIterator<Item = T::AccountId>,
		_score_of: Box<dyn Fn(&T::AccountId) -> Self::Score>,
	) -> u32 {
		defensive!();
		0
	}

	fn unsafe_clear() {
		defensive!();
	}

	#[cfg(feature = "try-runtime")]
	fn try_state() -> Result<(), &'static str> {
		defensive!();
		Ok(())
	}

	/// If `who` changes by the returned amount they are guaranteed to have a worst case change
	/// in their list position.
	#[cfg(feature = "runtime-benchmarks")]
	fn score_update_worst_case(_who: &T::AccountId, _is_increase: bool) -> Self::Score {
		unreachable!()
	}
}

pub struct TrivialSessionHandler<T>(PhantomData<T>);

impl<T: Config> pallet_session::SessionHandler<T::AccountId> for TrivialSessionHandler<T> {
	const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[];

	fn on_genesis_session<Ks: sp_runtime::traits::OpaqueKeys>(validators: &[(T::AccountId, Ks)]) {
		for (id, _) in validators {
			pallet_staking_substrate::Validators::<T>::insert(id, ValidatorPrefs::default());
		}
	}

	fn on_new_session<Ks: sp_runtime::traits::OpaqueKeys>(
		_changed: bool,
		_validators: &[(T::AccountId, Ks)],
		_queued_validators: &[(T::AccountId, Ks)],
	) {
	}

	fn on_disabled(_validator_index: u32) {}
}

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, RuntimeDebug, Deserialize, Serialize)]
pub struct NoKeys;

impl OpaqueKeys for NoKeys {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [sp_runtime::KeyTypeId] {
		static EMPTY: &[sp_runtime::KeyTypeId] = &[];
		EMPTY
	}

	fn get_raw(&self, _i: sp_runtime::KeyTypeId) -> &[u8] {
		static EMPTY: &[u8] = &[];
		EMPTY
	}
}

pub struct TrivialTargetList<T: Config>(PhantomData<T>);
impl<T: Config> SortedListProvider<AccountId32> for TrivialTargetList<T>
where
	AccountId32: EncodeLike<T::AccountId>,
{
	type Error = ();
	type Score = u128;

	fn iter() -> Box<dyn Iterator<Item = AccountId32>> {
		let x = AccountId32::new([0; 32]);
		pallet_staking_substrate::Validators::<T>::insert(x, ValidatorPrefs::default());
		Box::new(vec![AccountId32::new([0; 32])].into_iter())
	}

	fn iter_from(
		_start: &AccountId32,
	) -> Result<Box<dyn Iterator<Item = AccountId32>>, Self::Error> {
		unreachable!()
	}

	fn count() -> u32 {
		logger!(debug, "Faking TargetList count");
		pallet_staking_substrate::Validators::<T>::count()
	}

	fn contains(_id: &AccountId32) -> bool {
		defensive!();
		false
	}

	fn on_insert(_id: AccountId32, _score: Self::Score) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn on_update(_id: &AccountId32, _score: Self::Score) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn get_score(_id: &AccountId32) -> Result<Self::Score, Self::Error> {
		defensive!();
		Ok(Zero::zero())
	}

	fn on_remove(_id: &AccountId32) -> Result<(), Self::Error> {
		defensive!();
		Ok(())
	}

	fn unsafe_regenerate(
		_all: impl IntoIterator<Item = AccountId32>,
		_score_of: Box<dyn Fn(&AccountId32) -> Self::Score>,
	) -> u32 {
		defensive!();
		0
	}

	fn unsafe_clear() {
		defensive!();
	}

	#[cfg(feature = "try-runtime")]
	fn try_state() -> Result<(), &'static str> {
		defensive!();
		Ok(())
	}

	/// If `who` changes by the returned amount they are guaranteed to have a worst case change
	/// in their list position.
	#[cfg(feature = "runtime-benchmarks")]
	fn score_update_worst_case(_who: &AccountId32, _is_increase: bool) -> Self::Score {
		unreachable!()
	}
}

pub struct DefaultElection<X>(sp_std::marker::PhantomData<X>);

impl<AccountId, BlockNumber, DataProvider> ElectionProviderBase
	for DefaultElection<(AccountId, BlockNumber, DataProvider)>
where
	DataProvider: ElectionDataProvider<AccountId = AccountId, BlockNumber = BlockNumber>,
{
	type AccountId = AccountId;
	type BlockNumber = BlockNumber;
	type Error = &'static str;
	type MaxWinners = ConstU32<1>;
	type DataProvider = DataProvider;
}

impl<AccountId, BlockNumber, DataProvider> ElectionProvider
	for DefaultElection<(AccountId, BlockNumber, DataProvider)>
where
	DataProvider: ElectionDataProvider<AccountId = AccountId, BlockNumber = BlockNumber>,
	AccountId: Clone + Debug,
{
	fn ongoing() -> bool {
		false
	}

	fn elect() -> Result<BoundedSupportsOf<Self>, Self::Error> {
		// Get electable targets
		DataProvider::electable_targets(Some(1))
			// Wrap in defensive
			.defensive_proof("Trivial 0 AccountId")
			// Map resulting vector of vectors
			.map(|accounts| {
				// Wrap resulting Vec into a BoundedVec via truncation
				BoundedVec::defensive_truncate_from(
					accounts
						// Turn given vector into an iterator
						.iter()
						// Map to a tuple
						.map(|acc| (acc.clone(), Support::default()))
						// Collect as a Vec
						.collect(),
				)
			})
	}
}

pub struct StakingAuthorship<T: Config + SigningTypes>(PhantomData<T>);

impl<T: Config + SigningTypes + TaskSchedulerConfig> Authorship for StakingAuthorship<T>
where
	RuntimePublicOf<T>: Clone,
	T::Public: From<RuntimePublicOf<T>>,
{
	type RuntimePublic = RuntimePublicOf<T>;
	type Public = T::Public;
	type AccountId = T::AccountId;

	fn is_authorized(who: &Self::AccountId) -> bool {
		let res = matches!(Pallet::<T>::active_stake(who), Ok(active_stake) if active_stake > Zero::zero());
		logger!(trace, "{:?} authorship: {}", who, res);
		res
	}
}
