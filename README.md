# Gluwa Creditcoin

## What is Creditcoin?

Creditcoin is a network that enables cross-blockchain credit transaction and credit history building. Creditcoin uses blockchain technology to ensure the objectivity of its credit transaction history: each transaction on the network is distributed and verified by the network.

The Creditcoin protocol was created by Gluwa. Gluwa Creditcoin is the official implementation of the Creditcoin protocol by Gluwa.

For more information, see [creditcoin.org](https://creditcoin.org), or read the [original whitepaper](https://creditcoin.org/white-paper).

## Getting Started

- [Miner Setup](./docs/miner-setup.md)
- [Legacy Account Migration](./docs/legacy-account-migration.md)
- [Using PolkadotJs](./docs/using-polkadotjs.md)

## Developer Setup

### Rust Setup

First, you must complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

```sh
cargo run --release -- --dev --tmp --mining-key <your mining key>
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Explore Node Options

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/creditcoin-node -h
```

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/creditcoin-node --dev --mining-key <your mining key>
```

Purge the development chain's state:

```bash
./target/release/creditcoin-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/creditcoin-node -ldebug --dev --mining-key <your mining key>
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).
[Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Links Regarding legacy Creditcoin implementation

- [Legacy Mining Setup](https://docs.creditcoin.org/creditcoin-miners-manual/pre-2.0-mining-setup)
- [Legacy Creditcoin Repos](https://github.com/gluwa?q=legacy)
