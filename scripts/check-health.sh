#!/bin/bash

set -xeuo pipefail

# Assert on the health of a freshly started container pointing to mainnet!
#
# IMPORTANT:
# In the initial 30 mins or so the node will be syncing blocks and the assertions
# below will PASS. Afterwards they will fail!

HEALTH=$(curl --silent http://127.0.0.1:9933/health)
IS_SYNCING=$(jq  -r '.isSyncing' <<< "$HEALTH")
SHOULD_HAVE_PEERS=$(jq  -r '.shouldHavePeers' <<< "$HEALTH")
PEERS=$(jq  -r '.peers' <<< "$HEALTH")

# WARNING: will fail once the node is within 5 blocks of tip
if [ "$IS_SYNCING" != "true" ]; then
    echo "FAIL"
    exit 1
fi

# WARNING: will fail if we're a boot node or haven't specified any --bootnodes
if [ "$SHOULD_HAVE_PEERS" != "true" ]; then
    echo "FAIL"
    exit 2
fi

# WARNING: will fail if running a local --dev node
if [[ ! "$PEERS" -gt 0 ]]; then
    echo "FAIL"
    exit 2
fi

# no assertion here, in case of error jq should fail
ROTATE_KEYS=$(curl --silent -X POST http://127.0.0.1:9933 -H 'Content-Type: application/json' -d '{ "jsonrpc": "2.0", "method": "author_rotateKeys", "params": [], "id": 1 }')
jq  -r '.result' <<< "$ROTATE_KEYS"

echo "PASS"
exit 0
