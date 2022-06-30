pub mod loan_terms;
pub mod platform;

pub use loan_terms::*;
pub use platform::*;

use codec::{Decode, Encode, EncodeLike, FullCodec, MaxEncodedLen};
use extend::ext;
use frame_support::{
	storage::types::QueryKindTrait,
	traits::{ConstU32, Get, StorageInstance},
	BoundedVec, RuntimeDebug, StorageHasher,
};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
use sha2::Digest;
use sp_core::ecdsa;
use sp_runtime::traits::Hash;
use sp_std::prelude::*;

pub type ExternalAmount = sp_core::U256;
type GuidLen = ConstU32<256>;
pub type Guid = BoundedVec<u8, GuidLen>;
type ExternalAddressLen = ConstU32<256>;
pub type ExternalAddress = BoundedVec<u8, ExternalAddressLen>;
type ExternalTxIdLen = ConstU32<256>;
pub type ExternalTxId = BoundedVec<u8, ExternalTxIdLen>;
type OtherChainLen = ConstU32<256>;
pub type OtherChain = BoundedVec<u8, OtherChainLen>;
type OtherTransferKindLen = ConstU32<256>;
pub type OtherTransferKind = BoundedVec<u8, OtherTransferKindLen>;

#[cfg(feature = "std")]
mod bounded_serde;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Blockchain {
	Ethereum,
	Rinkeby,
	Luniverse,
	Bitcoin,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	Other(OtherChain),
}

