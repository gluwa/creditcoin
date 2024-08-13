use crate::types::{AddressId, Blockchain, ExternalTxId, SystemConfig};
use frame_support::RuntimeDebug;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedCoins<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	pub tx_id: ExternalTxId,
	pub contract_type: ContractType,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedCoinsId<Hash>(Hash);

impl<H> CollectedCoinsId<H> {
	pub fn inner_hash<Hasher>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> H
	where
		Hasher: Hash<Output = H>,
	{
		let key = concatenate!(blockchain.as_bytes(), blockchain_tx_id);
		<Hasher as Hash>::hash(&key)
	}

	pub fn new<C: SystemConfig>(
		contract_chain: &Blockchain,
		blockchain_tx_id: &[u8],
	) -> CollectedCoinsId<H>
	where
		<C as SystemConfig>::Hashing: Hash<Output = H>,
	{
		let hash = Self::inner_hash::<C::Hashing>(contract_chain, blockchain_tx_id);
		CollectedCoinsId(hash)
	}

	pub fn into_inner(self) -> H {
		self.0
	}
}

impl<H> From<H> for CollectedCoinsId<H> {
	fn from(hash: H) -> Self {
		Self(hash)
	}
}
use crate::types::concatenate;
use sp_runtime::traits::Hash;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[allow(clippy::upper_case_acronyms)]
pub enum ContractType {
	GCRE,
	GATE,
}
