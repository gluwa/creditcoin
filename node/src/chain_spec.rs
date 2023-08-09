use creditcoin_node_runtime::{
	pallet_staking_substrate, AccountId, BabeConfig, BalancesConfig, CreditcoinConfig,
	DifficultyConfig, GenesisConfig, ImOnlineId, Perbill, PosSwitchConfig, SessionConfig,
	Signature, StakingConfig, SudoConfig, SystemConfig, TaskSchedulerConfig,
	TransactionPaymentConfig, CTC, WASM_BINARY,
};

use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public, U256};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	FixedU128,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

#[derive(Clone, Debug, Deserialize, Serialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<creditcoin_node_runtime::Block>,
	/// The public keys for the authorities of the initial round of GRANDPA.
	pub grandpa_initial_authorities: Option<Vec<GrandpaId>>,
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

type AuthorityKeys = (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId);

pub fn get_authority_keys_from_seed(seed: &str) -> AuthorityKeys {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

fn make_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
) -> creditcoin_node_runtime::SessionKeys {
	creditcoin_node_runtime::SessionKeys { grandpa, babe, im_online }
}

fn chain_properties() -> serde_json::Map<String, serde_json::Value> {
	match serde_json::json! ({
		"tokenDecimals": 18,
		"tokenSymbol": "CTC",
	}) {
		serde_json::Value::Object(o) => o,
		_ => unreachable!(),
	}
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities = vec![get_authority_keys_from_seed("Alice")];

	let extensions = Extensions {
		grandpa_initial_authorities: Some(
			initial_authorities.clone().into_iter().map(|x| x.2).collect(),
		),
		bad_blocks: None,
	};

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial authorities
				initial_authorities.clone(),
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				Some(6000),
				Some(128),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(chain_properties()),
		// Extensions
		extensions,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let initial_authorities = vec![get_authority_keys_from_seed("Alice")];

	let extensions = Extensions {
		grandpa_initial_authorities: Some(
			initial_authorities.clone().into_iter().map(|x| x.2).collect(),
		),
		bad_blocks: None,
	};

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial authorities
				initial_authorities.clone(),
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				None,
				None,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(chain_properties()),
		// Extensions
		extensions,
	))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../chainspecs/posTestnetSpec.json")[..])
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../chainspecs/mainnetSpec.json")[..])
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<AuthorityKeys>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	target_time: Option<u64>,
	adjustment: Option<i64>,
) -> GenesisConfig {
	const ENDOWMENT: u128 = 1_000_000 * CTC;
	const STASH: u128 = 1_000_000 * CTC;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of ENDOWMENT.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		difficulty: DifficultyConfig {
			initial_difficulty: U256::from(1_000_000u64),
			target_time: target_time.unwrap_or(60 * 1000),
			difficulty_adjustment_period: adjustment.unwrap_or(43),
		},
		creditcoin: CreditcoinConfig::default(),
		transaction_payment: TransactionPaymentConfig { multiplier: FixedU128::from_float(1.0) },
		task_scheduler: TaskSchedulerConfig::default(),
		pos_switch: PosSwitchConfig { switch_block_number: Some(0) },
		grandpa: Default::default(),
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(creditcoin_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						STASH,
						pallet_staking_substrate::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: pallet_staking_substrate::Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|(stash, _acct, grandpa, babe, im_online)| {
					(stash.clone(), stash, make_session_keys(grandpa, babe, im_online))
				})
				.collect(),
		},
		im_online: Default::default(),
		nomination_pools: Default::default(),
	}
}
