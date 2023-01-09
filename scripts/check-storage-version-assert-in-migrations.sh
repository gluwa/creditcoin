#!/bin/bash

set -euo pipefail

fail() {
    echo "FAIL: $1"
    exit 1
}

for MIGRATION in $(find ./pallets/ -wholename "*/migrations/v*.rs" | sort); do
    VERSION=$(basename "$MIGRATION" | tr -d 'v.rs')

    echo "INFO: inspecting $MIGRATION"

    # check if the migration file contains post_upgrade()
    grep "fn post_upgrade" "$MIGRATION" || fail "fn post_upgrade() not found in $MIGRATION"

    # check if the migration file asserts on storage version
    grep "StorageVersion::get::<crate::Pallet<T>>() == $VERSION" "$MIGRATION" || fail "Assertion 'StorageVersion == $VERSION' not found in $MIGRATION"
done

echo "PASS"
exit 0
