# General setup

```bash
sudo apt-get install nodejs npm
sudo npm install -g yarn
```

**WARNING:** Node.js 14.x || 16.x is required

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
export LENDER_PRIVATE_KEY=XXXX
export BORROWER_PRIVATE_KEY=YYYY

yarn test --config testnet.config.ts
```

**WARNING:**
when running this test-suite against `testnet` or `mainnet` make sure that the
source code matches the version of testnet/mainnet running on the nodes. Try
branching the tests from the relavant branch/tag before executing them.

Running a test-suite from `dev` against downstream branches is not supported and
will generally fail. The most likely failures will be missing extrinsics, different
parameters to extrinsics and/or mismatched TypeScript type definitions which are
automatically generated from the running version of `creditcoin-node`.
