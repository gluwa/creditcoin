# Reproduction

## Reproducing the bug

All commands from the root of the repo.

1. Build node

    ```bash
    cargo build --release --features fast-runtime
    ```

2. Import blocks for node1

    ```bash
    ./target/release/creditcoin-node import-blocks --chain ./integration-tests/csub-552/reproSpec.json --base-path ./integration-tests/csub-552/chaindata/node1 --state-pruning archive --blocks-pruning archive --database paritydb ./integration-tests/csub-552/repro.blocks
    ```

3. Run node1

    ```bash
    ./target/release/creditcoin-node --chain integration-tests/csub-552/reproSpec.json --validator --alice --mining-threads 1 --mining-key 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --pruning archive --node-key 844794b5f1da387970d769debc3a30f729f3515841121ccecebed2582723e04d --base-path ./integration-tests/csub-552/chaindata/node1
    ```

4. Initiate PoS switchover by calling the `posSwitch.switchToPos` extrinsic (through polkadot-js or whatever else)

5. Wait until all of the blocks are `finalized` (i.e. wait until the `finalized` head is >= 1070 or so)

6. Sync node2 with node1. In a separate terminal:

    ```bash
    ./target/release/creditcoin-node --chain integration-tests/csub-552/reproSpec.json --validator --mining-threads 1 --base-path integration-tests/csub-552/chaindata/node2 --rpc-cors=all --mining-key 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSrv5vZAh2Xr97BpyHLZ8scGqcaPr8CLYvm4EVtLwKiXy' --port 30334 --ws-port 9945 --rpc-port 9934 --pruning archive
    ```

7. 💥

## How `reproSpec.json` and `repro.blocks` were created

So there's no magic, here's how I created the chainspec and blocks.

### Chainspec (`reproSpec.json`)

1. Copy `devSpec.json`

    ```bash
    cp chainspecs/devSpec.json integration-tests/csub-552/reproSpec.json
    ```

2. Change chain name ID, and add some random genesis data
    The point of this is just to ensure that our chain has a different genesis hash from the dev network.
    Here is the diff I applied:

    ```diff
    2,4c2,4
    <   "name": "Creditcoin Dev",
    <   "id": "creditcoin_dev",
    <   "chainType": "Live",
    ---
    >   "name": "Creditcoin PoS Switch Panic Repro",
    >   "id": "creditcoin_pos_panic_repro",
    >   "chainType": "Local",
    17a18
    >         "0xfafa": "0xfafa",
    ```

### Blocks (`repro.blocks`)

1. Start node on `reproSpec.json`

    ```bash
    ./target/release/creditcoin-node --chain integration-tests/csub-552/reproSpec.json --validator --alice --mining-threads 1 --mining-key 5GsNKWrzHCPw1urznfdoHsrv1oDT1GxD7gpjvkR9LKibWTHh --pruning archive --node-key 844794b5f1da387970d769debc3a30f729f3515841121ccecebed2582723e04d --base-path ./integration-tests/csub-552/chaindata/node1
    ```

2. Set the block time to 5s (through polkadot-js)
3. Send runtime upgrade with `./target/release/wbuild/creditcoin-node-runtime/creditcoin_node_runtime.compact.compressed.wasm` as the wasm blob
4. Let node run until ~1070 blocks have been mined
    This number isn't particularly special, but the original panic occurred importing block 1061, so I aimed
    for at least that many blocks plus a few extra.

5. Export the blocks

    ```bash
    ./target/release/creditcoin-node export-blocks --chain integration-tests/csub-552/reproSpec.json --blocks-pruning archive --state-pruning archive --database paritydb integration-tests/csub-552/repro.blocks
    ```
