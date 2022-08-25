#!/usr/bin/env bash

# Check for any changes in any runtime directory. If there
# are no changes, check if the Substrate git SHA in Cargo.lock has been
# changed. If so, pull the repo and verify if {spec,impl}_versions have been
# altered since the previous Substrate version used.
#
# If there were changes in the runtime directory, check if {spec,impl}_version
# have been changed since the last release.


set -e

SUBSTRATE_REPO="https://github.com/gluwa/substrate"
SUBSTRATE_REPO_CARGO="git+${SUBSTRATE_REPO}"
MAIN_BRANCH=dev
RELEASE_BRANCH=main
RUNTIME="creditcoin-node-runtime"


# Reusable functions
function error {
    echo "Error; $1"
}

latest_release() {
  curl -s "https://api.github.com/repos/$1/releases/latest" | jq -r '.tag_name'
}
boldcat () { printf "|\n"; while read -r l; do printf "| \033[1m%s\033[0m\n" "${l}"; done; printf "|\n" ; }

has_runtime_changes() {
  from=$1
  to=$2

  if git diff --name-only "${from}...${to}" \
    | grep -q -e '^runtime'
  then
    return 0
  else
    return 1
  fi
}

# figure out the latest release tag
echo "Fetching git commit history for tags, $RELEASE_BRANCH and $MAIN_BRANCH"
git fetch --depth="${GIT_DEPTH:-100}" origin $RELEASE_BRANCH || exit 1
git fetch --depth="${GIT_DEPTH:-100}" origin 'refs/tags/*:refs/tags/*' || exit 1
LATEST_TAG=$(latest_release 'gluwa/creditcoin')
echo "latest release tag ${LATEST_TAG}"

echo "latest 10 commits of ${GITHUB_REF}"
git --no-pager log --graph --oneline --decorate=short -n 10

echo "make sure the MAIN branch is available in shallow clones"
git fetch --depth="${GIT_DEPTH:-100}" origin $MAIN_BRANCH || exit 1


# Helper function to join elements in an array with a multi-char delimiter
# https://stackoverflow.com/questions/1527049/how-can-i-join-elements-of-an-array-in-bash
function join_by { local d=$1; shift; echo -n "$1"; shift; printf "%s" "${@/#/$d}"; }
echo "check if the wasm sources changed since last commit to ${MAIN_BRANCH}"



if ! has_runtime_changes "origin/${MAIN_BRANCH}" "${GITHUB_SHA}"; then
    echo "No changes to any runtime source code detected"

    boldcat <<EOT
Checking Cargo.lock for changes in REFs for $SUBSTRATE_REPO
EOT

      SUBSTRATE_REFS_CHANGED="$(
    git diff "origin/$MAIN_BRANCH...${GITHUB_SHA}" Cargo.lock \
    | grep -e "$SUBSTRATE_REPO_CARGO" | awk -F '#' '{print $2}' | sort -u | wc -l
  )"

  # check Cargo.lock for substrate ref change
  case "$((SUBSTRATE_REFS_CHANGED))" in
    (0)
      echo "substrate refs not changed in Cargo.lock"
      exit 0
      ;;
    (2)
      echo "substrate refs updated since last commit to ${MAIN_BRANCH}"
      ;;
    (*)
      error "check unsupported: The commit REF targets are more than 2 in ${SUBSTRATE_REPO_CARGO}. Please fix it"
      exit 1
  esac

  SUBSTRATE_PREV_REF="$(
    git diff "origin/$MAIN_BRANCH...${GITHUB_SHA}" Cargo.lock \
    | grep -e '-source' | grep -e "$SUBSTRATE_REPO_CARGO" | awk -F '#' '{print $2}' | tr -d '"' | sort -u | head -n 1
  )"

  SUBSTRATE_NEW_REF="$(
    git diff "origin/$MAIN_BRANCH...${GITHUB_SHA}" Cargo.lock \
    | grep -e '+source' | grep -e "$SUBSTRATE_REPO_CARGO" | awk -F '#' '{print $2}' | tr -d '"' | sort -u | head -n 1
  )"

  if [[ -z "${SUBSTRATE_PREV_REF}" && -z "${SUBSTRATE_NEW_REF}" ]]; then
      error "The substrate dependency commit references are empty, ensure your branches are up to date"
      exit 1
  fi

  boldcat <<EOT
