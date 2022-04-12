#!/usr/bin/env bash

# Example: Compile and benchmark the difficulty pallet.
# ./scripts/bench.sh -p difficulty -cb

PALLET=creditcoin
COMMAND=build
BENCH=1
BUILD=1

while getopts "fcbp:" opt;do
    case $opt in
    (f) COMMAND=check BUILD=0;;
    (c) BUILD=0;;
    (p) PALLET=$OPTARG;;
    (b) BENCH=0;;
    esac
done


OUTPUT=./pallets/$PALLET/src/weights.rs

if [[ $BUILD -eq 0 ]]
then
    cargo $COMMAND --release --features runtime-benchmarks || exit 1;
fi

if [[ $BENCH -eq 0 ]]
then
    ./target/release/creditcoin-node benchmark --chain dev --steps=50 --repeat=30 --pallet pallet_$PALLET --extrinsic='*' --execution wasm --wasm-execution=compiled --heap-pages=10000 --output $OUTPUT
    sed -i s/pallet_$PALLET/super/ $OUTPUT
fi
