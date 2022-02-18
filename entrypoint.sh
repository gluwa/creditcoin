#!/bin/bash
#shellcheck disable=SC2153

base_path="--base-path ${DATA:-/data}"

if [ "$CHAIN" = "testnet" ]; then
    chain="--chain /testnetSpec.json"
elif [ "$CHAIN" = "qa" ]; then
    chain="--chain /qaSpec.json"
elif [ -z "$CHAIN" ]; then # no chain specified, fallback to testnet for now
    chain="--chain /testnetSpec.json"
else # try whatever is specified
    chain="--chain ${CHAIN}"
fi

if [ "$MODE" = "rpc" ]; then
    ws='--ws-external'
    rpc='--rpc-external'
    cors="--rpc-cors ${CORS:-all}"
    pruning="--pruning ${PRUNING:-archive}"
else
    validator='--validator'
    if [ "$MINING_KEY" ]; then
        mining_key="--mining_key ${MINING_KEY}"
    else
        echo 'Generating mining keypair because none was specified, access the keystore to see the secret key'
        #shellcheck disable=SC2086
        mining_key="--mining-key $(/bin/creditcoin-node generate-mining-key $chain --quiet $base_path)"
    fi
    if [ "$MODE" = "authority" ]; then
        if [ -n "$AUTHORITY_SECRET" ]; then
            #shellcheck disable=SC2086
            /bin/creditcoin-node key insert $chain $base_path --key-type 'ctcs' --scheme 'Sr25519' --suri "$AUTHORITY_SECRET" || exit 1
        else
            echo 'Authority nodes require AUTHORITY_SECRET env var'
            exit 1
        fi
        if [ -n "$AUTHORITY_RPC_MAPPING" ]; then
            rpc_mapping="--rpc-mapping $AUTHORITY_RPC_MAPPING"
        else
            echo 'Authority nodes should have AUTHORITY_RPC_MAPPING specified'
            exit 1
        fi
    fi
fi
if [ "$RESYNC" ]; then
    #shellcheck disable=SC2086
    /bin/creditcoin-node purge-chain -y $chain $base_path
fi
if [ "$PROMETHEUS" = "1" ]; then
    prometheus='--prometheus-external'
fi
if [ -n "$BOOTNODE_IP" ] || [ -n "$BOOTNODE_FQDN" ]; then
    boot_id="${BOOTNODE_PEER_ID:-12D3KooWSnPk7hN9epDUonVuNVgTvxbmnuQbGP8ghMvuoH9GAsz5}"
    if [ "$NODE_NAME" ]; then
        name="--name $NODE_NAME"
    fi
    if [ "$BOOTNODE_FQDN" ]; then
        bootnode="--bootnodes /dns4/$BOOTNODE_FQDN/tcp/30333/p2p/$boot_id"
    else
        bootnode="--bootnodes /ip4/$BOOTNODE_IP/tcp/30333/p2p/$boot_id"
    fi
    #shellcheck disable=SC2086
    /bin/creditcoin-node $name $bootnode \
     --port 30333 --ws-port 9944 --rpc-port 9933 \
     --public-addr "/dns4/$FQDN/tcp/30333" \
     --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
     $chain $base_path $validator $mining_key $ws $rpc \
     $cors $pruning $prometheus $rpc_mapping $EXTRA_ARGS
else
    key="${NODE_KEY:-c5eb4a9ada5c9dd76378d000f046e8cde064d68e96a1df569190eee70afba8e7}"
    node_name="${NODE_NAME:-bootnode}"
    #shellcheck disable=SC2086
    /bin/creditcoin-node --node-key "$key" --name "$node_name" \
     --port 30333 --ws-port 9944 --rpc-port 9933 \
     --public-addr "/dns4/$FQDN/tcp/30333" \
     --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
     $chain $base_path $validator $mining_key $ws $rpc \
     $cors $pruning $prometheus $rpc_mapping $EXTRA_ARGS
fi
