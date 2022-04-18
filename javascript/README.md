# General setup

```bash
sudo apt-get install nodejs npm
sudo npm install -g yarn
```

**WARNING:** Node.js 14.x || 16.x is required

# Creditcoin PolkdotJs Examples

## Getting Started

This project contains examples of how to interact with Creditcoin using PolkadotJs. To execute the examples run

```bash
yarn install
yarn start
```

See the `./src/examples/` directory for more information.

# Integration tests for Creditcoin Substrate node

The directory `integration-tests/` contains tests used by the Creditcoin team. These tests also
reuse the type definitions and some helper functions provided by the examples!

## Getting Started

0. Build the software under test, see **Single-Node Development Chain** in `../README.md`
   and execute it locally:

```bash
./target/release/creditcoin-node --dev --mining-key XYZ
```

1. Execute a local Ethereum node:

```bash
docker run --rm -it -p 8545:8545 gluwa/hardhat-dev
```

2. Prepare the local development environment:

```bash
./prepare-devel-env.sh
```

**NOTE:** this repository comes with type definitions in `./src/interfaces/*.ts` to make
it easier to execute the examples. The above command waits for a local creditcoin-node to
start accepting connections and will regenerate type definitions from the blockchain metadata.

3. Execute the test suite:

```bash
yarn test
```
