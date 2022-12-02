use crate::RUNTIME_API_VERSIONS;
use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("creditcoin-node"),
	impl_name: create_runtime_str!("creditcoin-node"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 212,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 10,
	state_version: 1,
};
