#!/bin/bash

if [ -n "$BOOTNODE_IP" ]; then
    boot_id="${BOOTNODE_PEER_ID:-12D3KooWSnPk7hN9epDUonVuNVgTvxbmnuQbGP8ghMvuoH9GAsz5}"
    if [ "$NODE_NAME"]; then
        name="--name $NODE_NAME"
    else
        name=""
    fi
    /bin/creditcoin-node --chain /chainspec/testnetSpec.json $name --bootnodes "/ip4/$BOOTNODE_IP/tcp/30333/p2p/$boot_id" \
     --port 30333 --ws-port 9944 --rpc-port 9933 --validator \
     --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
else
    key="${NODE_KEY:-c5eb4a9ada5c9dd76378d000f046e8cde064d68e96a1df569190eee70afba8e7}"
    node_name="${NODE_NAME:-bootnode}"
    /bin/creditcoin-node --chain /chainspec/testnetSpec.json --node-key $key --name "$node_name" \
     --port 30333 --ws-port 9944 --rpc-port 9933 --validator \
     --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
fi