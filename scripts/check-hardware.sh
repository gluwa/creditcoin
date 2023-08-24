#!/bin/bash

set -xeuo pipefail

cat /proc/cpuinfo
free -m
cat /proc/meminfo

./target/release/creditcoin-node benchmark machine --chain main
