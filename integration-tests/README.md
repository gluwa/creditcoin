IGNORE THIS. TESTING

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
pushd ../creditcoin-js/ && yarn install && popd
yarn install
```

3. Execute the test suite:

```bash
yarn test
```
