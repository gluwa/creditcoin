#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use frame_support::traits::Get;
use frame_system::offchain::AppCrypto;
use frame_system::pallet_prelude::BlockNumberFor;
use frame_system::Config as SystemConfig;
pub use ocw::nonce::nonce_key;
#[cfg(feature = "std")]
pub use pallet::GenesisConfig;
pub use pallet::{Authorities, Call, Config, Error, Event, Pallet, WeightInfo};
pub use pallet::{
	__InherentHiddenInstance, __substrate_call_check, __substrate_event_check,
	__substrate_genesis_config_check, tt_default_parts, tt_error_token,
};
use sp_core::offchain::KeyTypeId;
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::traits::Saturating;
use tracing as log;

pub mod authority;
pub mod authorship;
pub mod benchmarking;
pub mod mock;
pub mod mocked_task;
pub mod ocw;
pub mod tasks;
pub mod tests;
#[allow(clippy::unnecessary_cast)]
pub mod weights;

//gluwa's offchain task scheduler
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"gots");

pub mod crypto {
	use super::AppCrypto;
	use crate::KEY_TYPE;
	use sp_core::crypto::Wraps;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	#[derive(Clone, PartialEq, Eq, core::fmt::Debug)]
	pub struct AuthorityId;

	impl AppCrypto<MultiSigner, MultiSignature> for AuthorityId {
		type RuntimeAppPublic = Public;
		type GenericPublic = <Public as Wraps>::Inner;
		type GenericSignature = sp_core::sr25519::Signature;
	}

	impl From<Public> for MultiSigner {
		fn from(public: Public) -> MultiSigner {
			sp_core::sr25519::Public::from(public).into()
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::{
		authorship::Authorship,
		log,
		ocw::caching::OutputCache,
		tasks::{self, ForwardTask},
		AppCrypto, Saturating, SystemConfig,
	};
	use crate::ocw::RuntimePublicOf;
	use crate::ocw::{disputing::Disputable, sampling::Sampling};
	use crate::tasks::TaskScheduler as TaskSchedulerT;
	use core::fmt::Debug;
	use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
	use frame_support::pallet_prelude::*;
	use frame_support::transactional;
	use frame_system::pallet_prelude::*;
	use frame_system::{offchain::CreateSignedTransaction, RawOrigin};
	use scale_info::TypeInfo;
	use sp_core::sr25519::Public;
	use sp_runtime::codec::FullCodec;
	use sp_std::boxed::Box;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_timestamp::Config + CreateSignedTransaction<Self::TaskCall>
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Task: TypeInfo
			+ FullCodec
			+ MaxEncodedLen
			+ ForwardTask<Self, Call = Self::TaskCall>
			+ Debug;
		type UnverifiedTaskTimeout: Get<<Self as SystemConfig>::BlockNumber>;
		type WeightInfo: WeightInfo;
		type TaskCall: Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
			+ From<Call<Self>>
			+ Clone
			+ Encode
			+ Decode
			+ Parameter
			+ GetDispatchInfo;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		type Authorship: Authorship<
			RuntimePublic = RuntimePublicOf<Self>,
			AccountId = Self::AccountId,
		>;
		type Sampling: Sampling<Id = Self::Hash, AccountId = Self::AccountId>;
		type OutputCache: OutputCache<Id = Self::Hash, Output = Self::TaskCall>;
		type Dispute: Disputable<ItemId = Self::Hash, Item = Self::TaskCall, Who = Self::AccountId>;
	}

	pub trait WeightInfo {
		fn on_initialize(p: u32) -> Weight;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A task is finished. [id, result]
		TaskCompleted { task_id: T::Hash, result: DispatchResult },
		/// A task expired. [id]
		TaskExpired { task_id: T::Hash },
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn pending_tasks)]
	pub type PendingTasks<T: Config> =
		StorageDoubleMap<_, Identity, T::BlockNumber, Identity, T::Hash, T::Task>;

	#[pallet::storage]
	#[pallet::getter(fn authorities)]
	pub type Authorities<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, ()>;

