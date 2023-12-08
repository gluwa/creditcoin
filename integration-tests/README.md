# General setup

Install Node.js and Yarn. You can do so from here: <https://nodejs.org/en/download>

**WARNING:** Node.js 14.x || 16.x is required

Alternatively, a convenient tool to manage Node/npm installations is `nvm`.
You can get it here: <https://github.com/nvm-sh/nvm#installing-and-updating>

And then install the required version as follows.

```bash
nvm install 16
nvm alias default 16
nvm use 16
```

We use Yarn for our package management and build scripts. To install Yarn enter:
```bash
npm install -g yarn
```

This test suite is built with creditcoin-js, Polkadot.js and Jest!

## Getting Started

0. In the root directory of this repository, start the software under test, see **Single-Node Development Chain** in `../README.md`
   for more information:

```bash
cargo run --release --bin creditcoin-node -- --dev --monitor-nonce auto
```

1. Execute a local Ethereum node:

```bash
docker run --rm -it -p 8545:8545 gluwa/hardhat-dev
```

2. Install dependencies for creditcoin-js, and update its type definitions:

```bash

    pushd creditcoin-js/

    yarn install
    ./get-metadata.sh
    yarn build:types
    yarn format

    popd
```

**Important:**

These are updated automatically in CI. If you are not sure that your
branch is up-to-date or if you are modifying Creditcoin types then execute
this step!

**Warning:** don't forget to commit changed definitions to git!

3. From the directory of this README.md (`integration-tests`), install dependencies and execute the test node:

```bash
./yarn-install-and-wait-for-creditcoin.sh
yarn test
```

To execute individual tests use something like:

```bash
yarn test src/test/collect-coins.test.ts
```

To produce a more verbose test output and/or specify other command line
options replace `test` with `jest`. For example:

```bash
yarn jest src/test/collect-coins.test.ts
```

You can use `CREDITCOIN_WS_PORT` and `CREDITCOIN_METRICS_PORT` environment variables
to adjust the port values when running this test suite against a local node. That's mainly
useful for executing the loan cycle against a Zombienet chain running locally.


## Testing against Testnet

1. Install dependencies:

```bash
pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn install
```

2. Execute the test suite:

```bash
export ETHEREUM_NODE_URL=https://goerli.infura.io/v3/abcdef
export LENDER_PRIVATE_KEY=XXXXXX
export BORROWER_PRIVATE_KEY=YYYY

yarn test --config testnet.config.ts
```

`LENDER_PRIVATE_KEY` and `BORROWER_PRIVATE_KEY` are Ethereum private keys
which hold 0.1 ETH!

**WARNING:**
when running this test-suite against `testnet` make sure that the
source code matches the version of testnet running on the nodes. Try
branching the tests from the relevant branch/tag before executing them.

Running a test-suite from `dev` against downstream branches is not supported and
will generally fail. The most likely failures will be missing extrinsics, different
parameters to extrinsics and/or mismatched TypeScript type definitions which are
automatically generated from the running version of `creditcoin-node`.


## Testing against Mainnet

1. Install dependencies:

```bash
pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn install
```

2. Execute the test suite:

```bash
export ETHEREUM_NODE_URL=https://rinkeby.infura.io/v3/abcdef
export LENDER_PRIVATE_KEY=XXXXXX
export BORROWER_PRIVATE_KEY=YYYY
export LENDER_SEED=AAAAAAAAA
export BORROWER_SEED=BBBBBBB
export SUDO_SEED=CCCCCCCCCCC

yarn test --config testnet.config.ts
```

`LENDER_PRIVATE_KEY` and `BORROWER_PRIVATE_KEY` are Ethereum private keys
which hold 0.1 ETH!

`LENDER_SEED`, `BORROWER_SEED` and `SUDO_SEED` are secret seeds/private keys
for funded Creditcoin accounts which will be used for testing!

**WARNING:**
when running this test-suite against `mainnet` make sure that the
source code matches the version of mainnet running on the nodes. Try
branching the tests from the relavant branch/tag before executing them.

Running a test-suite from `dev` against downstream branches is not supported and
will generally fail. The most likely failures will be missing extrinsics, different
parameters to extrinsics and/or mismatched TypeScript type definitions which are
automatically generated from the running version of `creditcoin-node`.
