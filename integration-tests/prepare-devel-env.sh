#!/bin/bash

TARGET_URL=http://127.0.0.1:9933

curl -H "Content-Type: application/json" -d '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' $TARGET_URL > creditcoin.json

yarn build:types
