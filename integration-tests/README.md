# Integration tests for Creditcoin Substrate node

## General setup

```bash
sudo apt-get install nodejs npm
sudo npm install -g yarn
yarn install --immutable
```

**WARNING:** Node.js 14.x || 16.x is required


## Getting Started

1. Build the software under test, see **Single-Node Development Chain** in `../README.md`
   and execute it locally:

```bash
./target/debug/creditcoin-node --dev --mining-key XYZ
```

2. Prepare the local development environment:

```bash
./prepare-devel-env.sh
```

3. Execute this test suite:

```bash
yarn test
```
