use crate::{
	self as pallet_creditcoin,
	ocw::rpc::{JsonRpcRequest, JsonRpcResponse},
	Blockchain, LegacySighash,
};
use ethereum_types::U256;
use frame_support::{
	once_cell::sync::Lazy,
	parameter_types,
	traits::{ConstU32, ConstU64, GenesisBuild, Hooks},
};
use frame_system as system;
use pallet_offchain_task_scheduler::crypto::AuthorityId;
pub(crate) use pallet_offchain_task_scheduler::tasks::TaskScheduler as TaskSchedulerT;
pub(crate) use parking_lot::RwLock;
use serde_json::Value;
use sp_core::H256;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
pub(crate) use sp_runtime::offchain::testing::OffchainState;
use sp_runtime::{
	offchain::{
		storage::StorageValueRef,
		testing::{PendingRequest, PoolState, TestOffchainExt, TestTransactionPoolExt},
		OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
	},
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature, RuntimeAppPublic,
};
pub(crate) use std::sync::Arc;
use std::{cell::Cell, collections::HashMap};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub(crate) type Balance = u128;
pub type Signature = MultiSignature;
pub type Extrinsic = TestXt<RuntimeCall, ()>;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type BlockNumber = u64;
pub type Hash = H256;
pub type Moment = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Creditcoin: pallet_creditcoin::{Pallet, Call, Storage, Event<T>, Config<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		TaskScheduler: pallet_offchain_task_scheduler::{Pallet, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	// used in tests, lower values == faster execution
	pub const PendingTxLimit: u32 = 500;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<{ u32::MAX }>;
}

impl pallet_timestamp::Config for Test {
	type Moment = Moment;

	type OnTimestampSet = ();

	type MinimumPeriod = ConstU64<1>;

	type WeightInfo = ();
}

impl pallet_creditcoin::Config for Test {
	type RuntimeEvent = RuntimeEvent;

	type Call = RuntimeCall;

	type Signer = <Signature as Verify>::Signer;
	type SignerSignature = Signature;

	type HashIntoNonce = H256;

	type UnverifiedTaskTimeout = ConstU64<5>;

	type WeightInfo = super::weights::WeightInfo<Test>;

	type TaskScheduler = Self;
}

impl pallet_offchain_task_scheduler::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type UnverifiedTaskTimeout = ConstU64<5>;
	type AuthorityId = AuthorityId;
	type AccountIdFrom = AccountId;
	type InternalPublic = sp_core::sr25519::Public;
	type PublicSigning = <Signature as Verify>::Signer;
	type TaskCall = RuntimeCall;
	type WeightInfo = pallet_offchain_task_scheduler::weights::WeightInfo<Self>;
	type Task = pallet_creditcoin::Task<AccountId, BlockNumber, Hash, Moment>;
}

impl Test {
	pub(crate) fn unverified_transfer_deadline() -> u64 {
		Test::deadline()
	}
}

thread_local! {
	pub static CREATE_TRANSACTION_FAIL: Cell<bool> = Cell::new(false);
}

pub(crate) fn with_failing_create_transaction<R>(f: impl FnOnce() -> R) -> R {
	CREATE_TRANSACTION_FAIL.with(|c| {
		c.set(true);
		let result = f();
		c.set(false);
		result
	})
}

impl<LocalCall> system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: Self::Public,
		_account: Self::AccountId,
		nonce: Self::Index,
	) -> Option<(RuntimeCall, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		CREATE_TRANSACTION_FAIL.with(|should_fail| {
			if should_fail.get() {
				eprintln!("Failing!");
				None
			} else {
				eprintln!("Not failing!");
				Some((call, (nonce, ())))
			}
		})
	}
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
	RuntimeCall: From<C>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}
impl system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

#[allow(dead_code)]
#[derive(Default)]
pub struct ExtBuilder {
	balances: Vec<(AccountId, Balance)>,
	authorities: Vec<AccountId>,
	legacy_wallets: Vec<(LegacySighash, Balance)>,
	legacy_keeper: Option<AccountId>,
	keystore: Option<KeyStore>,
}

