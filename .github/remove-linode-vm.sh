#!/bin/bash

set -x

# Install linode-cli
python3 --version
pipx install linode-cli
export PATH="$PATH:~/.local/bin"
linode-cli --version

# if a specific VM was not specified then remove all zombies
if [ -z "$LC_RUNNER_VM_NAME" ]; then
    THRESHOLD=$(date --utc "+%Y-%m-%dT%H:%M:%S" -d "5 hours ago")

    for VM_ID in $(linode-cli linodes list --json | jq ".[] | select(.created <= \"$THRESHOLD\")" | jq -r '.id'); do
        echo "INFO: going to remove expired VM $VM_ID"
        linode-cli linodes delete "$VM_ID"
    done
else
    VM_ID=$(linode-cli linodes list --json --label "$LC_RUNNER_VM_NAME" | jq -r '.[0].id')
    linode-cli linodes delete "$VM_ID"
fi