	#[derive(PartialEq, Eq)]
	#[pallet::error]
	pub enum Error<T> {
		/// Failed to send an offchain callback transaction. This is likely
		/// an internal error.
		OffchainSignedTxFailed,
		/// The node is an authority but there is no account to create a
		/// callback transaction. This is likely an internal error.
		NoLocalAcctForSignedTx,
		/// The caller does not have authority to submit or process tasks.
		UnauthorizedSubmission,
		///Could not finish proving sample.
		ProvingSamplingFailed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		<T::AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic:
			Into<T::Public> + AsRef<Public> + sp_std::fmt::Debug + Clone,
	{
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			log::debug!("Cleaning up expired entries");

			let mut unverified_task_count = 0u32;
			for (task_id, _) in PendingTasks::<T>::drain_prefix(block_number) {
				unverified_task_count.saturating_accrue(1);
				T::Dispute::clear(&task_id);
				Self::deposit_event(Event::TaskExpired { task_id });
			}

			<T as Config>::WeightInfo::on_initialize(unverified_task_count)
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			let signer = match Self::authority_pubkey() {
				Some(pubkey) => pubkey,
				None => {
					log::debug!(target: "runtime::task", "Not an authority, skipping offchain work");
					return;
				},
			};

			for (deadline, id, task) in PendingTasks::<T>::iter() {
				let storage_key = tasks::lock_key(&id);
				let mut lock = tasks::task_lock::<T>(&storage_key);

				let guard = match lock.try_lock() {
					Ok(g) => g,
					Err(_) => continue,
				};

				log::trace!(target: "runtime::task", "@{block_number:?} Task {:8?}", id);

				let mut cache_hit = false;
				let result = match T::OutputCache::get(&id) {
					Ok(None) => task.forward_task(),
					Ok(Some(o)) => {
						cache_hit = true;
						Ok(o)
					},
					Err(e) => {
						log::info!(target: "runtime::task", "@{block_number:?} Task {id:8?} Cache get op errored {e:?}, recomputing the task.");
						task.forward_task()
					},
				};

				use tasks::error::TaskError::*;
				match result {
					Ok(call) => {
						if !cache_hit {
							T::OutputCache::set(&id, &call);
						}

						if let Some(sampled) = T::Sampling::sample(&id, &signer) {
							match sampled {
								Ok(_proof) if cache_hit => guard.forget(),
								Ok(proof) => Self::try_submit(
									block_number,
									deadline,
									id,
									signer.clone(),
									Box::new(call),
									guard,
									proof,
								),
								Err(proof) if T::Dispute::disputable(&id) => {
									if T::Dispute::disagree(&id, &call) {
										Self::try_submit(
											block_number,
											deadline,
											id,
											signer.clone(),
											Box::new(call),
											guard,
											proof,
										)
									} else {
										guard.forget();
									}
								},
								Err(_) if !cache_hit => guard.forget(),
								Err(_) => drop(guard),
							}

							// Clear the cache otherwise.
							continue;
						} else {
							log::debug!( target: "runtime::task", "@{block_number:?} Failed to sample {signer:?}: {deadline:?}, {id:?} {task:?}");
							guard.forget();
						}
					},
					Err(FinishedTask) => {
						log::debug!( target: "runtime::task", "@{block_number:?} Already handled Task ({deadline:?}, {id:?}) {task:?}");
						guard.forget();
					},
					Err(Evaluation(cause)) => {
						log::warn!(target: "runtime::task", "@{block_number:?} Failed to verify pending task {task:?} : {cause:?}",);
					},
					Err(Scheduler(error)) => {
						log::error!( target: "runtime::task", "@{block_number:?} Task verification encountered a processing error {error:?}",);
					},
				}

				T::OutputCache::clear(&id);
			}
		}
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub authorities: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<Runtime: Config> Default for GenesisConfig<Runtime> {
		fn default() -> Self {
			Self { authorities: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for authority in &self.authorities {
				Authorities::<T>::insert(authority.clone(), ());
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[transactional]
		#[pallet::weight({ let dispatch_info = call.get_dispatch_info(); (dispatch_info.weight, dispatch_info.class) })]
		pub fn submit_output(
			origin: OriginFor<T>,
			deadline: T::BlockNumber,
			task_id: T::Hash,
			call: Box<T::TaskCall>,
			proof: <T::Sampling as Sampling>::Proof,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(T::Authorship::is_authorized(&who), Error::<T>::UnauthorizedSubmission);

			T::Sampling::prove_sampled(&task_id, &who, proof)
				.ok_or(Error::<T>::ProvingSamplingFailed)?;
			//check if submitter or disputer
			//idempotency checks for voting, add an index.
			if let Some(output) = T::Dispute::vote_on(&who, &task_id, &call)? {
				Self::deposit_event(Event::TaskCompleted {
					task_id,
					result: output
						.dispatch(RawOrigin::Root.into())
						.map(|_| ())
						.map_err(|e| e.error),
				});
				Self::remove(&deadline, &task_id);
			};

			//FIXIT, accurate weights.
			Ok(PostDispatchInfo { actual_weight: None, pays_fee: Pays::No })
		}
	}
}

type TaskFor<T> = <T as Config>::Task;
type HashFor<T> = <T as SystemConfig>::Hash;

impl<Runtime: Config> tasks::TaskScheduler for Pallet<Runtime> {
	type BlockNumber = BlockNumberFor<Runtime>;
	type Hash = HashFor<Runtime>;
	type Task = TaskFor<Runtime>;

	fn deadline() -> BlockNumberFor<Runtime> {
		let offset = Runtime::UnverifiedTaskTimeout::get();
		let block = frame_system::Pallet::<Runtime>::current_block_number();
		offset.saturating_add(block)
	}
	fn is_scheduled(deadline: &BlockNumberFor<Runtime>, id: &HashFor<Runtime>) -> bool {
		crate::pallet::PendingTasks::<Runtime>::contains_key(deadline, id)
	}
	fn insert(deadline: &BlockNumberFor<Runtime>, id: &HashFor<Runtime>, task: TaskFor<Runtime>) {
		crate::pallet::PendingTasks::<Runtime>::insert(deadline, id, task);
	}
	fn remove(deadline: &BlockNumberFor<Runtime>, id: &HashFor<Runtime>) {
		crate::pallet::PendingTasks::<Runtime>::remove(deadline, id);
	}
}
