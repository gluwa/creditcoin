# Architecture and repo layout

## Client / Outer Node

This is the part that handles all interactions with the host machine and outside world.
By extension, the client is the primary point of interaction for node operators - whether that be
through the CLI or by performing RPC calls.
The code for the client lives in the [`node` directory](../../../node).

### CLI

The command line interface you interact with as a user (the `creditcoin-node` binary) uses
[clap](https://docs.rs/clap), similar to many Rust projects.
The actual interface is defined in [`node/src/cli.rs`](../../../node/src/cli.rs). It consists of the
CLI options and subcommands.

The actual parsing and execution of the CLI command occurs in [`command.rs`](../../../node/src/command.rs).
This is the entrypoint to a creditcoin node (`main` just calls into `command::run`)
For most subcommands this means calling the appropriate implementations provided by substrate.

When running a node (as opposed to a subcommand), we call into the `service` module to actual construct
the client.

### Service

This is where we put together all of the pieces of the client - configuring storage, kicking off networking,
setting up the RPC server,
connecting to telemetry, setting up the block import pipeline (and consensus, which is part of the import pipeline), and more.
This is also the entrypoint to mining (we just spawn a bunch of threads which are tasked with mining and submitting results).

This code lives in [`node/src/service.rs`](../../../node/src/service.rs).

### RPC

This is where we define custom RPC methods and extend the standard RPC server with our custom method handlers.
For example, we expose a custom RPC method for retrieving
your node's current hashrate so node operators can monitor their mining performance.

The code for extending the RPC server with custom handlers lives in [`node/src/rpc.rs`](../../../node/src/rpc.rs). Once
you've defined your custom RPC methods and their handlers, you would need to edit this code to register your new handlers.

The code for _defining_ new RPC methods is currently located in [`node/rpc`](../../../node/rpc).

## Runtime

At a high level, the runtime is the state transition function of the blockchain. Roughly you can think of each block
as containing a set of operations to perform (transactions) and a snapshot of the state of the world after those operations
have been carried out. Then, to link a series of blocks together each block has a pointer to its predecessor.
The runtime, then, is what dictates how to actually execute those operations and how the state of the world is
modified in the process. You may also see the runtime and its components referred to as the on-chain logic. On-chain in this context means
an execution environment that can modify the state of the chain, and all modifications to state will be tracked and recorded.

Substrate (which creditcoin builds on) uses the FRAME framework to define independent modules (called _pallets_)
which can be composed to build up the runtime. So, for instance, you might have a [pallet](https://github.com/paritytech/substrate/tree/master/frame/balances)
which maintains balances for a set of accounts and provides operations on those balances (transfer funds, deposit funds, withdrawals, etc.).

The runtime is where we glue together all of the pieces of on-chain logic, and
basically consists of configuring all of the pallets we use and incorporating them into the runtime.

For each pallet, you'll probably have a `Config` trait which allows pallets to be generic over certain types
that can be configured at the runtime level. For instance, here is where we decide that block numbers are
represented by a `u32` and balances by a `u128`. For each pallet the runtime has to implement the `Config` trait.

Some pallets also define runtime APIs. For the most part you can ignore these, but for context runtime APIs are essentially an interface
for the outer node/client to call into the runtime. So, for instance, if you wanted to expose an RPC that needed some information from the runtime,
you would define a runtime API, implement the logic in the runtime, and then call your runtime API from the RPC handler.

Pretty much the entirety of the runtime is in [`lib.rs`](../../../runtime/src/lib.rs). If you need to increase the runtime version
(required after non-backwards-compatible changes to the runtime, i.e. any consensus-affecting change) that's in [`version.rs`](../../../runtime/src/version.rs)

### Pallets