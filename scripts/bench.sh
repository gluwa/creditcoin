#!/usr/bin/env bash

set -e

# Example: Compile and benchmark the difficulty pallet.
# ./scripts/bench.sh -p difficulty -cb

PALLET=creditcoin
COMMAND=build
BENCH=1
BUILD=1
REPEAT=30
STEPS=50

while getopts "fcbp:r:s:" opt;do
    case $opt in
        (f) COMMAND=check BUILD=0;;
        (c) BUILD=0;;
        (p) PALLET=$OPTARG;;
        (b) BENCH=0;;
        (r) REPEAT=$OPTARG;;
        (s) STEPS=$OPTARG;;
        (*)
            echo "ERROR: Invalid flag detected"
            exit 3
    esac
done


OUTPUT="./pallets/$PALLET/src/weights.rs"
mkdir -p "pallets/$PALLET/src"

if [[ $BUILD -eq 0 ]]
then
    cargo $COMMAND --release --features runtime-benchmarks || exit 1;
fi

if [[ $BENCH -eq 0 ]]
then
    ./target/release/creditcoin-node benchmark pallet --chain dev --steps="$STEPS" --repeat="$REPEAT" --pallet "pallet_$PALLET" --extrinsic='*' --execution wasm --wasm-execution=compiled --heap-pages=10000 --output "$OUTPUT"
    sed -i "s/pallet_$PALLET/crate/" "$OUTPUT"
fi
