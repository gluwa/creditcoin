#!/usr/bin/env bash

# This script:
# - Checks for changes in transaction version in runtime/src/version.rs
# - Downloads latest release binary from gluwa/creditcoin (RELEASE_BIN)
# - Compiles and build a binary from the current branch (HEAD_BIN)
# - Runs the two nodes

set -ex

HEAD_BIN=./target/release/creditcoin-node
HEAD_WS=ws://localhost:9944
RELEASE_WS=ws://localhost:9945

runtimes=(
  "creditcoin-node"
)

# First we fetch the latest released binary
latest_release() {
  curl -s "https://api.github.com/repos/$1/releases/latest" | jq -r '.tag_name'
}

latest_release=$(latest_release 'gluwa/creditcoin')
RELEASE_BIN="./creditcoin-node"
echo "[+] Fetching binary for Creditcoin version $latest_release"
curl -L "https://github.com/gluwa/creditcoin/releases/download/$latest_release/creditcoin-${latest_release}-x86_64-unknown-linux-gnu.zip"  --output creditcoin.zip
unzip creditcoin.zip
chmod +x "$RELEASE_BIN"
git fetch --depth="${GIT_DEPTH:-100}" origin 'refs/tags/*:refs/tags/*'


for RUNTIME in "${runtimes[@]}"; do
  echo "[+] Checking runtime: ${RUNTIME}"

  release_transaction_version=$(git show "tags/$latest_release:runtime/src/version.rs" | grep 'transaction_version')

  current_transaction_version=$(
    grep 'transaction_version' "./runtime/src/version.rs"
  )

  echo "[+] Release: ${release_transaction_version}"
  echo "[+] Ours: ${current_transaction_version}"


  # Start running the nodes in the background
  $HEAD_BIN --chain=local --tmp >head-node.log 2>&1 &
  $RELEASE_BIN --chain=local --ws-port 9945 --tmp --port 30334 --rpc-port 9934 >release-node.log 2>&1 &
  jobs

  #Wait for HEAD BINARY
  ./integration-tests/wait-for-creditcoin.sh 'http://127.0.0.1:9933'
  #Wait for RELEASE BINARY
  ./integration-tests/wait-for-creditcoin.sh 'http://127.0.0.1:9934'

  changed_extrinsics=$(
    polkadot-js-metadata-cmp "$RELEASE_WS" "$HEAD_WS" \
      | sed 's/^ \+//g' | grep -e 'idx: [0-9]\+ -> [0-9]\+' || true
  )

  # compare to mainnet and testnet explicitly b/c latest release could be any of them
  # for now this comparison is only used to provide more info in CI
  polkadot-js-metadata-cmp wss://rpc.mainnet.creditcoin.network/ws "$HEAD_WS" > metadata-cmp-with-mainnet.txt
  polkadot-js-metadata-cmp wss://rpc.testnet.creditcoin.network/ws "$HEAD_WS" > metadata-cmp-with-testnet.txt

  if [ -n "$changed_extrinsics" ]; then
    echo "[!] Extrinsics indexing/ordering has changed in the ${RUNTIME} runtime! If this change is intentional, please bump transaction_version in lib.rs. Changed extrinsics:"
    echo "$changed_extrinsics"

    if [ "$release_transaction_version" == "$current_transaction_version" ]; then
        exit 1
    else
        echo "[+] Transaction version for ${RUNTIME} has been bumped since last release. Exiting."
    fi
  fi

  echo "[+] No change in extrinsics ordering for the ${RUNTIME} runtime"
  jobs -p | xargs kill -9
done
