#!/bin/bash

set -x

# Install linode-cli
python3 --version
pip install -r .github/requirements.txt
linode-cli --version

# Authorize hosted-runner
mkdir -p ~/.ssh/
ssh-keygen -q -t rsa -N '' -f ~/.ssh/id_rsa
cat ~/.ssh/id_rsa.pub >> .github/authorized_keys


# Provision VM
echo "INFO: From ENVs: RUNNER_VM_NAME=$LC_RUNNER_VM_NAME"

# inject authorized keys into cloud-init for the `ubuntu@` user
while read -r LINE; do
  echo "      - $LINE" >> .github/linode-cloud-init.template
done < .github/authorized_keys

# retry until we get a VM
IP_ADDRESS=""
COUNTER=0
while [ -z "$IP_ADDRESS" ]; do
    # if all jobs retry rate-limited queries at the same time they still hit the limit
    # and subsequently fail. Max number of retries is hard-coded to 3 in linodecli
    # use up to 60 sec random delay to avoid everything being scheduled at once!
    sleep $((RANDOM % 60))

    # WARNING: we do not specify --authorized_keys for root b/c
    # linode-cli expects each key as a separate argument and iteratively constructing
    # the argument list hits issues with quoting the jey values b/c of white-space.
    # All SSH logins should be via the `ubuntu@` user. For more info see:
    # https://www.linode.com/community/questions/21290/how-to-pass-multiple-ssh-public-keys-with-linode-cli-linodes-create
    linode-cli linodes create --json \
        --image 'linode/ubuntu22.04' --region "$LINODE_REGION" \
        --type "$LINODE_VM_SIZE" --label "$LC_RUNNER_VM_NAME" \
        --root_pass "$(uuidgen)" --backups_enabled false --booted true --private_ip false \
        --metadata.user_data "$(base64 --wrap 0 < .github/linode-cloud-init.template)" > "retry_$COUNTER.json"

    IP_ADDRESS=$(jq -r '.[0].ipv4[0]' < "retry_$COUNTER.json")

    (( COUNTER=COUNTER+1 ))
done

# provision the GitHub Runner binary on the VM
# passing additional ENV values
SSH_USER_AT_HOSTNAME="ubuntu@$IP_ADDRESS"
echo "INFO: $SSH_USER_AT_HOSTNAME"

until ssh -i ~/.ssh/id_rsa \
  -o StrictHostKeyChecking=no "$SSH_USER_AT_HOSTNAME" < .github/install-docker-engine-from-upstream.sh; do
  echo "DEBUG: retrying ssh connection ..."
  sleep 30
done

until ssh -i ~/.ssh/id_rsa \
  -o SendEnv=LC_GITHUB_REPO_ADMIN_TOKEN,LC_RUNNER_VM_NAME,LC_WORKFLOW_ID,LC_PROXY_ENABLED,LC_PROXY_SECRET_VARIANT,LC_PROXY_TYPE \
  -o StrictHostKeyChecking=no "$SSH_USER_AT_HOSTNAME" < scripts/provision-github-runner.sh; do
  echo "DEBUG: retrying ssh connection ..."
  sleep 30
done
