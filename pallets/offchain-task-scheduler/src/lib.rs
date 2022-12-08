#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use frame_support::traits::Get;
use frame_system::offchain::AppCrypto;
use frame_system::pallet_prelude::BlockNumberFor;
use frame_system::Config as SystemConfig;
pub use ocw::nonce::nonce_key;
pub use pallet::{Authorities, Config, Error, Event, Pallet, WeightInfo};
pub use pallet::{__substrate_call_check, __substrate_event_check, tt_error_token};
use sp_core::offchain::KeyTypeId;
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::traits::Saturating;
use tracing as log;

pub mod authority;
pub mod benchmarking;
pub mod mock;
pub mod ocw;
pub mod tasks;
#[allow(clippy::unnecessary_cast)]
pub mod weights;

//gluwa's offchain task scheduler
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"gots");

pub mod crypto {
	use super::AppCrypto;
	use crate::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct AuthorityId;

	impl AppCrypto<MultiSigner, MultiSignature> for AuthorityId {
		type RuntimeAppPublic = Public;
		type GenericPublic = sp_core::sr25519::Public;
		type GenericSignature = sp_core::sr25519::Signature;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::{
		log,
		tasks::{self, ForwardTask},
		AppCrypto, Saturating, SystemConfig,
	};
	use core::fmt::Debug;
	use frame_support::dispatch::Dispatchable;
	use frame_support::dispatch::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::offchain::CreateSignedTransaction;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::codec::FullCodec;

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
		type AuthorityId: AppCrypto<
			Self::Public,
			<Self as frame_system::offchain::SigningTypes>::Signature,
		>;
		type AccountIdFrom: From<sp_core::sr25519::Public>
			+ IsType<Self::AccountId>
			+ Clone
			+ core::fmt::Debug
			+ PartialEq
			+ AsRef<[u8; 32]>;

		type InternalPublic: sp_core::crypto::UncheckedFrom<[u8; 32]>;
		type PublicSigning: From<Self::InternalPublic> + Into<Self::Public>;
		type TaskCall: Dispatchable<RuntimeOrigin = Self::RuntimeOrigin> + Clone;
	}

	pub trait WeightInfo {
		fn on_initialize(p: u32) -> Weight;
	}

	#[pallet::event]
	pub enum Event<T: Config> {}

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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			log::debug!("Cleaning up expired entries");

			let mut unverified_task_count = 0u32;
			let mut cursor: Option<Vec<u8>> = None;
			loop {
				let result = PendingTasks::<T>::clear_prefix(
					block_number,
					u32::MAX,
					cursor.as_ref().map(|c| c.as_slice()),
				);
				unverified_task_count.saturating_accrue(result.backend);
				cursor = result.maybe_cursor;
				if cursor.is_none() {
					break;
				}
			}

			<T as Config>::WeightInfo::on_initialize(unverified_task_count)
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			let auth_id = match Self::authority_id() {
				Some(id) => id,
				None => {
					log::debug!(target: "task", "Not an authority, skipping offchain work");
					return;
				},
			};

			for (deadline, id, task) in PendingTasks::<T>::iter() {
				let storage_key = tasks::storage_key(&id);
				let mut lock = tasks::task_lock::<T>(&storage_key);

				let guard = match lock.try_lock() {
					Ok(g) => g,
					Err(_) => continue,
				};

				log::trace!(target: "task", "@{block_number:?} Task {:8?}", id);

				use tasks::error::TaskError::*;
				match task.forward_task(deadline) {
					Ok(call) => {
						match Self::submit_txn_with_synced_nonce(auth_id.clone(), |_| call.clone())
						{
							Ok(_) => guard.forget(),
							Err(e) => {
								log::error!("Failed to send a dispatchable transaction: {:?}", e)
							},
						}
					},
					Err(FinishedTask) => {
						log::debug!("Already handled Task ({:?}, {:?}) {task:?}", deadline, id);
						guard.forget();
						continue;
					},
					Err(Evaluation(cause)) => {
						log::warn!("Failed to verify pending task {:?} : {:?}", task, cause);
					},
					Err(Scheduler(error)) => {
						log::error!("Task verification encountered a processing error {:?}", error)
					},
				}
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
}

type TaskFor<T> = <T as Config>::Task;
type HashFor<T> = <T as SystemConfig>::Hash;

impl<Runtime: Config>
	tasks::TaskScheduler<BlockNumberFor<Runtime>, HashFor<Runtime>, TaskFor<Runtime>> for Runtime
{
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
}
