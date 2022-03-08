use std::convert::TryFrom;

use frame_support::BoundedVec;
use serde::{Deserializer, Serialize, Serializer};
use sp_core::Bytes;

pub fn serialize<T, S>(value: &BoundedVec<u8, T>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	Bytes(value.clone().into_inner()).serialize(serializer)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: TryFrom<Vec<u8>>,
	<T as TryFrom<Vec<u8>>>::Error: std::fmt::Debug,
{
	struct BytesVisitor;
	impl<'de> serde::de::Visitor<'de> for BytesVisitor {
		type Value = Vec<u8>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			formatter.write_str("bytes")
		}

		fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E> {
			Ok(bytes.into())
		}
	}
	let bytes = deserializer.deserialize_bytes(BytesVisitor)?;
	Ok(T::try_from(bytes).unwrap())
}
