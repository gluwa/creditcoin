#!/bin/bash

set -xeuo pipefail

# Colorful output.
function greenprint {
    echo -e "\033[1;32m[$(date -Isecond)] ${1}\033[0m"
}

check_version() {
    # WARNING: exits on error
    from=$1
    to=$2

    if git --no-pager diff "${from}...${to}" | grep '^diff --git' | grep 'runtime/src/version.rs'; then
        greenprint "PASS: version.rs was modified!"
    else
        greenprint "FAIL: version.rs was not modified!"
        exit 1
    fi
}

#### main part

VERSION_FROM_CARGO_TOML=$(grep "^version =" Cargo.toml  | cut -f2 -d'=' | tr -d "' \"")

SPEC_VERSION=$(grep spec_version: runtime/src/version.rs | cut -f2 -d: | tr -d " ,")
IMPL_VERSION=$(grep impl_version: runtime/src/version.rs | cut -f2 -d: | tr -d " ,")
VERSION_FROM_VERSION_RS="2.$SPEC_VERSION.$IMPL_VERSION"

# Since PR #969 version strings in Cargo.toml and version.rs should be in sync
echo "INFO: Cargo.toml version is $VERSION_FROM_CARGO_TOML"
echo "INFO: version.rs version is $VERSION_FROM_VERSION_RS"
if [ "$VERSION_FROM_CARGO_TOML" != "$VERSION_FROM_VERSION_RS" ]; then
    echo "FAIL: Versions in Cargo.toml and runtime/src/version.rs are not in sync"
    exit 2
fi


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
    greenprint "INFO: runtime/src/ has been modified"
    check_version "${FROM}" "${TO}"
else
    greenprint "INFO: runtime/src/ didn't change. Will inspect Cargo.lock"
    if git --no-pager diff "${FROM}"..."${TO}" Cargo.lock | grep '+source = "git+https://github.com/gluwa/substrate.git'; then
        echo "INFO: Cargo.lock references to Substrate have been modified"
        check_version "${FROM}" "${TO}"
    else
        greenprint "INFO: Cargo.lock references to Substrate did not change"
    fi
fi
