use self::errors::RpcError;

use super::OffchainResult;
use alloc::string::String;
use core::{convert::TryFrom, fmt};
use ethereum_types::{H160, H256, U256, U64};
use serde::{
	de::{Error, Unexpected, Visitor},
	Deserialize, Deserializer, Serialize, Serializer,
};
use sp_runtime::offchain::{http, Duration, Timestamp};
use sp_std::{prelude::*, vec::Vec};

use crate::ExternalTxId;

pub mod errors {
	use super::JsonRpcError;
	use crate::ocw::errors::impl_from_error;
	use sp_runtime::offchain::{http::PendingRequest, HttpError};

	#[derive(Debug)]
	pub enum RpcError {
		NoResult,
		FailureResponse(JsonRpcError),
		SerdeError(serde_json::Error),
		HttpError(HttpError),
		RequestError(sp_runtime::offchain::http::Error),
		InvalidArgument(&'static str),
		Timeout(PendingRequest),
	}

	impl_from_error!(
		RpcError,
		JsonRpcError => FailureResponse,
		serde_json::Error => SerdeError,
		HttpError => HttpError,
		sp_runtime::offchain::http::Error => RequestError,
		PendingRequest => Timeout
	);
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct VecString(Vec<u8>, ());

impl TryFrom<&[u8]> for VecString {
	type Error = core::str::Utf8Error;
	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let _ = core::str::from_utf8(value)?;
		Ok(VecString(Vec::from(value), ()))
	}
}

impl From<&str> for VecString {
	fn from(s: &str) -> Self {
		VecString(Vec::from(s.as_bytes()), ())
	}
}

impl serde::Serialize for VecString {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serde::Serialize::serialize(
			core::str::from_utf8(&self.0)
				.expect("vecstrings cannot be constructed without validating utf8; qed"),
			serializer,
		)
	}
}

impl<'de> serde::Deserialize<'de> for VecString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s: &str = serde::Deserialize::deserialize(deserializer)?;
		Ok(VecString(Vec::from(s.as_bytes()), ()))
	}
}

/// Raw bytes wrapper
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Bytes(pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Bytes {
	fn from(data: T) -> Self {
		Bytes(data.into())
	}
}

impl Serialize for Bytes {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut serialized = String::from("0x");
		serialized.push_str(&hex::encode(&self.0));
		serializer.serialize_str(serialized.as_ref())
	}
}

impl<'a> Deserialize<'a> for Bytes {
	fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
	where
		D: Deserializer<'a>,
	{
		deserializer.deserialize_identifier(BytesVisitor)
	}
}

pub type Address = H160;

struct BytesVisitor;

