//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

use mimalloc_rust::GlobalMiMalloc;

#[global_allocator]
static ALLOCATOR: GlobalMiMalloc = GlobalMiMalloc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
