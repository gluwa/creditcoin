use sc_cli::RunCmd;
use structopt::StructOpt;

fn parse_rpc_pair(input: &str) -> Result<(String, String), String> {
	let (name, uri) = input
		.split_once('=')
		.ok_or_else(|| String::from("expected a key-value pair separated by '='"))?;
	let unquote = |s: &str| {
		if s.starts_with('\'') && s.ends_with('\'') {
			Ok(s.trim_matches('\'').into())
		} else if s.starts_with('\"') && s.ends_with('\"') {
			Ok(s.trim_matches('\"').into())
		} else if !s.starts_with(&['\'', '\"']) {
			Ok(s.into())
		} else {
			Err(String::from("invalid quotes in rpc mapping"))
		}
	};

	let name = unquote(name.trim())?;
	let uri = unquote(uri.trim())?;
	Ok((name, uri))
}

mod parse_tests {
	#[test]
	fn parse_rpc_pair_quoted() {
		assert_eq!(
			super::parse_rpc_pair(r#""ethereum"="https://mainnet.infura.io/thingwith=foo""#),
			Ok(("ethereum".into(), "https://mainnet.infura.io/thingwith=foo".into()))
		);
		assert_eq!(
			super::parse_rpc_pair(r#"'ethereum'='https://mainnet.infura.io/thingwith=foo'"#),
			Ok(("ethereum".into(), "https://mainnet.infura.io/thingwith=foo".into()))
		)
	}
}

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,

	#[structopt(long)]
	/// The public key or SS58 Address of the account to receive mining rewards in.
	pub mining_key: Option<String>,

	#[structopt(long)]
	/// The number of mining worker threads to spawn. Defaults to the number of cores if omitted.
	pub mining_threads: Option<usize>,

	#[structopt(long, parse(try_from_str = parse_rpc_pair))]
	/// If the node is an oracle authority, the RPC URL to use for a given external chain.
	pub rpc_mapping: Option<Vec<(String, String)>>,

	#[structopt(long)]
	pub monitor_nonce: Option<String>,
}
#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
