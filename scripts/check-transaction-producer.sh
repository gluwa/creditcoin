#!/bin/bash

set -xeuo pipefail

# Sanity check that creditcoin-transaction-producer didn't crash

# IMPORTANT: log file is created from the calling environment

grep "Server running! Navigate to http://localhost:8080" ~/creditcoin-transaction-producer.log
curl http://localhost:8080
