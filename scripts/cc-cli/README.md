# Creditcoin CLI Tool

## Building

This tool depends on `creditcoin-js`.
Make sure to pack the library using `yarn pack` in the `creditcoin-js` folder before building.
It should be available as `creditcoin-js-vX.Y.Z.tgz`.

Build using yarn.

```bash
yarn install
yarn build
```

Install globally using npm.

```bash
npm install -g .
```

Or use with node from the project directory.

```bash
node dist/index.js
```

## Commands

- **balance**:            Get balance of an account
- **bond**:               Bond CTC from a Stash account
- **chill**:              Signal intention to stop validating from a Controller account
- **collect-coins**:      Swap GCRE for CTC
- **distribute-rewards**: Distribute all pending rewards for all validators
- **new**:                Create new seed phrase
- **register-address**:   Link a CreditCoin address to an address from another blockchain
- **rotate-keys**:        Rotate session keys for a specified node
- **send**:               Send CTC from an account
- **set-keys**:           Set session keys for a Controller account
- **show-address**:       Show account address
- **status**:             Get staking status for an address
- **unbond**:             Schedule a portion of the stash to be unlocked
- **validate**:           Signal intention to validate from a Controller account
- **withdraw-unbonded**:  Withdraw unbonded funds from a stash account
- **wizard**:             Run the validator setup wizard. Only requires funded stash and controller accounts.

To view all commands run the tool with the `--help` flag.

## Examples

### Running from the Creditcoin Docker container

From the root of the Creditcoin repository, build and run the image.

```bash
docker build -t creditcoin-node .
docker run --name creditcoin creditcoin-node
```

Execute the CLI tool with the `exec` Docker command like so:

```bash
docker exec creditcoin creditcoin-cli --help
docker exec creditcoin creditcoin-cli new
```

### Create a new seed

```bash
creditcoin-cli new-seed
```

### Create a new seed and save it to a file

```bash
creditcoin-cli new-seed --file seed.txt
```

### Show address for a particular account

```bash
creditcoin-cli receive -f seed.txt
```

### Check the balance of an account

```bash
creditcoin-cli balance -f seed.txt
```

### Bond CTC tokens using a stash account

```bash
creditcoin-cli bond -f seed.txt --amount 1000 --controller 5DJ8qkxAbSVfyvorNBKt4BwDR9hUUzH8aqofuTAMTkLZtpv9
```

### Rotate session keys of a particular node

```bash
creditcoin-cli rotate-keys -u http://localhost:8000
```

### Run the validator setup wizard
This example asumes seeds are saved in `stashseed` and `controllerseed` files and a node is running on `ws://localhost:9944`.

```bash
creditcoin-cli wizard -sf stashseed -cf controllerseed -a 1000 -u ws://localhost:9944
```

⚠️🔧 Warning: This tool is currently under development! 🔧⚠️

Please be aware that the command line tool you are using is still in active development.
It may contain bugs, incomplete features, or unexpected behavior.
Exercise caution and use it at your own risk.
Feedback and bug reports are greatly appreciated to help improve the tool and ensure its stability.

## Local testing for NPoS

You need to have the following:
- `gluwa/hardhat-dev` container to simulate Ethereum
- `creditcoin-node`

    cargo build --release --features fast-runtime
    ./target/release/creditcoin-node --chain local --validator --alice --node-key d182d503b7dd97e7c055f33438c7717145840fd66b2a055284ee8d768241a463 -lrpc=info --enable-log-reloading --pruning archive --base-path ./demo --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all

- Subscan database & Subscan API containers running
- Creditcoin Staking Dashboard running
- Account 0: Private Key from hardhat

Execute like so:

    $ node dist/index.js collect-coins -k 0xAccount0privateKey --seed //Alice --debug
