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
For example, we expose a custom RPC method for retrieving
your node's current hashrate so node operators can monitor their mining performance.

The code for extending the RPC server with custom handlers lives in [`node/src/rpc.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/rpc.rs). Once
you've defined your custom RPC methods and their handlers, you would need to edit this code to register your new handlers.

The code for _defining_ new RPC methods is currently located in [`node/rpc`](https://github.com/gluwa/creditcoin/tree/dev/node/rpc).

## Consensus / PoW

The client also contains consensus-related code. Creditcoin uses Proof of Work, which requires block authors
to generate solutions to a problem (mining) and if a "good enough" solution is produced then a block can be authored. The majority of the actual
consensus is implemented in substrate, so the only parts we have to worry about are providing the difficulty, verifying
a given solution, and generating solutions (mining).

### Difficulty

The difficulty is actually determined in runtime logic, so on the client-side we use a runtime API to call into the runtime
logic and get the difficulty for the current block. More specifically, the difficulty adjustment and management occurs in the
difficulty pallet (detailed in the `runtime` section).

### Verifying a Solution

First to clarify, in our case the "problem" miners are solving is the following:

```pseudocode
encode(arg) = SCALE encode arg to bytes
sha3(bytes) = calculate sha3 hash of the given bytes
concat(a, b,...) = concatenate a, b, ...

// H256 is a 256-bit hash type, U256 is a 256-bit unsigned integer type

def do_work(difficulty: U256, pre_hash: H256, nonce: H256) -> H256:
    return sha3(concat(encode(difficulty), encode(pre_hash), encode(nonce)))

def is_solution(work: H256, nonce: H256, difficulty: U256, pre_hash: H256) -> bool:
    calculated = do_work(difficulty)

    // U256.MAX is the maximum value for an unsigned 256-bit integer, i.e.  2^256 - 1

    return work == calculated and U256(work) * difficulty <= U256.MAX

// choose a nonce such that is_solution(do_work(difficulty, pre_hash, nonce), nonce, difficulty, pre_hash) == True
```

Given a proposed solution, we consider it valid if

1. The hash is correct (matches the value obtained by recalculating the hash from input data)
2. The product of the hash and difficulty do not overflow a 256-bit unsigned integer. In other words `hash * difficulty <= 2^256 - 1`

This code lives in the [`sha3pow` crate](https://github.com/gluwa/creditcoin/tree/dev/sha3pow).

### Generating Solutions (Mining)

Mining comes down to essentially picking random nonce values until you find one with the correct properties.
Once we find an appropriate nonce, we submit the solution to a `MiningHandle` which then proceeds with verification and moving forward
with publishing the block. This occurs in [`service.rs`](https://github.com/gluwa/creditcoin/tree/dev/node/src/service.rs) in the `creditcoin-node` crate.
