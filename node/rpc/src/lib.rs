pub mod friendly;
use codec::Decode;
use creditcoin_node_runtime::Event as RuntimeEvent;
use frame_system::EventRecord;
use futures::prelude::*;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{
	manager::SubscriptionManager, typed::Subscriber, PubSubMetadata, SubscriptionId,
};
use sc_client_api::{BlockchainEvents, StorageKey};
use sp_api::{BlockId, ProvideRuntimeApi, StateBackend};
use sp_blockchain::HeaderBackend;
use sp_core::H256;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};

pub(crate) type Hash = <creditcoin_node_runtime::Runtime as frame_system::Config>::Hash;
pub(crate) type AccountId = <creditcoin_node_runtime::Runtime as frame_system::Config>::AccountId;
pub(crate) type BlockNumber =
	<creditcoin_node_runtime::Runtime as frame_system::Config>::BlockNumber;
pub(crate) type Moment = <creditcoin_node_runtime::Runtime as pallet_timestamp::Config>::Moment;

#[rpc]
pub trait CreditcoinApi<BlockHash> {
	type Metadata: PubSubMetadata;
	#[rpc(name = "creditcoin_getEvents")]
	fn get_events(&self, at: Option<BlockHash>) -> RpcResult<Vec<friendly::Event>>;

	#[pubsub(subscription = "events", subscribe, name = "creditcoin_eventsSubscribe")]
	fn events_subscribe(&self, _: Self::Metadata, _: Subscriber<Vec<friendly::Event>>);

	#[pubsub(subscription = "events", unsubscribe, name = "creditcoin_eventsUnsubscribe")]
	fn events_unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> RpcResult<bool>;
}

pub struct CreditcoinRpc<C, P, B> {
	client: Arc<C>,
	backend: Arc<B>,
	manager: SubscriptionManager,
	_marker: PhantomData<P>,
}

impl<C, P, B> CreditcoinRpc<C, P, B> {
	pub fn new<E>(client: Arc<C>, backend: Arc<B>, executor: Arc<E>) -> Self
	where
		E: futures::task::Spawn + Send + Sync + 'static,
	{
		Self { client, backend, manager: SubscriptionManager::new(executor), _marker: PhantomData }
	}
}

pub enum Error {
	StorageError,
	DecodeError,
	SubscriptionError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::StorageError => 1,
			Error::DecodeError => 2,
			Error::SubscriptionError => 3,
		}
	}
}

fn events_storage_key() -> StorageKey {
	let module = sp_core::twox_128(b"System");
	let name = sp_core::twox_128(b"Events");
	let mut key = Vec::with_capacity(32);
	key.extend(module);
	key.extend(name);
	StorageKey(key)
}

impl<C, Block, B> CreditcoinApi<<Block as BlockT>::Hash> for CreditcoinRpc<C, Block, B>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block> + BlockchainEvents<Block>,
	B: 'static + sc_client_api::Backend<Block>,
{
	type Metadata = sc_rpc::Metadata;

	fn get_events(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Vec<friendly::Event>> {
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		let StorageKey(address_key) = events_storage_key();
		let events_bytes = self
			.backend
			.state_at(at)
			.map_err(|e| RpcError::invalid_params(format!("invalid blockhash: {}", e)))?
			.storage(&address_key)
			.map_err(|e| RpcError {
				code: ErrorCode::ServerError(Error::StorageError.into()),
				message: format!("Unable to retrieve address from storage: {}", e),
				data: None,
			})?
			.ok_or_else(|| RpcError::invalid_params("events not found"))?;

		let events =
			<Vec<EventRecord<RuntimeEvent, H256>>>::decode(&mut &*events_bytes).map_err(|e| {
				RpcError {
					code: ErrorCode::ServerError(Error::DecodeError.into()),
					message: format!("Unable to decode events: {}", e),
					data: None,
				}
			})?;

		let events_out = events
			.into_iter()
			.filter_map(|record| friendly::Event::from_runtime(record.event))
			.collect();

		Ok(events_out)
	}

	fn events_subscribe(&self, _: Self::Metadata, subscriber: Subscriber<Vec<friendly::Event>>) {
		let events_key = events_storage_key();
		let stream =
			match self.client.storage_changes_notification_stream(Some(&[events_key]), None) {
				Ok(stream) => stream,
				Err(err) => {
					let _ = subscriber.reject(RpcError {
						code: ErrorCode::ServerError(Error::SubscriptionError.into()),
						message: format!("Failed to subscribe to storage changes: {}", err),
						data: None,
					});
					return;
				},
			};
		let stream =
			stream.map(move |(_block, changes)| -> Result<RpcResult<Vec<friendly::Event>>, ()> {
				let mut events = Vec::new();
				for (_, _, data) in changes.iter() {
					if let Some(sc_client_api::StorageData(data)) = data {
						match <Vec<EventRecord<RuntimeEvent, H256>>>::decode(&mut data.as_slice()) {
							Ok(records) => {
								events.extend(
									records
										.into_iter()
										.filter_map(|r| friendly::Event::from_runtime(r.event)),
								);
							},
							Err(e) => {
								return Ok(Err(RpcError {
									code: ErrorCode::ServerError(Error::DecodeError.into()),
									message: format!("Unable to decode events: {}", e),
									data: None,
								}));
							},
						}
					}
				}
				Ok(Ok(events))
			});
		self.manager.add(subscriber, move |sink| {
			stream
				.forward(sink.sink_map_err(|e| log::warn!("Error sending notifications: {}", e)))
				.map(|_| ())
		});
	}

	fn events_unsubscribe(&self, _: Option<Self::Metadata>, id: SubscriptionId) -> RpcResult<bool> {
		Ok(self.manager.cancel(id))
	}
}
