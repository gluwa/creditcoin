#!/usr/bin/env bash

set -eu


GITHUB_BASE_REF="${1:-origin/testnet}"
GITHUB_HEAD_REF="${2:-HEAD}"

BASE_REF=$(git rev-parse "${GITHUB_BASE_REF}")

echo "base ref = $BASE_REF"

HEAD_REF=$(git rev-parse "${GITHUB_HEAD_REF}")

echo "head ref = $HEAD_REF"

get_spec_version () {
    local rev=$1

    git checkout "$rev" >/dev/null 2>&1

    local version
    version=$(grep -Eow -m 1 "spec_version: [0-9]+" ./runtime/src/version.rs | grep -Eow -m 1 "[0-9]+")

    if [[ -z $version ]]; then
        >&2 echo "Could not find spec version in version.rs at $rev"
        exit 1
    fi

    git checkout - >/dev/null 2>&1

    echo "$version"
}

base_version=$(get_spec_version "$BASE_REF")

head_version=$(get_spec_version "$HEAD_REF")

echo "base version = $base_version ; head version = $head_version"

if [[ "$base_version" == "$head_version" ]]; then
    echo 'No spec version change!'
    echo "::set-output name=needs_bench::0"
else
    echo 'Spec version changed!'
    echo "::set-output name=needs_bench::1"
fi