impl ExtBuilder {
	#[allow(dead_code)]
	pub fn fund(&mut self, account: AccountId, amount: Balance) -> &mut ExtBuilder {
		self.balances.push((account, amount));
		self
	}
	pub fn generate_authority(&mut self) -> sp_core::sr25519::Public {
		const PHRASE: &str =
			"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
		if self.keystore.is_none() {
			self.keystore = Some(KeyStore::new());
		}
		let pubkey = self
			.keystore
			.as_ref()
			.unwrap()
			.sr25519_generate_new(
				pallet_offchain_task_scheduler::crypto::Public::ID,
				Some(&format!("{}/auth{}", PHRASE, self.authorities.len() + 1)),
			)
			.unwrap();
		self.authorities.push(AccountId::new(pubkey.into_account().0));
		pubkey
	}
	pub fn legacy_wallets(
		&mut self,
		wallets: impl IntoIterator<Item = (LegacySighash, Balance)>,
	) -> &mut ExtBuilder {
		self.legacy_wallets.extend(wallets);
		self
	}
	pub fn legacy_balance_keeper(&mut self, acct: AccountId) -> &mut ExtBuilder {
		self.legacy_keeper = Some(acct);
		self
	}
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let _ = pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut storage);

		pallet_offchain_task_scheduler::pallet::GenesisConfig::<Test> {
			authorities: self.authorities,
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		let _ = crate::GenesisConfig::<Test> {
			authorities: vec![],
			legacy_wallets: self.legacy_wallets,
			legacy_balance_keeper: self.legacy_keeper,
		}
		.assimilate_storage(&mut storage);

		storage.into()
	}

	pub fn build_with(
		mut self,
		offchain: TestOffchainExt,
	) -> (sp_io::TestExternalities, Arc<RwLock<PoolState>>) {
		if self.keystore.is_none() {
			self.keystore = Some(KeyStore::new());
		}
		let keystore = core::mem::take(&mut self.keystore);
		let mut ext = self.build();

		ext.register_extension(OffchainDbExt::new(offchain.clone()));
		ext.register_extension(OffchainWorkerExt::new(offchain));
		let (pool, p) = TestTransactionPoolExt::new();
		ext.register_extension(TransactionPoolExt::new(pool));

		if let Some(keystore) = keystore {
			ext.register_extension(KeystoreExt(Arc::new(keystore)));
		}
		(ext, p)
	}

	pub fn build_offchain(
		mut self,
	) -> (sp_io::TestExternalities, Arc<RwLock<OffchainState>>, Arc<RwLock<PoolState>>) {
		if self.keystore.is_none() {
			self.keystore = Some(KeyStore::new());
		}
		let keystore = core::mem::take(&mut self.keystore);
		let mut ext = self.build();
		let (offchain, offchain_state) = TestOffchainExt::new();
		let (pool, pool_state) = TestTransactionPoolExt::new();
		ext.register_extension(OffchainDbExt::new(offchain.clone()));
		ext.register_extension(OffchainWorkerExt::new(offchain));
		ext.register_extension(TransactionPoolExt::new(pool));
		if let Some(keystore) = keystore {
			ext.register_extension(KeystoreExt(Arc::new(keystore)));
		}

		(ext, offchain_state, pool_state)
	}
	pub fn build_and_execute<R>(self, test: impl FnOnce() -> R) -> R {
		self.build().execute_with(test)
	}
	#[allow(dead_code)]
	pub fn build_offchain_and_execute<R>(self, test: impl FnOnce() -> R) -> R {
		let (mut ext, _, _) = self.build_offchain();
		ext.execute_with(test)
	}
	#[allow(dead_code)]
	pub fn build_offchain_and_execute_with_state<R>(
		self,
		test: impl FnOnce(Arc<RwLock<OffchainState>>, Arc<RwLock<PoolState>>) -> R,
	) -> R {
		let (mut ext, state, pool) = self.build_offchain();
		ext.execute_with(|| test(state, pool))
	}
}

#[allow(dead_code)]
pub fn roll_to(n: BlockNumber) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		TaskScheduler::on_initialize(i);
		TaskScheduler::on_finalize(i);
	}
}

#[allow(dead_code)]
pub fn roll_to_with_ocw(n: BlockNumber) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		TaskScheduler::on_initialize(i);
		TaskScheduler::offchain_worker(i);
		TaskScheduler::on_finalize(i);
	}
}

