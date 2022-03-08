pub mod friendly;

use std::{marker::PhantomData, sync::Arc};

use codec::{Decode, Encode};
use futures::prelude::*;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{
	manager::SubscriptionManager, typed::Subscriber, PubSubMetadata, SubscriptionId,
};
use sc_client_api::{BlockchainEvents, StorageKey};
use serde::{Deserialize, Serialize};
use sp_api::{BlockId, HashFor, ProvideRuntimeApi, StateBackend};
use sp_blockchain::HeaderBackend;
use sp_core::{crypto::Ss58Codec, Bytes, H256};
use sp_runtime::{traits::Block as BlockT, AccountId32};

#[rpc]
pub trait CreditcoinApi<BlockHash> {
	type Metadata: PubSubMetadata;

	#[rpc(name = "creditcoin_getEvents")]
	fn get_events(&self, at: Option<BlockHash>) -> Result<Vec<friendly::Event>>;

	#[pubsub(subscription = "events", subscribe, name = "creditcoin_eventsSubscribe")]
	fn events_subscribe(&self, _: Self::Metadata, _: Subscriber<Vec<friendly::Event>>);

	#[pubsub(subscription = "events", unsubscribe, name = "creditcoin_eventsUnsubscribe")]
	fn events_unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
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

	fn get_events(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Vec<friendly::Event>> {
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
		let module = sp_core::twox_128(b"System");
		let name = sp_core::twox_128(b"Events");
		let mut address_key = Vec::with_capacity(32);
		address_key.extend(module);
		address_key.extend(name);
		let events_bytes = self
			.backend
			.state_at(at)
			.map_err(|e| RpcError::invalid_params(format!("invalid blockhash: {}", e)))?
			.storage(&address_key)
			.map_err(|e| RpcError {
				code: ErrorCode::ServerError(Error::StorageError.into()),
				message: "Unable to retrieve address from storage".into(),
				data: None,
			})?
			.ok_or(RpcError::invalid_params("events not found"))?;

		let events =
			<Vec<frame_system::EventRecord<creditcoin_node_runtime::Event, H256>>>::decode(
				&mut &*events_bytes,
			)
			.map_err(|e| RpcError {
				code: ErrorCode::ServerError(Error::DecodeError.into()),
				message: format!("Unable to decode events: {}", e),
				data: None,
			})?;

		let events_out = events
			.into_iter()
			.filter_map(|record| friendly::Event::from_runtime(record.event))
			.collect();

		Ok(events_out)
	}

	fn events_subscribe(&self, _: Self::Metadata, subscriber: Subscriber<Vec<friendly::Event>>) {
		let at = BlockId::<Block>::hash(self.client.info().best_hash);
		let module = sp_core::twox_128(b"System");
		let name = sp_core::twox_128(b"Events");
		let mut address_key = Vec::with_capacity(32);
		address_key.extend(module);
		address_key.extend(name);
		let key = StorageKey(address_key);
		let stream = match self.client.storage_changes_notification_stream(Some(&[key]), None) {
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

		let mut stream = stream.map(move |(block, changes)| {
			Ok(changes
				.iter()
				.filter_map(|(_, _, data)| {
					data.map(|sc_client_api::StorageData(data)| {
						<Vec<frame_system::EventRecord<creditcoin_node_runtime::Event, H256>>>::decode(
						&mut data.as_slice(),
							).map_err(|e| RpcError {
								code: ErrorCode::ServerError(Error::DecodeError.into()),
								message: format!("Unable to decode events: {}", e),
								data: None,
							}).map(|records| records.into_iter().filter_map(|r| {log::info!("event {:?}", r); friendly::Event::from_runtime(r.event)}).collect())
					})
				})
				.next()
				.unwrap())
		});

		self.manager.add(subscriber, move |sink| {
			let mut stream = stream.boxed();
			stream
				.forward(sink.sink_map_err(|e| log::warn!("Error sending notifications: {}", e)))
				.map(|_| ())
		});
	}

	fn events_unsubscribe(&self, _: Option<Self::Metadata>, id: SubscriptionId) -> Result<bool> {
		Ok(self.manager.cancel(id))
	}
}
