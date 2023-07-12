#!/bin/bash

set -xeuo pipefail

# Colorful output.
function greenprint {
    echo -e "\033[1;32m[$(date -Isecond)] ${1}\033[0m"
}

check_block_time() {
    # WARNING: exits on error
    from=$1
    to=$2

    if git --no-pager diff "${from}...${to}" | grep 'MILLISECS_PER_BLOCK'; then
        greenprint "FAIL: MILLISECS_PER_BLOCK has been modified! This will brick the blockchain!"
        exit 1
    else
        greenprint "PASS: MILLISECS_PER_BLOCK has not been modified!"
    fi
}

check_epoch_duration() {
    # WARNING: exits on error
    from=$1
    to=$2

    if git --no-pager diff "${from}...${to}" | grep 'EPOCH_DURATION'; then
        greenprint "FAIL: EPOCH_DURATION has been modified! This will brick the blockchain!"
        exit 1
    else
        greenprint "PASS: EPOCH_DURATION has not been modified!"
    fi
}

check_slot_duration() {
    # WARNING: exits on error
    from=$1
    to=$2

    if git --no-pager diff "${from}...${to}" | grep 'SLOT_DURATION'; then
        greenprint "FAIL: SLOT_DURATION has been modified! This will brick the blockchain!"
        exit 1
    else
        greenprint "PASS: SLOT_DURATION has not been modified!"
    fi
}


#### main part

FROM=$(git rev-parse "${1:-origin/dev}")
TO=$(git rev-parse "${2:-HEAD}")

greenprint "DEBUG: Inspecting range $FROM..$TO"

if [ -z "$FROM" ]; then
    echo "ERROR: FROM is empty. Exiting..."
    exit 2
fi

if [ -z "$TO" ]; then
    echo "ERROR: TO is empty. Exiting..."
    exit 2
fi

if git --no-pager diff --name-only "${FROM}"..."${TO}" | grep -e '^runtime'; then
    greenprint "INFO: runtime/ has been modified. Checking for changes in EPOCH_DURATION!"
    check_block_time "${FROM}" "${TO}"
    check_epoch_duration "${FROM}" "${TO}"
    check_slot_duration "${FROM}" "${TO}"
else
    greenprint "INFO: runtime/ has NOT been modified. Will NOT check for changes in EPOCH_DURATION!"
fi
