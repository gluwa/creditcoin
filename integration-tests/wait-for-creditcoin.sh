#!/bin/bash

pushd ../creditcoin-js/ && yarn install && popd
yarn install

TARGET_URL=${1:-http://127.0.0.1:9933}
CURL_PARAMS="-H 'Content-Type: application/json' -d '{\"id\":\"1\", \"jsonrpc\":\"2.0\", \"method\": \"state_getMetadata\", \"params\":[]}' $TARGET_URL"

COUNTER=0
# make sure there is a node running at TARGET_URL
while [[ "$(eval curl -s -o /dev/null -w '%{http_code}' $CURL_PARAMS)" != "200" && $COUNTER -lt 20 ]]; do
    echo "INFO: $COUNTER - Not ready yet ....."
    let COUNTER=COUNTER+1
    sleep 20
done

# fail if we still can't connect after 20 attempts
set -e

# Note: using eval b/c params are specified as string above
$(eval curl $CURL_PARAMS > /dev/null)
