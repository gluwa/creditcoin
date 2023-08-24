#!/bin/bash

set -euo pipefail

### find the latest release for testnet or mainnet
GREP_FOR="$1"
RELEASE_TAG=$(curl --silent https://api.github.com/repos/gluwa/creditcoin/releases | jq -r ".[].tag_name" | grep "$GREP_FOR" | head -n1)

echo "$RELEASE_TAG"
