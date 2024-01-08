# Gluwa Creditcoin

## What is Creditcoin?

Creditcoin is a network that enables cross-blockchain credit transaction and credit history building. Creditcoin uses blockchain technology to ensure the objectivity of its credit transaction history: each transaction on the network is distributed and verified by the network.

The Creditcoin protocol was created by Gluwa. Gluwa Creditcoin is the official implementation of the Creditcoin protocol by Gluwa.

For more information, see [creditcoin.org](https://creditcoin.org), or read the [original whitepaper](https://creditcoin.org/white-paper).

## Getting Started

- [Miner Setup](./docs/miner-setup.md)
- [Using PolkadotJs](./docs/using-polkadotjs.md)

## Developer Setup

### Dependency Setup

First, you must complete the
[basic environment setup](https://github.com/gluwa/creditcoin/blob/dev/docs/dev-guide/src/getting-started/building.md#build-prerequisites).

### Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

```sh
cargo run --release -- --dev --tmp
```

### Explore Node Options

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/creditcoin-node -h
```

### Connecting to Creditcoin Networks

A node can be configured to connect to different Creditcoin networks. Each network has different configurations and use cases.

|               | Devnet                                        | Testnet                                       | Mainnet                           |
|---------------|-----------------------------------------------|-----------------------------------------------|-----------------------------------|
| Overview      | Local/public development environment          | Public testing environment                    | Live production environment       |
| Users         | Developers                                    | Developers & testers                          | End users                         |
| Function      | To develop new features & improvements        | To test new features & improvements           | To secure credit history on-chain |
| Tokens        | Test tokens with no real world economic value | Test tokens with no real world economic value | Real tokens with economic value   |
| Chain history | Wiped frequently                              | Wiped occasionally                            | Preserved                         |

The network configuration is specified using the `--chain` flag and the `--bootnodes` flag, which specifies the initial nodes to connect to. Currently, only the `test` network chain specs include bootnodes. The `main` and `dev` networks bootnodes must be specified manually.

Example:

```bash
./target/release/creditcoin-node --chain main --bootnodes "/dns4/bootnode.creditcoin.network/tcp/30333/p2p/12D3KooWAEgDL126EUFxFfdQKiUhmx3BJPdszQHu9PsYsLCuavhb"
```

#### ChainSpecs

Creditcoin networks are configured using a `ChainSpec`. The `ChainSpec` is a JSON file that defines the initial configuration of the network. To use a `ChainSpec`, use the `--chain` flag when starting the node.

- Mainnet: `--chain main`
- Testnet: `--chain test`
- Devnet: `--chain dev`

#### Bootnodes

Bootnodes are nodes that are always on and can be used to bootstrap new nodes and discover other nodes in the network. To use a bootnode, use the `--bootnodes` flag when starting the node followed by the bootnode's address.

Mainnet bootnodes:

- `/dns4/bootnode.creditcoin.network/tcp/30333/p2p/12D3KooWAEgDL126EUFxFfdQKiUhmx3BJPdszQHu9PsYsLCuavhb`
- `/dns4/bootnode2.creditcoin.network/tcp/30333/p2p/12D3KooWSQye3uN3bZQRRC4oZbpiAZXkP2o5UZh6S8pqyh24bF3k`
- `/dns4/bootnode3.creditcoin.network/tcp/30333/p2p/12D3KooWFrsEZ2aSfiigAxs6ir2kU6en4BewotyCXPhrJ7T1AzjN`

Testnet bootnodes:

- `/dns4/testnet-bootnode.creditcoin.network/tcp/30333/p2p/12D3KooWG3eEuYxo37LvU1g6SSESu4i9TQ8FrZmJcjvdys7eA3cH`
- `/dns4/testnet-bootnode2.creditcoin.network/tcp/30333/p2p/12D3KooWLq7wCMQS3qVMCNJ2Zm6rYuYh74cM99i9Tm8PMdqJPDzb`
- `/dns4/testnet-bootnode3.creditcoin.network/tcp/30333/p2p/12D3KooWAKUrvmchoLomoouoN1sKfF9kq8dYtCVFvtPuvqp7wFBS`

Devnet bootnodes:

- `/dns4/devnet-bootnode.creditcoin.network/tcp/30333/p2p/12D3KooWMtJz2E3ENY66Sfoa1MDmV3ZATXRKUWdeZgtEjfme6iwS`

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/creditcoin-node --dev
```

Purge the development chain's state:

```bash
./target/release/creditcoin-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/creditcoin-node -ldebug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Links Regarding legacy Creditcoin 1.x implementation

- [Legacy Creditcoin 1.x Account Migration](./docs/legacy-account-migration.md)
- [Legacy Creditcoin 1.x Repos](https://github.com/gluwa?q=legacy)

### Testing
