#!/bin/bash

set -e

PALLET_WHITELIST='pos-switch'
FILES_WITH_EXTRINSICS=$(grep -E "pallet::call|pallet::hooks" pallets/* -R | grep -Ev "$PALLET_WHITELIST" | cut -f1 -d: | sort | uniq)
PALLETS=$(echo "$FILES_WITH_EXTRINSICS" | cut -f2 -d/ | sort | uniq | xargs)

if [ "$1" == "--show-pallets" ]; then
    echo "$PALLETS" | tr "-" "_"
    exit 0
fi

# prepare the value for use with grep -E
FILES_WITH_EXTRINSICS=$(echo "$FILES_WITH_EXTRINSICS" | xargs)
WHITELIST="fail_task persist_task_output issue_reward reward_amount sawtooth_adjusted_height"

# NOTE: $FILES_WITH_EXTRINSICS isn't quoted below because we want the shell
# to split the words, i.e. tell grep to search only in specific files
# shellcheck disable=SC2086
EXTRINSICS=$(grep "pub fn" $FILES_WITH_EXTRINSICS | cut -f2 -d":" | cut -f1 -d"(" | sed 's/pub fn //' | tr -d ' \t' | sort)

echo "----- Detected extrinsics are -----"
echo "$EXTRINSICS"

for EXTRINSIC in $EXTRINSICS; do
    if [[ $WHITELIST =~ $EXTRINSIC ]]; then
        echo "***** Skipping $EXTRINSIC - white listed"
        continue
    fi

    echo "----- Searching weights for $EXTRINSIC -----"
    grep "fn $EXTRINSIC" pallets/*/src/weights.rs

    # makes sure the weight for an extrinsic function has the
    # same name as the extrinsic - in case copy&paste errors, etc
    echo "----- Double check weights for $EXTRINSIC -----"
    grep -B1 "pub fn $EXTRINSIC" pallets/*/src/lib.rs | grep "WeightInfo::$EXTRINSIC"

    echo "----- Searching benchmarks for $EXTRINSIC -----"
    grep "$EXTRINSIC {" pallets/*/src/benchmarking.rs
done

echo "----- DONE - ALL PASS -----"
