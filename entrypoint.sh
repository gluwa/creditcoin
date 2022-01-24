#!/bin/env bash

if [[ -n "$BOOTNODE" ]]; then
    /bin/creditcoin-node --chain /chainspec/testnetSpec.json --node-key "$NODE_KEY " --name "$NODE_NAME" --bootnodes "$BOOTNODE" --port 30333 --ws-port 9944 --rpc-port 9933 \
    --base-path "$BASE_PATH"
else
    /bin/creditcoin-node --chain /chainspec/testnetSpec.json --node-key "$NODE_KEY " --name "$NODE_NAME" --port 30333 --ws-port 9944 --rpc-port 9933 \
    --base-path "$BASE_PATH"
fi