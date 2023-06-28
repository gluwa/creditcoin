#!/bin/bash
# shellcheck disable=SC2309
set -euo pipefail

# Wait until existing blocks are finalized
RPC_URL=${1:-http://127.0.0.1:9933}

LAST_BLOCK_NUMBER=0xff
FINAL_BLOCK_NUMBER=0x0

# wait until finalized block is within 5 blocks of the last one
while [[ $((LAST_BLOCK_NUMBER - FINAL_BLOCK_NUMBER)) -gt 0x5 ]]; do
# Bash seems to support hex math & comparisons if numbers are prefixed with 0x
# https://stackoverflow.com/a/13503150/1431647

    echo "INFO: Block Number from chain: $LAST_BLOCK_NUMBER; finalized: $FINAL_BLOCK_NUMBER"

    # WARNING: using getBlockHash() instead of getFinalizedHead() b/c PoW doesn't have finalization
    LAST_BLOCK=$(curl --silent -H "Content-Type: application/json" \
                  -d '{"id": 1, "jsonrpc": "2.0", "method": "chain_getBlockHash", "params": [] }' \
                    "$RPC_URL" | jq -r '.result'
    )

    LAST_BLOCK_NUMBER=$(curl --silent -H "Content-Type: application/json" \
         -d "{\"id\": 1, \"jsonrpc\": \"2.0\", \"method\": \"chain_getBlock\", \"params\": [\"$LAST_BLOCK\"] }" \
        "$RPC_URL" | jq -r '.result.block.header.number')

    if [ "$LAST_BLOCK_NUMBER" == "null" ]; then
        LAST_BLOCK_NUMBER="0xff"
        echo "FAIL: fetching block info for $LAST_BLOCK"
    fi

    FINAL_BLOCK=$(curl --silent -H "Content-Type: application/json" \
                  -d '{"id": 1, "jsonrpc": "2.0", "method": "chain_getFinalizedHead", "params": [] }' \
                    "$RPC_URL" | jq -r '.result'
    )

    FINAL_BLOCK_NUMBER=$(curl --silent -H "Content-Type: application/json" \
         -d "{\"id\": 1, \"jsonrpc\": \"2.0\", \"method\": \"chain_getBlock\", \"params\": [\"$FINAL_BLOCK\"] }" \
        "$RPC_URL" | jq -r '.result.block.header.number')

    if [ "$FINAL_BLOCK_NUMBER" == "null" ]; then
        FINAL_BLOCK_NUMBER="0x0"
        echo "FAIL: fetching block info for $FINAL_BLOCK"
    fi

    sleep 10
done

echo "DONE: blocks are finalized"
echo "INFO: Block Number from chain: $LAST_BLOCK_NUMBER; finalized: $FINAL_BLOCK_NUMBER"

exit 0
