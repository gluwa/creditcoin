#!/bin/bash

set -e
PALLETS=$(grep "pallet::call" pallets/*/src/lib.rs -R | cut -f2 -d/ | sort | xargs)

if [ "$1" == "--show-pallets" ]; then
    echo "$PALLETS"
    exit 0
fi

# prepare the value for use with grep -E
PALLETS=$(echo "$PALLETS" | tr ' ' '|')
WHITELIST="fail_task persist_task_output"
EXTRINSICS=$(grep "pub fn" pallets/*/src/lib.rs | grep -E "$PALLETS" | cut -f2 -d":" | cut -f1 -d"(" | sed 's/pub fn //' | tr -d ' \t' | sort)

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