previous substrate commit id ${SUBSTRATE_PREV_REF}
new substrate commit id      ${SUBSTRATE_NEW_REF}
EOT

  # NOTE: The gluwa/substrate repository is cloned using git checkouts. To run it local, uncomment:
  #   git clone --depth="${GIT_DEPTH:-100}" -n --no-tags \
  #     "${SUBSTRATE_REPO}" || exit 1 #"${SUBSTRATE_CLONE_DIR}" || exit 1
  

  echo "Checking for spec/impl_version changes in substrate repo."
  git --no-pager -C "./substrate" diff "${SUBSTRATE_PREV_REF}..${SUBSTRATE_NEW_REF}" \
    | grep -E '^[\+\-][[:space:]]+(spec|impl)_version: +([0-9]+),$' || exit 0


  boldcat <<EOT
spec_version or or impl_version have changed in substrate after updating Cargo.lock
please make sure versions are bumped up accordingly
EOT
  exit 1

fi


# Check if there were changes in runtime.
# If not, we can skip to the next runtime

if ! git diff --name-only "origin/${MAIN_BRANCH}...${GITHUB_SHA}" \
    | grep -E -q -e "runtime"; then
    echo "No changes in runtime"
fi
# check for spec_version updates: if the spec versions changed, then there is
# consensus-critical logic that has changed. the runtime wasm blobs must be
# rebuilt.

boldcat <<EOT
Checking for version changes in add_spec_version in version.rs
EOT
add_spec_version="$(
    git diff "origin/${MAIN_BRANCH}...${GITHUB_SHA}" "runtime/src/version.rs" \
    | sed -n -r "s/^\+[[:space:]]+spec_version: +([0-9]+),$/\1/p"
)"
sub_spec_version="$(
    git diff "origin/${MAIN_BRANCH}...${GITHUB_SHA}" "runtime/src/version.rs" \
    | sed -n -r "s/^\-[[:space:]]+spec_version: +([0-9]+),$/\1/p"
)"


# see if the version and the binary blob changed
if [ "${add_spec_version}" != "${sub_spec_version}" ]
then

    boldcat <<EOT
## RUNTIME: ${RUNTIME} ##

changes to the ${RUNTIME} sources and changes in the spec version.

spec_version: ${sub_spec_version} -> ${add_spec_version}

EOT
else
    # check for impl_version updates: if only the impl versions changed, we assume
    # there is no consensus-critical logic that has changed.

    boldcat <<EOT
Checking for version changes in add_impl_version in version.rs
EOT

    add_impl_version="$(
    git diff "origin/${MAIN_BRANCH}...${GITHUB_SHA}" "runtime/src/version.rs" \
    | sed -n -r 's/^\+[[:space:]]+impl_version: +([0-9]+),$/\1/p'
    )"
    sub_impl_version="$(
    git diff "origin/${MAIN_BRANCH}...${GITHUB_SHA}" "runtime/src/version.rs" \
    | sed -n -r 's/^\-[[:space:]]+impl_version: +([0-9]+),$/\1/p'
    )"


    # see if the impl version changed
    if [ "${add_impl_version}" != "${sub_impl_version}" ]
    then
    boldcat <<EOT

## RUNTIME: ${RUNTIME} ##

changes to the ${RUNTIME} runtime sources and changes in the impl version.

impl_version: ${sub_impl_version} -> ${add_impl_version}

EOT
        echo "INFO: which change is fine, exiting ..."
        exit 0
    fi

    boldcat <<EOT
wasm source files changed or the spec version in the substrate reference in
the Cargo.lock but not the spec/impl version. If changes made do not alter
logic, just bump 'impl_version'. If they do change logic, bump
'spec_version'.

source file directories:
- runtime

version files: ${failed_runtime_checks[@]}
EOT
    exit 1
fi