impl Blockchain {
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			Blockchain::Ethereum => b"ethereum",
			Blockchain::Rinkeby => b"rinkeby",
			Blockchain::Luniverse => b"luniverse",
			Blockchain::Bitcoin => b"bitcoin",
			Blockchain::Other(chain) => chain.as_slice(),
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum TransferKind {
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	Erc20(ExternalAddress),
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	Ethless(ExternalAddress),
	Native,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	Other(OtherTransferKind),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Address<AccountId> {
	pub blockchain: Blockchain,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub value: ExternalAddress,
	pub owner: AccountId,
}

impl<AccountId> Address<AccountId> {
	pub fn matches_chain_of(&self, other: &Address<AccountId>) -> bool {
		self.blockchain == other.blockchain
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct CollectedCoins<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub tx_id: ExternalTxId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Transfer<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub kind: TransferKind,
	pub from: AddressId<Hash>,
	pub to: AddressId<Hash>,
	pub order_id: OrderId<BlockNum, Hash>,
	pub amount: ExternalAmount,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub tx_id: ExternalTxId,
	pub block: BlockNum,
	pub is_processed: bool,
	pub account_id: AccountId,
	pub timestamp: Option<Moment>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct UnverifiedCollectedCoins {
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub to: ExternalAddress,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub tx_id: ExternalTxId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> {
	pub transfer: Transfer<AccountId, BlockNum, Hash, Moment>,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub from_external: ExternalAddress,
	#[cfg_attr(feature = "std", serde(with = "bounded_serde"))]
	pub to_external: ExternalAddress,
	pub deadline: BlockNum,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Offer<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub ask_id: AskOrderId<BlockNum, Hash>,
	pub bid_id: BidOrderId<BlockNum, Hash>,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AskOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub lender_address_id: AddressId<Hash>,
	pub terms: AskTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct BidOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: BidTerms,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub borrower: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct DealOrder<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub offer_id: OfferId<BlockNum, Hash>,
	pub lender_address_id: AddressId<Hash>,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: LoanTerms,
	pub expiration_block: BlockNum,
	pub timestamp: Moment,
	pub block: Option<BlockNum>,
	pub funding_transfer_id: Option<TransferId<Hash>>,
	pub repayment_transfer_id: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub borrower: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AddressId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AskOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct BidOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct DealOrderId<BlockNum, Hash>(BlockNum, Hash);

#[cfg(test)]
impl<B: Default, H: Default> DealOrderId<B, H> {
	pub fn dummy() -> Self {
		Self(B::default(), H::default())
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct RepaymentOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum OrderId<BlockNum, Hash> {
	Deal(DealOrderId<BlockNum, Hash>),
	Repayment(RepaymentOrderId<BlockNum, Hash>),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct OfferId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct TransferId<Hash>(Hash);

#[cfg(test)]
impl<Hash> TransferId<Hash> {
	pub fn make(hash: Hash) -> Self {
		Self(hash)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct CollectedCoinsId<Hash>(Hash);

fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
	const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
	let mut hex = Vec::with_capacity(bytes.len() * 2);
	for byte in bytes {
		hex.push(HEX_CHARS_LOWER[(byte >> 4) as usize]);
		hex.push(HEX_CHARS_LOWER[(byte & 0x0F) as usize]);
	}
	hex
}

macro_rules! concatenate {
	(@strip_plus + $($rest: tt)*) => {
		$($rest)*
	};
	($($bytes: expr),+) => {
		{
			let mut buf = Vec::with_capacity($crate::types::concatenate!(@strip_plus $(+ $bytes.len())+));
			$(buf.extend($bytes);)+
			buf
		}
	};
}
pub(crate) use concatenate;

impl<B, H> OrderId<B, H>
where
	H: AsRef<[u8]>,
{
	pub fn to_hex(&self) -> Vec<u8> {
		let bytes = match self {
			OrderId::Deal(deal) => deal.1.as_ref(),
			OrderId::Repayment(repay) => repay.1.as_ref(),
		};
		bytes_to_hex(bytes)
	}
}

impl<H> AddressId<H> {
	pub fn new<Config>(blockchain: &Blockchain, address: &[u8]) -> AddressId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(blockchain.as_bytes(), address);
		AddressId(Config::Hashing::hash(&key))
	}
}

impl<B, H> AskOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> AskOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		AskOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<B, H> BidOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> BidOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		BidOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<B, H> RepaymentOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, guid: &[u8]) -> RepaymentOrderId<B, H>
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		RepaymentOrderId(expiration_block, Config::Hashing::hash(guid))
	}
}

impl<H> TransferId<H> {
	pub fn new<Config>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> TransferId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(blockchain.as_bytes(), blockchain_tx_id);
		TransferId(Config::Hashing::hash(&key))
	}
}

use crate::ocw::tasks::collect_coins::CONTRACT_CHAIN;
impl<H> CollectedCoinsId<H> {
	pub fn new<Config>(blockchain_tx_id: &[u8]) -> CollectedCoinsId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(CONTRACT_CHAIN.as_bytes(), blockchain_tx_id);
		CollectedCoinsId(Config::Hashing::hash(&key))
	}
}

impl<B, H> OfferId<B, H> {
	pub fn new<Config>(
		expiration_block: B,
		ask_order_id: &AskOrderId<BlockNumberFor<Config>, H>,
		bid_order_id: &BidOrderId<BlockNumberFor<Config>, H>,
	) -> Self
	where
		Config: frame_system::Config<BlockNumber = B>,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		let ask_bytes = ask_order_id.1.as_ref();
		let bid_bytes = bid_order_id.1.as_ref();
		let key = concatenate!(ask_bytes, bid_bytes);
		OfferId(expiration_block, Config::Hashing::hash(&key))
	}
}

impl<B, H> DealOrderId<B, H> {
	pub fn new<Config>(expiration_block: B, offer_id: &OfferId<BlockNumberFor<Config>, H>) -> Self
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
		H: AsRef<[u8]>,
	{
		DealOrderId(expiration_block, Config::Hashing::hash(offer_id.1.as_ref()))
	}
}

pub(crate) trait Id<BlockNum, Hash> {
	fn expiration(&self) -> BlockNum;
	fn hash(&self) -> Hash;
}

macro_rules! impl_id {
	($id: ident) => {
		impl<BlockNum, Hash> Id<BlockNum, Hash> for $id<BlockNum, Hash>
		where
			BlockNum: Clone,
			Hash: Clone,
		{
			fn expiration(&self) -> BlockNum {
				self.0.clone()
			}
			fn hash(&self) -> Hash {
				self.1.clone()
			}
		}

		impl<'a, BlockNum, Hash> Id<BlockNum, Hash> for &'a $id<BlockNum, Hash>
		where
			BlockNum: Clone,
			Hash: Clone,
		{
			fn expiration(&self) -> BlockNum {
				self.0.clone()
			}
			fn hash(&self) -> Hash {
				self.1.clone()
			}
		}

		impl<BlockNum, H> $id<BlockNum, H> {
			pub fn with_expiration_hash<Config>(expiration_block: BlockNum, hash: H) -> Self
			where
				Config: frame_system::Config<BlockNumber = BlockNum>,
				<Config as frame_system::Config>::Hashing: Hash<Output = H>,
			{
				Self(expiration_block, hash)
			}
		}
	};
}

impl_id!(DealOrderId);
impl_id!(AskOrderId);
impl_id!(BidOrderId);
impl_id!(OfferId);
impl_id!(RepaymentOrderId);

impl<'a, B, H> Id<B, H> for &'a OrderId<B, H>
where
	B: Clone,
	H: Clone,
{
	fn expiration(&self) -> B {
		match self {
			OrderId::Deal(deal) => deal.expiration(),
			OrderId::Repayment(repay) => repay.expiration(),
		}
	}

	fn hash(&self) -> H {
		match self {
			OrderId::Deal(deal) => deal.hash(),
			OrderId::Repayment(repay) => repay.hash(),
		}
	}
}
impl<B, H> Id<B, H> for OrderId<B, H>
where
	B: Clone,
	H: Clone,
{
	fn expiration(&self) -> B {
		(&self).expiration()
	}

	fn hash(&self) -> H {
		(&self).hash()
	}
}

#[ext(name = DoubleMapExt)]
pub(crate) impl<Prefix, Hasher1, Key1, Hasher2, Key2, Value, QueryKind, OnEmpty, MaxValues, IdTy>
	frame_support::storage::types::StorageDoubleMap<
		Prefix,
		Hasher1,
		Key1,
		Hasher2,
		Key2,
		Value,
		QueryKind,
		OnEmpty,
		MaxValues,
	> where
	Prefix: StorageInstance,
	Hasher1: StorageHasher,
	Hasher2: StorageHasher,
	Key1: FullCodec + Clone,
	Key2: FullCodec + Clone,
	Value: FullCodec,
	QueryKind: QueryKindTrait<Value, OnEmpty>,
	OnEmpty: Get<QueryKind::Query> + 'static,
	MaxValues: Get<Option<u32>>,
	IdTy: Id<Key1, Key2>,
{
	fn insert_id<V>(id: IdTy, val: V)
	where
		V: EncodeLike<Value>,
	{
		Self::insert(id.expiration(), id.hash(), val);
	}

	fn try_get_id(id: &IdTy) -> frame_support::dispatch::result::Result<Value, ()> {
		Self::try_get(id.expiration(), id.hash())
	}

	fn contains_id(id: &IdTy) -> bool {
		Self::contains_key(id.expiration(), id.hash())
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct LegacySighash([u8; 60]);

impl core::default::Default for LegacySighash {
	fn default() -> LegacySighash {
		LegacySighash([0u8; 60])
	}
}

impl From<&ecdsa::Public> for LegacySighash {
	fn from(public_key: &ecdsa::Public) -> Self {
		let compressed_key_hex = hex::encode(public_key.as_ref());
		let mut hasher = sha2::Sha512::new();
		hasher.update(compressed_key_hex.as_bytes());
		let key_hash = hasher.finalize();
		let key_hash_hex = hex::encode(&key_hash);

		const SKIP_TO_GET_60: usize = 512 / 8 * 2 - 60; // 512 - hash size in bits, 8 - bits in byte, 2 - hex digits for byte, 60 - merkle address length (70) without creditcoin namespace length (6) and prefix length (4)

		LegacySighash::try_from(&key_hash_hex[SKIP_TO_GET_60..])
			.expect("the output of Sha512 is 64 bytes. the hex encoding of that is 128 bytes,\
			therefore key_hash_hex[68..] must be 128-68=60 bytes long and so the conversion to [u8; 60] cannot fail; qed")
	}
}

impl TryFrom<&str> for LegacySighash {
	type Error = ();

	fn try_from(hex: &str) -> Result<Self, Self::Error> {
		if hex.len() == 60 {
			let mut res = LegacySighash::default();
			res.0.copy_from_slice(hex.as_bytes());
			Ok(res)
		} else {
			Err(())
		}
	}
}

#[cfg(feature = "std")]
impl serde::Serialize for LegacySighash {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(
			core::str::from_utf8(self.0.as_slice())
				.expect("LegacySighash can only be constructed with valid UTF-8, through `Default` or `TryFrom`; qed"),
		)
	}
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for LegacySighash {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Self::try_from(&*String::deserialize(deserializer)?)
			.map_err(|()| serde::de::Error::custom("expected 60 bytes"))
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Task<AccountId, BlockNum, Hash, Moment> {
	VerifyTransfer(UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>),
	CollectCoins(UnverifiedCollectedCoins),
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>) -> Self {
		Task::VerifyTransfer(transfer)
	}
}

impl<AccountId, BlockNum, Hash, Moment> From<UnverifiedCollectedCoins>
	for Task<AccountId, BlockNum, Hash, Moment>
{
	fn from(coins: UnverifiedCollectedCoins) -> Self {
		Task::CollectCoins(coins)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskId<Hash> {
	VerifyTransfer(TransferId<Hash>),
	CollectCoins(CollectedCoinsId<Hash>),
}

impl<Hash> From<TransferId<Hash>> for TaskId<Hash> {
	fn from(id: TransferId<Hash>) -> Self {
		TaskId::VerifyTransfer(id)
	}
}

impl<Hash> From<CollectedCoinsId<Hash>> for TaskId<Hash> {
	fn from(id: CollectedCoinsId<Hash>) -> Self {
		TaskId::CollectCoins(id)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskOutput<AccountId, Balance, BlockNum, Hash, Moment> {
	VerifyTransfer(TransferId<Hash>, Transfer<AccountId, BlockNum, Hash, Moment>),
	CollectCoins(CollectedCoinsId<Hash>, CollectedCoins<Hash, Balance>),
}

impl<AccountId, Balance, BlockNum, Hash, Moment>
	From<(TransferId<Hash>, Transfer<AccountId, BlockNum, Hash, Moment>)>
	for TaskOutput<AccountId, Balance, BlockNum, Hash, Moment>
{
	fn from(
		(id, transfer): (TransferId<Hash>, Transfer<AccountId, BlockNum, Hash, Moment>),
	) -> Self {
		Self::VerifyTransfer(id, transfer)
	}
}

impl<AccountId, Balance, BlockNum, Hash, Moment>
	From<(CollectedCoinsId<Hash>, CollectedCoins<Hash, Balance>)>
	for TaskOutput<AccountId, Balance, BlockNum, Hash, Moment>
{
	fn from((id, coins): (CollectedCoinsId<Hash>, CollectedCoins<Hash, Balance>)) -> Self {
		Self::CollectCoins(id, coins)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskData<AccountId, Balance, BlockNum, Hash, Moment> {
	VerifyTransfer(UnverifiedTransfer<AccountId, BlockNum, Hash, Moment>, Option<Moment>),
	CollectCoins(UnverifiedCollectedCoins, Balance),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TaskOracleData<Balance, Moment> {
	VerifyTransfer(Option<Moment>),
	CollectCoins(Balance),
}
