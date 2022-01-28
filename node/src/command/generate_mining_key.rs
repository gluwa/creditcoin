use std::path::PathBuf;

use bip39::{Language, Mnemonic, MnemonicType};
use sc_cli::{
	structopt, utils::print_from_uri, with_crypto_scheme, KeystoreParams, OutputTypeFlag,
	SubstrateCli,
};
use sc_keystore::LocalKeystore;
use sc_service::{config::KeystoreConfig, Arc, BasePath};
use sp_core::{
	crypto::{ExposeSecret, KeyTypeId, SecretString, Ss58Codec},
	Pair,
};
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::traits::IdentifyAccount;

use crate::cli::Cli;

#[derive(Debug, structopt::StructOpt)]
pub struct MiningKeySubcommand {
	/// The number of words in the phrase to generate. One of 12 (default), 15, 18, 21 and 24.
	#[structopt(long, short = "w", value_name = "WORDS")]
	words: Option<usize>,

	#[structopt(flatten)]
	pub output_scheme: OutputTypeFlag,

	#[structopt(flatten)]
	pub keystore_params: KeystoreParams,

	/// Specify the chain specification.
	///
	/// It can be one of the predefined ones (dev, local, or staging) or it can be a path to a file
	/// with the chainspec (such as one exported by the `build-spec` subcommand).
	#[structopt(long, value_name = "CHAIN_SPEC")]
	pub chain: Option<String>,

	/// Specify the development chain.
	///
	/// This flag sets `--chain=dev`, `--force-authoring`, `--rpc-cors=all`,
	/// `--alice`, and `--tmp` flags, unless explicitly overridden.
	#[structopt(long, conflicts_with_all = &["chain"])]
	pub dev: bool,

	/// Specify custom base path.
	#[structopt(long, short = "d", value_name = "PATH", parse(from_os_str))]
	pub base_path: Option<PathBuf>,

	/// Output only the public address of the generated key
	#[structopt(long, short = "q")]
	pub quiet: bool,

	/// Only generate the mining key, don't insert it into the keystore
	#[structopt(long)]
	pub no_insert: bool,
}

fn to_vec<P: sp_core::Pair>(
	uri: &str,
	pass: Option<SecretString>,
) -> Result<Vec<u8>, sc_cli::Error> {
	let p = sc_cli::utils::pair_from_suri::<P>(uri, pass)?;
	Ok(p.public().as_ref().to_vec())
}

impl MiningKeySubcommand {
	fn chain_id(&self) -> String {
		match self.chain {
			Some(ref chain) => chain.clone(),
			None =>
				if self.dev {
					"dev".into()
				} else {
					"".into()
				},
		}
	}
	pub fn run(&self, cli: &Cli) -> Result<(), sc_cli::Error> {
		let words = match self.words {
			Some(words) => MnemonicType::for_word_count(words).map_err(|_| {
				sc_cli::Error::Input(
					"Invalid number of words given for phrase: must be 12/15/18/21/24".into(),
				)
			})?,
			None => MnemonicType::Words12,
		};
		let mnemonic = Mnemonic::new(words, Language::English);
		let password = self.keystore_params.read_password()?;
		let output = self.output_scheme.output_type.clone();
		let uri = mnemonic.phrase();

		let pwd = password.as_ref().map(|s| s.expose_secret().as_str());
		if self.quiet {
			let (pair, _) = sp_core::ecdsa::Pair::from_phrase(uri, pwd)
				.expect("we just generated the valid phrase; qed");
			let public_address = pair.public().into_account().to_ss58check();
			println!("{}", public_address);
		} else {
			with_crypto_scheme!(
				sc_cli::CryptoScheme::Ecdsa,
				print_from_uri(uri, password, None, output)
			);
		}

		if self.no_insert {
			let suri = sc_cli::utils::read_uri(Some(&uri.into()))?;
			let base_path = self
				.base_path
				.clone()
				.map(Into::into)
				.unwrap_or_else(|| BasePath::from_project("", "", &Cli::executable_name()));
			let chain_id = self.chain_id();
			let chain_spec = cli.load_spec(&chain_id)?;
			let config_dir = base_path.config_dir(chain_spec.id());

			let (keystore, public) = match self.keystore_params.keystore_config(&config_dir)? {
				(_, KeystoreConfig::Path { path, password }) => {
					let public = with_crypto_scheme!(
						sc_cli::CryptoScheme::Ecdsa,
						to_vec(&suri, password.clone())
					)?;
					let keystore: SyncCryptoStorePtr =
						Arc::new(LocalKeystore::open(path, password)?);
					(keystore, public)
				},
				_ => unreachable!("keystore_config always returns path and password; qed"),
			};

			let key_type = KeyTypeId::from(sha3pow::app::ID);

			SyncCryptoStore::insert_unknown(&*keystore, key_type, &suri, &public[..])
				.map_err(|_| sc_cli::Error::KeyStoreOperation)?;
		}

		Ok(())
	}
}
