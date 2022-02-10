use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::{self as pallet_creditcoin, ocw::rpc::JsonRpcRequest};
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64, GenesisBuild, Hooks},
};
use frame_system as system;
use sp_core::H256;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
	offchain::{
		testing::{
			OffchainState, PendingRequest, PoolState, TestOffchainExt, TestTransactionPoolExt,
		},
		OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
	},
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature, RuntimeAppPublic,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
pub type Signature = MultiSignature;
pub type Extrinsic = TestXt<Call, ()>;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type BlockNumber = u64;

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
		Timestamp: pallet_timestamp::{Pallet, Call, Storage}
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const PendingTxLimit: u32 = 10000;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
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
	type Moment = u64;

	type OnTimestampSet = ();

	type MinimumPeriod = ConstU64<1>;

	type WeightInfo = ();
}

impl pallet_creditcoin::Config for Test {
	type Event = Event;

	type Call = Call;

	type AuthorityId = pallet_creditcoin::crypto::CtcAuthId;

	type Signer = <Signature as Verify>::Signer;
	type FromAccountId = AccountId;

	type InternalPublic = sp_core::sr25519::Public;

	type PublicSigning = <Signature as Verify>::Signer;

	type UnverifiedTransferLimit = PendingTxLimit;
}

impl system::offchain::CreateSignedTransaction<pallet_creditcoin::Call<Test>> for Test {
	fn create_transaction<C: system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Self::OverarchingCall,
		_public: Self::Public,
		_account: Self::AccountId,
		nonce: Self::Index,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
where
	Call: From<C>,
{
	type OverarchingCall = Call;
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
	type Event = Event;
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
	keystore: Option<KeyStore>,
}

impl ExtBuilder {
	#[allow(dead_code)]
	pub fn fund(&mut self, account: AccountId, amount: Balance) -> &mut ExtBuilder {
		self.balances.push((account, amount));
		self
	}
	pub fn generate_authority(&mut self) -> &mut ExtBuilder {
		const PHRASE: &str =
			"news slush supreme milk chapter athlete soap sausage put clutch what kitten";
		if let None = self.keystore {
			self.keystore = Some(KeyStore::new());
		}
		let pubkey = self
			.keystore
			.as_ref()
			.unwrap()
			.sr25519_generate_new(
				crate::crypto::Public::ID,
				Some(&format!("{}/auth{}", PHRASE, self.authorities.len() + 1)),
			)
			.unwrap();
		self.authorities.push(AccountId::new(pubkey.into_account().0));
		self
	}
	pub fn build(self) -> sp_io::TestExternalities {
		sp_tracing::try_init_simple();
		let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();

		let _ = pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut storage);

		let _ = crate::GenesisConfig::<Test> { authorities: self.authorities }
			.assimilate_storage(&mut storage);

		storage.into()
	}
	pub fn build_offchain(
		mut self,
	) -> (sp_io::TestExternalities, Arc<RwLock<OffchainState>>, Arc<RwLock<PoolState>>) {
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
}

#[allow(dead_code)]
pub fn roll_to(n: BlockNumber) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		Creditcoin::on_initialize(i);
		Creditcoin::on_finalize(i);
	}
}

#[allow(dead_code)]
pub fn roll_to_with_ocw(n: BlockNumber) {
	let now = System::block_number();
	for i in now + 1..=n {
		System::set_block_number(i);
		Creditcoin::on_initialize(i);
		Creditcoin::offchain_worker(i);
		Creditcoin::on_finalize(i);
	}
}

#[allow(dead_code)]
pub fn roll_by_with_ocw(n: BlockNumber) {
	let mut now = System::block_number();
	for _ in 0..n {
		if now == 0 {
			Creditcoin::offchain_worker(now);
		}
		now += 1;
		System::set_block_number(now);
		Creditcoin::on_initialize(now);
		Creditcoin::offchain_worker(now);
		Creditcoin::on_finalize(now);
	}
}

pub fn pending_rpc_request(
	method: &str,
	params: impl IntoIterator<Item = serde_json::Value>,
	uri: &str,
	responses: &HashMap<String, serde_json::Value>,
) -> PendingRequest {
	let rpc = JsonRpcRequest::new(method, params).to_bytes();
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
