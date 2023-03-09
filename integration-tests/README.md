# General setup

```bash
sudo apt-get install nodejs npm
sudo npm install -g yarn
```

**WARNING:** Node.js 14.x || 16.x is required

## Getting Started

0. Start the software under test, see **Single-Node Development Chain** in `../README.md`
   for more information:

```bash
cargo run --release -- --dev --mining-key 5DkPYq8hFiCeGxFBkz6DAwnTrvKevAJfTYrzFtr9hpDsEAU1 --monitor-nonce auto
```

1. Execute a local Ethereum node:

```bash
docker run --rm -it -p 8545:8545 gluwa/hardhat-dev
```

2. Install dependencies:

```bash
pushd ../creditcoin-js/ && yarn install && yarn pack && popd
yarn install
```

3. Execute the test suite:

```bash
yarn test
```

## Testing against Testnet

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

yarn test --config testnet.config.ts
```

`LENDER_PRIVATE_KEY` and `BORROWER_PRIVATE_KEY` are Ethereum private keys
which hold 0.1 ETH!

**WARNING:**
when running this test-suite against `testnet` make sure that the
source code matches the version of testnet running on the nodes. Try
branching the tests from the relavant branch/tag before executing them.

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
