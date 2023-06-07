#!/bin/bash

set -xeuo pipefail

# shellcheck disable=SC2038
OUTPUT=$(find . -wholename "*/migrations/*.rs" | xargs grep "log::warn" || true)
if [ -n "$OUTPUT" ]; then
    echo "FAIL"
    echo "$OUTPUT"
    exit 1
fi

echo "PASS"
exit 0
