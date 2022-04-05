#!/usr/bin/env bash

# Example: Compile and benchmark the difficulty pallet.
# ./scripts/bench.sh -p difficulty -cb

PALLET=pallet_creditcoin
COMMAND=build
BENCH=1
BUILD=1

while getopts "fcbp:" opt;do
    case $opt in
    (f) COMMAND=check BUILD=0;;
    (c) BUILD=0;;
    (p) PALLET="pallet_$OPTARG";;
    (b) BENCH=0;;
    esac
done

if [[ $BUILD -eq 0 ]]
then
    cargo $COMMAND --release --features runtime-benchmarks || exit 1;
fi

if [[ $BENCH -eq 0 ]]
then
    ./target/release/creditcoin-node benchmark --chain dev --steps=50 --repeat=30 --pallet $PALLET --extrinsic='*' --execution wasm --wasm-execution=compiled --heap-pages=10000 --output ./runtime/src/weights/$PALLET.rs;
fi
