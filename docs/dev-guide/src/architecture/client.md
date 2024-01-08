# Client / Outer Node

This is the part that handles all interactions with the host machine and outside world.
By extension, the client is the primary point of interaction for node operators - whether that be
through the CLI or by performing RPC calls.
The code for the client lives in the [`node` directory](https://github.com/gluwa/creditcoin/tree/dev/node).

## CLI

The command line interface you interact with as a user (the `creditcoin-node` binary) uses
[clap](https://docs.rs/clap), similar to many Rust projects.
The actual interface is defined in [`node/src/cli.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/cli.rs). It consists of the
CLI options and subcommands.

The actual parsing and execution of the CLI command occurs in [`command.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/command.rs).
This is the entrypoint to a creditcoin node (`main` just calls into `command::run`)
For most subcommands this means calling the appropriate implementations provided by substrate.

When running a node (as opposed to a subcommand), we call into the `service` module to actual construct
the client.

## Service

This is where we put together all of the pieces of the client - configuring storage, kicking off networking,
setting up the RPC server,
connecting to telemetry, setting up the block import pipeline (and consensus, which is part of the import pipeline), and more.
This is also the entrypoint to mining (we just spawn a bunch of threads which are tasked with mining and submitting results).

This code lives in [`node/src/service.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/service.rs).

## RPC

This is where we define custom RPC methods and extend the standard RPC server with our custom method handlers.

The code for extending the RPC server with custom handlers lives in [`node/src/rpc.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/rpc.rs). Once
you've defined your custom RPC methods and their handlers, you would need to edit this code to register your new handlers.

The code for _defining_ new RPC methods is currently located in [`node/rpc`](https://github.com/gluwa/creditcoin/tree/dev/node/rpc).

## Consensus / NPoS

The client also contains consensus-related code. 

Creditcoin uses Nominated Proof of Stake (NPoS), a variation of the Proof of Stake (PoS) consensus algorithm. NPoS introduces a nominator-validator model where nominators can select and back validators. Nominators delegate their stake to validators, and in return, they can receive a portion of the validator's rewards. This system allows token holders, who may not have enough stake or technical expertise to become a validator, to nonetheless participate in the network and earn rewards by supporting trusted validators instead. Thereby, it promotes a more inclusive and secure network, as the nomination process enables a broader set of token holders to participate in consensus and governance than under a traditional PoS model.

Read more about the NPoS system in the [Creditcoin Docs](https://docs.creditcoin.org/staking).
