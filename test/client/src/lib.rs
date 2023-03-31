use creditcoin_node_runtime::{
	self as runtime, Block, GenesisConfig, SystemConfig as SystemGenesisConfig, WASM_BINARY,
};
use sc_chain_spec::construct_genesis_block;
use sc_service::client;
use sp_core::twox_128;
use sp_runtime::traits::{Block as BlockT, Hash as HashT, Header as HeaderT};
use sp_runtime::Storage;
use std::collections::BTreeMap;

pub type Backend = sc_client_db::Backend<Block>;

/// Test client type with `LocalExecutorDispatch` and generic Backend.
pub type Client<B> = client::Client<
	B,
	client::LocalCallExecutor<Block, B, sc_executor::NativeElseWasmExecutor<LocalExecutorDispatch>>,
	Block,
	runtime::RuntimeApi,
>;

/// A `TestClient` with `test-runtime` builder.
pub type TestClientBuilder<E, B> =
	substrate_test_client::TestClientBuilder<Block, E, B, GenesisParameters>;

type TestClient = Client<Backend>;

/// Creates new client instance used for tests.
pub fn new() -> TestClient {
	TestClientBuilder::with_default_backend().build_with_native_executor(None).0
}

/// A unit struct which implements `NativeExecutionDispatch` feeding in the
/// hard-coded runtime.
pub struct LocalExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for LocalExecutorDispatch {
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		runtime::native_version()
	}
}

/// Parameters of test-client builder with test-runtime.
#[derive(Default)]
pub struct GenesisParameters {
	wasm_code: Option<Vec<u8>>,
}

impl GenesisParameters {
	fn genesis_config(&self) -> GenesisConfig {
		GenesisConfig {
			system: SystemGenesisConfig { code: WASM_BINARY.expect("WASM_BUILD").to_vec() },
			..Default::default()
		}
	}
}

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		use runtime::BuildStorage;
		use sp_runtime::codec::Encode;

		let mut storage = self.genesis_config().build_storage().unwrap();

		if let Some(ref code) = self.wasm_code {
			storage
				.top
				.insert(sp_core::storage::well_known_keys::CODE.to_vec(), code.clone());
		}

		let child_roots = storage.children_default.values().map(|child_content| {
			let state_root = <<<Block as BlockT>::Header as HeaderT>::Hashing as HashT>::trie_root(
				child_content.data.clone().into_iter().collect(),
				sp_runtime::StateVersion::V1,
			);
			let prefixed_storage_key = child_content.child_info.prefixed_storage_key();
			(prefixed_storage_key.into_inner(), state_root.encode())
		});
		let state_root = <<<Block as BlockT>::Header as HeaderT>::Hashing as HashT>::trie_root(
			storage.top.clone().into_iter().chain(child_roots).collect(),
			sp_runtime::StateVersion::V1,
		);
		let block: runtime::Block =
			construct_genesis_block(state_root, sp_runtime::StateVersion::V1);
		storage.top.extend(additional_storage_with_genesis(&block));

		storage
	}
}

fn additional_storage_with_genesis(genesis_block: &Block) -> BTreeMap<Vec<u8>, Vec<u8>> {
	sp_core::map![
		twox_128(&b"latest"[..]).to_vec() => genesis_block.hash().as_fixed_bytes().to_vec()
	]
}