#[allow(dead_code)]
pub fn roll_by_with_ocw(n: BlockNumber) {
	let mut now = System::block_number();
	for _ in 0..n {
		TaskScheduler::offchain_worker(now);
		now += 1;
		System::set_block_number(now);
		System::reset_events();
		System::on_initialize(now);
		TaskScheduler::on_initialize(now);
		TaskScheduler::on_finalize(now);
	}
}

// must be called in an externalities-provided environment
pub fn set_rpc_uri(blockchain: &Blockchain, value: impl AsRef<[u8]>) {
	let key = blockchain.rpc_key();
	let rpc_url_storage = StorageValueRef::persistent(&key);
	rpc_url_storage.set(&value.as_ref());
}

pub fn pending_rpc_request(
	method: &str,
	params: impl IntoIterator<Item = serde_json::Value>,
	uri: &str,
	responses: &HashMap<String, JsonRpcResponse<serde_json::Value>>,
) -> PendingRequest {
	let x = JsonRpcRequest::new(method, params);
	let rpc = x.to_bytes();
	let response = &responses[method];
	let response_body = serde_json::to_vec(response).unwrap();
	PendingRequest {
		method: "POST".into(),
		uri: uri.into(),
		headers: vec![("Content-Type".into(), "application/json".into())],
		body: rpc,
		response: Some(response_body),
		response_headers: vec![("Content-Type".into(), "application/json".into())],
		sent: true,
		..Default::default()
	}
}

pub(crate) static ETHLESS_RESPONSES: Lazy<HashMap<String, JsonRpcResponse<serde_json::Value>>> =
	Lazy::new(|| serde_json::from_slice(include_bytes!("tests/ethlessTransfer.json")).unwrap());

pub(crate) fn get_mock_tx_hash() -> String {
	let responses = &*ETHLESS_RESPONSES;
	responses["eth_getTransactionByHash"].result.clone().unwrap()["hash"]
		.clone()
		.as_str()
		.unwrap()
		.to_string()
}

pub(crate) fn get_mock_contract() -> String {
	let responses = &*ETHLESS_RESPONSES;

	responses["eth_getTransactionByHash"].result.clone().unwrap()["to"]
		.clone()
		.as_str()
		.unwrap()
		.to_string()
}

pub(crate) fn get_mock_tx_block_num() -> String {
	let responses = &*ETHLESS_RESPONSES;

	responses["eth_getTransactionByHash"].result.clone().unwrap()["blockNumber"]
		.clone()
		.as_str()
		.unwrap()
		.to_string()
}

pub(crate) fn get_mock_from_address() -> String {
	format!("{:?}", get_mock_contract_input(0, ethabi::Token::into_address))
}

fn get_mock_contract_input<T>(index: usize, convert: impl FnOnce(ethabi::Token) -> Option<T>) -> T {
	let responses = &*ETHLESS_RESPONSES;

	let abi = crate::ocw::tasks::verify_transfer::ethless_transfer_function_abi();
	let input = responses["eth_getTransactionByHash"].result.clone().unwrap()["input"]
		.clone()
		.as_str()
		.unwrap()
		.to_string();
	let input_bytes = hex::decode(&input.trim_start_matches("0x")).unwrap();
	let inputs = abi.decode_input(&input_bytes[4..]).unwrap();
	convert(inputs[index].clone()).unwrap()
}

pub(crate) fn get_mock_input_data() -> String {
	let responses = &*ETHLESS_RESPONSES;

	responses["eth_getTransactionByHash"].result.clone().unwrap()["input"]
		.clone()
		.as_str()
		.unwrap()
		.to_string()
}

pub(crate) fn get_mock_to_address() -> String {
	format!("{:?}", get_mock_contract_input(1, ethabi::Token::into_address))
}

pub(crate) fn get_mock_amount() -> U256 {
	get_mock_contract_input(2, ethabi::Token::into_uint)
}

pub(crate) fn get_mock_nonce() -> U256 {
	get_mock_contract_input(4, ethabi::Token::into_uint)
}

pub(crate) fn get_mock_timestamp() -> u64 {
	let responses = &*ETHLESS_RESPONSES;

	let timestamp_hex = responses["eth_getBlockByNumber"].result.clone().unwrap()["timestamp"]
		.clone()
		.as_str()
		.unwrap()
		.to_string();
	u64::from_str_radix(timestamp_hex.trim_start_matches("0x"), 16).unwrap()
}

