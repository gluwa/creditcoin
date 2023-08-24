#!/bin/bash

mkdir actions-runner
pushd actions-runner || exit 1

curl -L https://github.com/actions/runner/releases/download/v2.302.1/actions-runner-linux-x64-2.302.1.tar.gz > runner.tar.gz

tar xzf ./runner.tar.gz
sudo ./bin/installdependencies.sh
sudo apt install -y jq

OWNER_REPO_SLUG="gluwa/creditcoin"
REPOSITORY_URL="https://github.com/$OWNER_REPO_SLUG"
EPHEMERAL=${LC_RUNNER_EPHEMERAL:-true}

# we need a temporary registration token first
REGISTRATION_TOKEN=$(curl --silent -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer $LC_GITHUB_REPO_ADMIN_TOKEN" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "https://api.github.com/repos/$OWNER_REPO_SLUG/actions/runners/registration-token" | jq -r '.token')

# Important: ephemeral runners are removed after a single job is executed on them
# which is inline with the VM lifecycle
./config.sh --unattended --ephemeral "$EPHEMERAL" --url "$REPOSITORY_URL" --token "$REGISTRATION_TOKEN" --labels "$LC_RUNNER_VM_NAME"
nohup ./run.sh >/dev/null 2>&1 </dev/null &
