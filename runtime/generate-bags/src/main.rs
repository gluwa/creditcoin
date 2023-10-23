use std::path::PathBuf;

use clap::Parser;
use generate_bags::generate_thresholds;

#[derive(Parser)]
struct Cli {
	/// Total issuance of the currency in millions
	#[clap(short, long)]
	total_issuance: u128,

	/// Minimum account balance
	#[clap(short, long, default_value_t = 8456776)]
	minimum_balance: u128,

	#[clap(long, default_value_t = 200)]
	n_bags: usize,

	#[clap(long)]
	output: PathBuf,
}

/*
pub struct U128CurrencyToVote;

impl U128CurrencyToVote {
	fn factor(issuance: u128) -> u128 {
		(issuance / u64::MAX as u128).max(1)
	}
}

impl CurrencyToVote<u128> for U128CurrencyToVote {
	fn to_vote(value: u128, issuance: u128) -> u64 {
		(value / Self::factor(issuance)).saturated_into()
	}

	fn to_currency(value: u128, issuance: u128) -> u128 {
		value.saturating_mul(Self::factor(issuance))
	}
} */

fn main() -> Result<(), std::io::Error> {
	let Cli { total_issuance, minimum_balance, n_bags, output } = Cli::parse();

	let issuance_ctc = total_issuance * creditcoin_node_runtime::CTC * 1_000_000;

	println!("Issuance ctc = {issuance_ctc}; factor = {}", (issuance_ctc / u64::MAX as u128));

	generate_thresholds::<creditcoin_node_runtime::Runtime>(
		n_bags,
		&output,
		issuance_ctc,
		minimum_balance,
	)
}
