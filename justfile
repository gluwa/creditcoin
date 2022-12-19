set shell := ["bash", "-uc"]

creditcoin:
    pushd creditcoin-js
    rm -f *tgz
    yarn build:types
    yarn format
    yarn pack

test:
    pushd integration-tests
    yarn install --update-checksums
    yarn upgrade 'creditcoin-js'
    yarn test

check:
    cargo +nightly check --all-features --all-targets

clippy:
    SKIP_WASM_BUILD=1 cargo clippy --fix --all-features --all-targets --allow-staged

inspect PRIVATE_KEY:
    ./target/debug/creditcoin-node key inspect --scheme Ed25519 {{PRIVATE_KEY}}
    ./target/debug/creditcoin-node key inspect --scheme Sr25519 {{PRIVATE_KEY}}
    ./target/debug/creditcoin-node key inspect --scheme Ecdsa {{PRIVATE_KEY}}

up: check clippy test creditcoin