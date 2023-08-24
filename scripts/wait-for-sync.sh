#!/bin/bash

set -euo pipefail

# Wait until a node syncs with the rest of the blockchain!
WAIT_FOR_BLOCK_NUMBER=${1:-0x0}
RPC_URL=${2:-http://127.0.0.1:9933}
COUNTER=1

IS_SYNCING="true"
while [ "$IS_SYNCING" == "true" ]; do
    echo "INFO: $COUNTER - waiting for sync ..."
    COUNTER=$((COUNTER + 1))
    sleep 60

    IS_SYNCING=$(curl --silent "$RPC_URL/health" | jq  -r '.isSyncing')
done
echo "INFO: blockchain health reported that it's not syncing anymore!"
echo "INFO: will start comparing blocks numbers ......"


LAST_BLOCK_NUMBER=0x0
# Bash seems to support hex comparisons if numbers are prefixed with 0x
# https://stackoverflow.com/a/13503150/1431647
while [[ $LAST_BLOCK_NUMBER -lt $WAIT_FOR_BLOCK_NUMBER ]]; do
    echo "INFO: Block Number from chain: $LAST_BLOCK_NUMBER < target: $WAIT_FOR_BLOCK_NUMBER"
    # WARNING: using getBlockHash() instead of getFinalizedHead() b/c PoW doesn't have finalization
    LAST_BLOCK=$(curl --silent -H "Content-Type: application/json" \
                  -d '{"id": 1, "jsonrpc": "2.0", "method": "chain_getBlockHash", "params": [] }' \
                    "$RPC_URL" | jq -r '.result'
    )

    LAST_BLOCK_NUMBER=$(curl --silent -H "Content-Type: application/json" \
         -d "{\"id\": 1, \"jsonrpc\": \"2.0\", \"method\": \"chain_getBlock\", \"params\": [\"$LAST_BLOCK\"] }" \
        "$RPC_URL" | jq -r '.result.block.header.number')

    if [ "$LAST_BLOCK_NUMBER" == "null" ]; then
        LAST_BLOCK_NUMBER="0x0"
        echo "INFO: retry fetching block info for $LAST_BLOCK"
    fi

    sleep 60
done

echo "DONE: node is fully in sync"
echo "INFO: Block Number from chain: $LAST_BLOCK_NUMBER; target: $WAIT_FOR_BLOCK_NUMBER"

exit 0