impl<'a> Visitor<'a> for BytesVisitor {
	type Value = Bytes;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: Error,
	{
		if value.len() >= 2 && &value[0..2] == "0x" {
			let bytes = hex::decode(&value[2..])
				.map_err(|e| Error::custom(alloc::format!("Invalid hex: {}", e)))?;
			Ok(Bytes(bytes))
		} else {
			Err(Error::invalid_value(Unexpected::Str(value), &"0x prefix"))
		}
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
	where
		E: Error,
	{
		self.visit_str(value.as_ref())
	}
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct JsonRpcRequest {
	jsonrpc: VecString,
	method: VecString,
	params: Vec<serde_json::Value>,
	id: u64,
}

const REQUEST_TIMEOUT_PERIOD: Duration = Duration::from_millis(5000);

fn timeout() -> Timestamp {
	sp_io::offchain::timestamp().add(REQUEST_TIMEOUT_PERIOD)
}

impl JsonRpcRequest {
	#[allow(dead_code)]
	pub fn with_method(method: impl Into<VecString>) -> Self {
		let method = method.into();
		Self { jsonrpc: VecString::from("2.0"), method, params: Vec::new(), id: 1 }
	}
	pub fn new(
		method: impl Into<VecString>,
		params: impl IntoIterator<Item = serde_json::Value>,
	) -> Self {
		Self {
			jsonrpc: VecString::from("2.0"),
			method: method.into(),
			params: params.into_iter().collect(),
			id: 1,
		}
	}

	#[allow(dead_code)]
	pub fn param(&mut self, param: serde_json::Value) -> &mut Self {
		self.params.push(param);
		self
	}

	#[allow(dead_code)]
	pub fn to_bytes(&self) -> Vec<u8> {
		serde_json::to_vec(self).expect("serialization cannot fail; qed")
	}

	pub fn send<T: for<'de> serde::Deserialize<'de>>(
		self,
		rpc_url: &str,
	) -> OffchainResult<T, RpcError> {
		let rpc_bytes = serde_json::to_vec(&self).map_err(RpcError::SerdeError)?;
		let timeout = timeout();
		let response = http::Request::post(rpc_url, vec![rpc_bytes])
			.add_header("Content-Type", "application/json")
			.send()?
			.try_wait(timeout)??;
		let body: Vec<u8> = response.body().collect();
		let rpc_response: JsonRpcResponse<T> = serde_json::from_slice(&body)?;
		rpc_response.result()
	}
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct JsonRpcResponse<T> {
	#[allow(dead_code)]
	jsonrpc: VecString,
	#[allow(dead_code)]
	id: u64,
	error: Option<JsonRpcError>,
	result: Option<T>,
}

impl<T> JsonRpcResponse<T> {
	pub fn result(self) -> Result<T, RpcError> {
		if let Some(err) = self.error {
			return Err(err.into());
		}
		if let Some(result) = self.result {
			Ok(result)
		} else {
			Err(RpcError::NoResult)
		}
	}
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Clone, Debug)]
pub struct JsonRpcError {
	code: i32,
	message: String,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct EthTransaction {
	/// Hash
	pub hash: H256,
	/// Block number. None when pending.
	#[serde(rename = "blockNumber")]
	pub block_number: Option<U64>,
	/// Sender
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub from: Option<Address>,
	/// Recipient (None when contract creation)
	pub to: Option<Address>,
	/// Transfered value
	pub value: U256,
	/// Input data
	pub input: Bytes,
}

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct EthTransactionReceipt {
	/// Transaction hash.
	#[serde(rename = "transactionHash")]
	pub transaction_hash: H256,
	/// Number of the block this transaction was included within.
	#[serde(rename = "blockNumber")]
	pub block_number: Option<U64>,
	/// Sender
	/// Note: default address if the client did not return this value
	/// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
	#[serde(default)]
	pub from: Address,
	/// Recipient (None when contract creation)
	/// Note: Also `None` if the client did not return this value
	/// (maintains backwards compatibility for <= 0.7.0 when this field was missing)
	#[serde(default)]
	pub to: Option<Address>,
	/// Status: either 1 (success) or 0 (failure).
	pub status: Option<U64>,
}

impl EthTransactionReceipt {
	pub fn is_success(&self) -> bool {
		if let Some(status) = self.status {
			!status.is_zero()
		} else {
			false
		}
	}
}

pub fn eth_get_transaction(
	tx_id: &ExternalTxId,
	rpc_url: &str,
) -> OffchainResult<EthTransaction, RpcError> {
	let rpc_req = JsonRpcRequest::new(
		"eth_getTransactionByHash",
		vec![serde_json::Value::String(
			alloc::string::String::from_utf8(tx_id.clone().into()).map_err(|err| {
				log::error!("failed to get eth transaction, tx id is invalid utf8: {}", err);
				RpcError::InvalidArgument("transaction id is invalid utf8")
			})?,
		)],
	);
	rpc_req.send(rpc_url)
}

pub fn eth_get_transaction_receipt(
	tx_id: &ExternalTxId,
	rpc_url: &str,
) -> OffchainResult<EthTransactionReceipt, RpcError> {
	let rpc_req = JsonRpcRequest::new(
		"eth_getTransactionReceipt",
		vec![serde_json::Value::String(String::from_utf8(tx_id.clone().into()).map_err(
			|err| {
				log::error!(
					"failed to get eth transaction receipt, tx id is invalid utf8: {}",
					err
				);
				RpcError::InvalidArgument("transaction id is invalid utf8")
			},
		)?)],
	);
	rpc_req.send(rpc_url)
}

pub fn eth_get_block_number(rpc_url: &str) -> OffchainResult<U64, RpcError> {
	let rpc_req = JsonRpcRequest::new("eth_blockNumber", vec![]);
	rpc_req.send(rpc_url)
}
