pub mod loan_terms;
pub mod platform;

pub use loan_terms::*;
pub use platform::*;

use crate::ocw::tasks::collect_coins::GCreContract;
use extend::ext;
use frame_support::{
	storage::types::QueryKindTrait,
	traits::{ConstU32, Get, StorageInstance},
	BoundedVec, RuntimeDebug, StorageHasher,
};
use frame_system::pallet_prelude::BlockNumberFor;
use parity_scale_codec::{Decode, Encode, EncodeLike, FullCodec, MaxEncodedLen};
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

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum LegacyTransferKind {
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
pub struct CollectedCoins<Hash, Balance> {
	pub to: AddressId<Hash>,
	pub amount: Balance,
	pub tx_id: ExternalTxId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Transfer<AccountId, BlockNum, Hash, Moment> {
	pub blockchain: Blockchain,
	pub kind: TransferKind,
	pub from: AddressId<Hash>,
	pub to: AddressId<Hash>,
	pub deal_order_id: DealOrderId<BlockNum, Hash>,
	pub amount: ExternalAmount,
	pub tx_id: ExternalTxId,
	pub block: BlockNum,
	pub is_processed: bool,
	pub account_id: AccountId,
	pub timestamp: Option<Moment>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnverifiedCollectedCoins {
	pub to: ExternalAddress,
	pub tx_id: ExternalTxId,
	pub contract: GCreContract,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> {
	pub transfer: Transfer<AccountId, BlockNum, Hash, Moment>,
	pub from_external: ExternalAddress,
	pub to_external: ExternalAddress,
	pub deadline: BlockNum,
	pub currency_to_check: CurrencyOrLegacyTransferKind,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum CurrencyOrLegacyTransferKind {
	Currency(Currency),
	TransferKind(LegacyTransferKind),
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Offer<AccountId, BlockNum, Hash> {
	pub ask_id: AskOrderId<BlockNum, Hash>,
	pub bid_id: BidOrderId<BlockNum, Hash>,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AskOrder<AccountId, BlockNum, Hash> {
	pub lender_address_id: AddressId<Hash>,
	pub terms: AskTerms<Hash>,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub lender: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct BidOrder<AccountId, BlockNum, Hash> {
	pub borrower_address_id: AddressId<Hash>,
	pub terms: BidTerms<Hash>,
	pub expiration_block: BlockNum,
	pub block: BlockNum,
	pub borrower: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DealOrder<AccountId, BlockNum, Hash, Moment> {
	pub offer_id: OfferId<BlockNum, Hash>,
	pub lender_address_id: AddressId<Hash>,
	pub borrower_address_id: AddressId<Hash>,
	pub terms: LoanTerms<Hash>,
	pub expiration_block: BlockNum,
	pub timestamp: Moment,
	pub block: Option<BlockNum>,
	pub funding_transfer_id: Option<TransferId<Hash>>,
	pub repayment_transfer_id: Option<TransferId<Hash>>,
	pub lock: Option<AccountId>,
	pub borrower: AccountId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AddressId<Hash>(Hash);

#[cfg(test)]
impl<Hash> AddressId<Hash> {
	pub fn make(hash: Hash) -> Self {
		Self(hash)
	}
}

#[derive(
	Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, PartialOrd, Ord,
)]
pub struct AskOrderId<BlockNum, Hash>(BlockNum, Hash);

#[derive(
	Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, PartialOrd, Ord,
)]
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
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct OfferId<BlockNum, Hash>(BlockNum, Hash);

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransferId<Hash>(Hash);

#[cfg(test)]
impl<Hash> TransferId<Hash> {
	pub fn make(hash: Hash) -> Self {
		Self(hash)
	}
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CollectedCoinsId<Hash>(Hash);

#[cfg(test)]
impl<Hash> CollectedCoinsId<Hash> {
	pub fn make(hash: Hash) -> Self {
		Self(hash)
	}
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

impl<H> AddressId<H> {
	pub fn new<Config>(blockchain: &Blockchain, address: &[u8]) -> AddressId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(&*blockchain.as_bytes(), address);
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

impl<H> TransferId<H> {
	pub fn new<Config>(blockchain: &Blockchain, blockchain_tx_id: &[u8]) -> TransferId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(&*blockchain.as_bytes(), blockchain_tx_id);
		TransferId(Config::Hashing::hash(&key))
	}
}

impl<H> CollectedCoinsId<H> {
	pub fn new<Config>(contract_chain: &Blockchain, blockchain_tx_id: &[u8]) -> CollectedCoinsId<H>
	where
		Config: frame_system::Config,
		<Config as frame_system::Config>::Hashing: Hash<Output = H>,
	{
		let key = concatenate!(contract_chain.as_bytes().iter(), blockchain_tx_id);
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

#[cfg(test)]
mod test {
	use crate::{
		helpers::HexToAddress, mock, ocw::tasks::collect_coins::tests::TX_HASH, tests::TestInfo, *,
	};
	use parity_scale_codec::{Decode, Encode};
	use sp_runtime::testing::H256;

	type AccountId = mock::AccountId;
	type Balance = mock::Balance;
	type BlockNum = mock::BlockNumber;
	type Hash = H256;
	type Moment = u64;

	macro_rules! implements {(
		$T:ty : $($bounds:tt)*
	) => ({
		use ::core::marker::PhantomData;

		trait DefaultValue {
			fn value (self: &'_ Self) -> bool { false }
		}
		impl<T : ?Sized> DefaultValue for &'_ PhantomData<T> {}
		trait SpecializedValue {
			fn value (self: &'_ Self) -> bool { true }
		}
		impl<T : ?Sized> SpecializedValue for PhantomData<T>
		where
			T : $($bounds)*
		{}
		(&PhantomData::<$T>).value()
	})}

	macro_rules! trait_tests {
	($($name:ident: $type:ty: $default_value:expr,)*) => {
		use parity_scale_codec::MaxEncodedLen;
		use scale_info::TypeInfo;
	$(
		mod $name {
			use super::*;

			#[test]
			fn test_typeinfo() {
				<$type>::type_info();
			}

			#[test]
			fn test_maxencodedlen() {
				if (implements!($type : MaxEncodedLen)) {
					let result = <$type>::max_encoded_len();
					assert!(result > 0);
				}
			}

			#[test]
			fn test_encode_decode() {
				if (implements!($type : Encode)) {
					mock::ExtBuilder::default().build_and_execute(|| {
						// assign $default_value to a local variable to prevent double
						// evaluation which leads to AddressAlreadyRegistered error
						let value = $default_value;

						let as_scale = value.encode();
						assert!(as_scale.len() > 0);

						let decoded = <$type>::decode(&mut &as_scale[..]).unwrap();
						assert_eq!(decoded, value);
					})
				}
			}

			#[test]
			fn test_runtimedebug() {
				mock::ExtBuilder::default().build_and_execute(|| {
					let value = $default_value;
					format!("{:?}", value);
				})
			}

			#[test]
			fn test_clone_and_partialeq() {
				mock::ExtBuilder::default().build_and_execute(|| {
					let a = $default_value;
					let b = a.clone();
					let c = b.clone();

					// exercise equality comparisons, see
					// https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
					// https://users.rust-lang.org/t/what-is-the-difference-between-eq-and-partialeq/15751/2

					// symmetric
					assert!(a == b);
					assert!(b == a);

					// transitive
					assert!(a == b);
					assert!(b == c);
					assert!(a == c);
				})
			}

		}
	)*}}

	fn create_funding_transfer() -> tests::TestTransfer {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		test_info.create_funding_transfer(&deal_order_id)
	}

	fn create_collected_coins() -> CollectedCoins<Hash, Balance> {
		CollectedCoins {
			to: AddressId::new::<mock::Test>(&Blockchain::RINKEBY, b"tester"),
			amount: 1000,
			tx_id: TX_HASH.hex_to_address(),
		}
	}

	fn create_unverified_collected_coins() -> UnverifiedCollectedCoins {
		UnverifiedCollectedCoins {
			to: b"baba".to_vec().try_into().unwrap(),
			tx_id: TX_HASH.hex_to_address(),
			contract: Default::default(),
		}
	}

	fn create_unverified_transfer() -> UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> {
		let test_info = TestInfo::new_defaults();
		let (deal_order_id, _) = test_info.create_deal_order();
		let (_, transfer) = test_info.create_funding_transfer(&deal_order_id);
		UnverifiedTransfer {
			currency_to_check: CurrencyOrLegacyTransferKind::Currency(test_info.currency),
			transfer,
			from_external: b"lender".to_vec().try_into().unwrap(),
			to_external: b"borrower".to_vec().try_into().unwrap(),
			deadline: 1_000_000,
		}
	}

	fn create_address() -> Address<AccountId> {
		Address {
			blockchain: Blockchain::RINKEBY,
			value: ExternalAddress::try_from(
				hex::decode("09231da7b19A016f9e576d23B16277062F4d46A8").unwrap(),
			)
			.unwrap(),
			owner: AccountId::new([77; 32]),
		}
	}

	trait_tests! {
	address: Address<AccountId> : create_address(),
	collected_coins: CollectedCoins<Hash, Balance> : create_collected_coins(),
	transfer: Transfer<AccountId, BlockNum, Hash, Moment> : create_funding_transfer().1,
	unverified_collected_coins: UnverifiedCollectedCoins : create_unverified_collected_coins(),
	unverified_transfer: UnverifiedTransfer<AccountId, BlockNum, Hash, Moment> : create_unverified_transfer(),
	offer: Offer<AccountId, BlockNum, Hash> : TestInfo::new_defaults().create_offer().1,
	ask_order: AskOrder<AccountId, BlockNum, Hash> : TestInfo::new_defaults().create_ask_order().1,
	bid_order: BidOrder<AccountId, BlockNum, Hash> : TestInfo::new_defaults().create_bid_order().1,
	deal_order: DealOrder<AccountId, BlockNum, Hash, Moment> : TestInfo::new_defaults().create_deal_order().1,
	address_id: AddressId<Hash> : AddressId::new::<mock::Test>(&Blockchain::RINKEBY, b"0"),
	ask_order_id: AskOrderId<BlockNum, Hash> : TestInfo::new_defaults().create_ask_order().0,
	bid_order_id: BidOrderId<BlockNum, Hash> : TestInfo::new_defaults().create_bid_order().0,
	deal_order_id: DealOrderId<BlockNum, Hash> : TestInfo::new_defaults().create_deal_order().0,
	offer_id: OfferId<BlockNum, Hash> : TestInfo::new_defaults().create_offer().0,
	transfer_id: TransferId<Hash> : TransferId::new::<mock::Test>(&Blockchain::RINKEBY, b"0"),
	collected_coins_id: CollectedCoinsId<Hash> : CollectedCoinsId::new::<mock::Test>(&Blockchain::RINKEBY, &[0]),
	legacy_sighash: LegacySighash : LegacySighash::default(),
	task: Task<AccountId, BlockNum, Hash, Moment> : Task::<AccountId, BlockNum, Hash, Moment>::from(create_unverified_collected_coins()),
	task_id: TaskId<Hash> : TaskId::from(create_funding_transfer().0),
	task_output: TaskOutput<AccountId, Balance, BlockNum, Hash, Moment> : TaskOutput::<AccountId, Balance, BlockNum, Hash, Moment>::from(
		create_funding_transfer()
	),
	task_data: TaskData<AccountId, Balance, BlockNum, Hash, Moment> : TaskData::<AccountId, Balance, BlockNum, Hash, Moment>::CollectCoins(
		create_unverified_collected_coins(), 2000
	),

	// from types/loan_terms.rs
	duration: Duration : Duration::from_millis(100),
	interest_type: InterestType : InterestType::Simple,
	interest_rate: InterestRate : InterestRate::default(),
	loan_terms: LoanTerms<Hash> : TestInfo::new_defaults().loan_terms,
	ask_terms: AskTerms<Hash> : AskTerms::try_from(TestInfo::new_defaults().loan_terms).unwrap(),
	bid_terms: BidTerms<Hash> : BidTerms::try_from(TestInfo::new_defaults().loan_terms).unwrap(),

	// from types/platform.rs
	evm_chain_id: EvmChainId : EvmChainId::from(44),
	evm_info: EvmInfo : EvmInfo { chain_id: 0.into() },
	blockchain: Blockchain : Blockchain::Evm(EvmInfo { chain_id: 0.into() }),
	evm_transfer_kind: EvmTransferKind : EvmTransferKind::Erc20,
	evm_currency_type: EvmCurrencyType : match Currency::default() {
		Currency::Evm(currency_type, _) => currency_type,
	},
	currency: Currency : Currency::default(),
	transfer_kind: TransferKind : TransferKind::Evm(EvmTransferKind::Erc20),
	currency_id: CurrencyId<Hash> : CurrencyId::new::<mock::Test>(&Currency::default()),
	}

	#[test]
	fn test_blockchain_as_bytes() {
		let other = Blockchain::Evm(EvmInfo { chain_id: EvmChainId::new(10) });
		assert_eq!(&*other.as_bytes(), b"evm-10");
	}

	#[test]
	fn test_legacysighash_try_from_when_string_is_shorter_than_60_chars() {
		let result = LegacySighash::try_from("too-short");
		assert!(result.is_err());
	}

	#[test]
	fn test_legacysighash_try_from_when_string_is_longer_than_60_chars() {
		let result = LegacySighash::try_from(
			"this-dummy-string-is-very-very-very-long-and-cannot-be-a-legacy-sighash",
		);
		assert!(result.is_err());
	}

	#[test]
	fn test_legacysighash_serialize_deserialize() {
		let value = LegacySighash::default();

		let json_string = serde_json::to_string(&value).unwrap();
		let deserialized_value = serde_json::from_str(&json_string).unwrap();
		assert_eq!(value, deserialized_value);
	}

	#[test]
	fn test_duration_new() {
		let result = Duration::new(5u64, 4000000u32);
		assert_eq!(result, Duration::from_millis(5004));
	}

	#[test]
	fn test_bidterms_match_with() {
		mock::ExtBuilder::default().build_and_execute(|| {
			let loan_terms = TestInfo::new_defaults().loan_terms;
			let ask_terms = AskTerms::try_from(loan_terms.clone()).unwrap();
			let bid_terms = BidTerms::try_from(loan_terms).unwrap();

			assert!(bid_terms.match_with(&ask_terms));
		})
	}

	#[test]
	fn test_bidterms_agreed_terms() {
		mock::ExtBuilder::default().build_and_execute(|| {
			let loan_terms = TestInfo::new_defaults().loan_terms;
			let ask_terms = AskTerms::try_from(loan_terms.clone()).unwrap();
			let bid_terms = BidTerms::try_from(loan_terms).unwrap();

			assert_eq!(
				ask_terms.agreed_terms(bid_terms.clone()),
				bid_terms.agreed_terms(&ask_terms),
			);
		})
	}

	#[test]
	fn test_evmchainid_new() {
		assert_eq!(EvmChainId::new(46), EvmChainId::from(46),);
	}

	#[test]
	#[allow(clippy::clone_on_copy)]
	fn exercise_invalid_term_length_error_clone_and_runtime_debug() {
		let value = InvalidTermLengthError;
		let new_value = value.clone();
		format!("{:?}", new_value);
	}
}
