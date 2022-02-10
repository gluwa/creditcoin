use codec::{Decode, Encode, EncodeLike, FullCodec, MaxEncodedLen};
use extend::ext;
use frame_support::{
	storage::types::QueryKindTrait,
	traits::{ConstU32, Get, StorageInstance},
	BoundedVec, RuntimeDebug, StorageHasher,
};
use frame_system::pallet_prelude::BlockNumberFor;
use scale_info::TypeInfo;
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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Blockchain {
	Ethereum,
	Rinkeby,
	Luniverse,
	Bitcoin,
	Other(OtherChain),
}

impl Blockchain {
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			Blockchain::Ethereum => &*b"ethereum",
			Blockchain::Rinkeby => &*b"rinkeby",
			Blockchain::Luniverse => &*b"luniverse",
			Blockchain::Bitcoin => &*b"bitcoin",
			Blockchain::Other(chain) => chain.as_slice(),
		}
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum TransferKind {
	Erc20(ExternalAddress),
	Ethless(ExternalAddress),
	Native,
	Other(OtherTransferKind),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Address<AccountId> {
	pub blockchain: Blockchain,
	pub value: ExternalAddress,
	pub owner: AccountId,
}

impl<AccountId> Address<AccountId> {
	pub fn matches_chain_of(&self, other: &Address<AccountId>) -> bool {
		self.blockchain == other.blockchain
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Transfer<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub kind: TransferKind,
	pub from: AddressId<Hash>,
	pub to: AddressId<Hash>,
	pub order: OrderId<BlockNum, Hash>,
	pub amount: ExternalAmount,
	pub tx: ExternalTxId,
	pub block: BlockNum,
	pub processed: bool,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnverifiedTransfer<AccountId, BlockNum, Hash> {
	pub transfer: Transfer<AccountId, BlockNum, Hash>,
	pub from_external: ExternalAddress,
	pub to_external: ExternalAddress,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Offer<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub ask_order: AskOrderId<BlockNum, Hash>,
	pub bid_order: BidOrderId<BlockNum, Hash>,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskOrder<AccountId, Balance, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: Moment,
	pub fee: Balance,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Fee<BlockNum> {
	pub block: BlockNum,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidOrder<AccountId, Balance, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: Moment,
	pub fee: Balance,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RepaymentOrder<AccountId, BlockNum, Hash> {
	pub blockchain: Blockchain,
	pub src_address: AddressId<Hash>,
	pub dst_address: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub deal: DealOrderId<BlockNum, Hash>,
	pub previous_owner: Option<AccountId>,
	pub transfer: TransferId<Hash>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DealOrder<AccountId, Balance, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub lender: AddressId<Hash>,
	pub borrower: AddressId<Hash>,
	pub amount: ExternalAmount,
	pub interest: ExternalAmount,
	pub maturity: Moment,
	pub fee: Balance,
	pub expiration_block: BlockNum,
	pub timestamp: Moment,
	pub loan_transfer: Option<TransferId<Hash>>,
	pub repayment_transfer: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub sighash: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AddressId<Hash>(Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DealOrderId<BlockNum, Hash>(BlockNum, Hash);

#[cfg(test)]
impl<B: Default, H: Default> DealOrderId<B, H> {
	pub fn dummy() -> Self {
		Self(B::default(), H::default())
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RepaymentOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum OrderId<BlockNum, Hash> {
	Deal(DealOrderId<BlockNum, Hash>),
	Repayment(RepaymentOrderId<BlockNum, Hash>),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OfferId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransferId<Hash>(Hash);

fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
	const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
	let mut hex = Vec::with_capacity(bytes.len() * 2);
	for byte in bytes {
		hex.push(HEX_CHARS_LOWER[(byte >> 4) as usize]);
		hex.push(HEX_CHARS_LOWER[(byte & 0x0F) as usize]);
	}
	hex
}

macro_rules! strip_plus {
    (+ $($rest: tt)*) => {
        $($rest)*
    }
}

macro_rules! concatenate {
	($($bytes: expr),+) => {
		{
			let mut buf = Vec::with_capacity(strip_plus!($(+ $bytes.len())+));
			$(buf.extend($bytes);)+
			buf
		}
	};

	($($bytes: expr),+; $last_bytes: expr; sep = $sep: literal) => {
		{
			let mut buf = Vec::with_capacity(strip_plus!($(+ $bytes.len())+) + count_tts!($($bytes)+) );
			$(
				buf.extend($bytes);
				buf.push($sep);
			)+
			buf.extend($last_bytes);
			buf
		}
	}
}

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
