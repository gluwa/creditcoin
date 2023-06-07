#!/bin/bash

# Check if there are git [forks] specified in Cargo.toml files!
# Generally we don't want anything coming from git repositories b/c
# it will not be updated by Dependabot. Substrate is a notable exception!
#
# Any other dependency coming from a git repository could be a fork
# which is probably old and may lead to issues when upgrading.

USED_FORKS=$(find ./ -name Cargo.toml -print0 | xargs --null grep git |
# whitelist begin
    grep -v "repository =" |
    grep -v gluwa/substrate.git |
    # https://gluwa.atlassian.net/browse/CSUB-458
    grep -v nathanwhit/rust-bech32-bitcoin
# whitelist end
)
echo "INFO: Used forks in Cargo.toml files"
echo "$USED_FORKS"

if [ -n "$USED_FORKS" ]; then
    echo "FAIL: Cargo.toml files seem to specify dependencies from git forks"
    echo
    echo "TODO: For each individual dependency"
    cat <<_EOF_
1) Open a PR with upstream
2) Open a Jira ticket with a reference to the upstream PR +
   action item to replace the git repository in Cargo.toml start with crates.io +
   action item to clean-up the whitelist above!
3) Update the whitelist at the top of ./scripts/check-for-used-forks.sh to pass CI
_EOF_
    echo "TODO: ===== end ====="

    exit 1
fi

echo "PASS: Cargo.toml files don't specify dependencies from git forks"
exit 0