#[extend::ext(name = PendingRequestExt)]
pub(crate) impl Option<PendingRequest> {
	fn set_response(&mut self, response: impl serde::Serialize) {
		self.as_mut().unwrap().response = Some(serde_json::to_vec(&response).unwrap());
	}

	fn set_empty_response(&mut self) {
		self.set_response(JsonRpcResponse::<()> {
			jsonrpc: "2.0".into(),
			id: 1,
			error: None,
			result: None,
		});
	}
}

pub(crate) struct MockedRpcRequests {
	pub(crate) get_transaction: Option<PendingRequest>,
	pub(crate) get_transaction_receipt: Option<PendingRequest>,
	pub(crate) get_block_number: Option<PendingRequest>,
	pub(crate) get_block_by_number: Option<PendingRequest>,
	pub(crate) chain_id: Option<PendingRequest>,
}

impl MockedRpcRequests {
	pub(crate) fn new<'a>(
		rpc_uri: impl Into<Option<&'a str>>,
		tx_hash: &str,
		tx_block_number: &str,
		responses: &HashMap<String, JsonRpcResponse<Value>>,
	) -> Self {
		let uri = rpc_uri.into().unwrap_or("dummy");
		let get_transaction = Some(pending_rpc_request(
			"eth_getTransactionByHash",
			vec![tx_hash.into()],
			uri,
			responses,
		));
		let get_transaction_receipt = Some(pending_rpc_request(
			"eth_getTransactionReceipt",
			vec![tx_hash.into()],
			uri,
			responses,
		));
		let get_block_number = Some(pending_rpc_request("eth_blockNumber", None, uri, responses));
		let get_block_by_number = Some(pending_rpc_request(
			"eth_getBlockByNumber",
			vec![tx_block_number.into(), false.into()],
			uri,
			responses,
		));
		let chain_id = Some(pending_rpc_request("eth_chainId", vec![], uri, responses));
		Self {
			get_transaction,
			get_transaction_receipt,
			get_block_number,
			get_block_by_number,
			chain_id,
		}
	}

	pub(crate) fn mock_chain_id(&mut self, state: &mut OffchainState) -> &mut Self {
		let chain_id = self.chain_id.take().unwrap();
		state.expect_request(chain_id);
		self
	}

	/// Mocks only the RPC response for get_transaction
	pub(crate) fn mock_get_transaction(&mut self, state: &mut OffchainState) {
		let get_transaction = self.get_transaction.take().unwrap();
		state.expect_request(get_transaction);
	}

	/// Mocks the RPC responses up to (inclusive) get_transaction_receipt
	pub(crate) fn mock_get_transaction_receipt(&mut self, state: &mut OffchainState) {
		self.mock_get_transaction(state);
		let get_transaction_receipt = self.get_transaction_receipt.take().unwrap();
		state.expect_request(get_transaction_receipt);
	}

	/// Mocks the RPC responses up to (inclusive) get_block_number
	pub(crate) fn mock_get_block_number(&mut self, state: &mut OffchainState) {
		self.mock_get_transaction_receipt(state);
		let get_block_number = self.get_block_number.take().unwrap();
		state.expect_request(get_block_number);
	}

	/// Mocks the RPC responses up to (inclusive) get_block_by_number
	pub(crate) fn mock_get_block_by_number(&mut self, state: &mut OffchainState) {
		self.mock_get_block_number(state);
		let get_block_by_number = self.get_block_by_number.take().unwrap();
		state.expect_request(get_block_by_number);
	}

	/// Mocks all of the RPC responses
	pub(crate) fn mock_all(mut self, state: &mut OffchainState) {
		self.mock_get_block_by_number(state);
	}
}

#[test]
#[tracing_test::traced_test]
fn offchain_worker_should_log_when_authority_is_missing() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		System::set_block_number(1);

		TaskScheduler::offchain_worker(System::block_number());
		assert!(logs_contain("Not an authority, skipping offchain work"));
	});
}

#[test]
fn default_works() {
	ExtBuilder::default().build_offchain_and_execute(|| {
		let defaults = crate::GenesisConfig::<Test>::default();

		assert_eq!(defaults.authorities.len(), 0);
		assert_eq!(defaults.legacy_wallets.len(), 0);
		assert_eq!(defaults.legacy_balance_keeper, None);
	});
}
