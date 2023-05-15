# hadolint global ignore=DL3008,DL4006,SC2086

FROM ubuntu:22.04 as builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
    ca-certificates cmake pkg-config libssl-dev git build-essential clang libclang-dev curl protobuf-compiler && \
    update-ca-certificates
RUN useradd --home-dir /creditcoin-node --create-home creditcoin
USER creditcoin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/sh -s -- -y

WORKDIR /creditcoin-node
COPY ci/env .
SHELL ["/bin/bash", "-c"]
# shellcheck source=/dev/null
RUN source ~/.cargo/env && \
    source ./env && \
    rustup default $RUSTC_VERSION && \
    rustup update $RUSTC_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUSTC_VERSION

COPY Cargo.toml .
COPY Cargo.lock .
COPY node /creditcoin-node/node
COPY pallets /creditcoin-node/pallets
COPY primitives /creditcoin-node/primitives
COPY runtime /creditcoin-node/runtime
COPY sha3pow /creditcoin-node/sha3pow
COPY chainspecs /creditcoin-node/chainspecs
COPY test /creditcoin-node/test
RUN source ~/.cargo/env && cargo build --release


FROM node:16.13.0-bullseye-slim AS cli-builder
WORKDIR /creditcoin-cli
COPY creditcoin-js /creditcoin-cli/creditcoin-js
COPY scripts/cc-cli /creditcoin-cli/scripts/cc-cli
# RUN apt-get update && apt-get install curl -y && \
#     curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
#     apt-get install -y nodejs
WORKDIR /creditcoin-cli/creditcoin-js
RUN yarn install && yarn build && yarn pack
WORKDIR /creditcoin-cli/scripts/cc-cli
RUN yarn install && yarn build && yarn pack

FROM ubuntu:22.04
ARG cli_version=1.0.0
ARG creditcoin_js_version=0.9.5
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=cli-builder /creditcoin-cli/creditcoin-js/creditcoin-js-v${creditcoin_js_version}.tgz /creditcoin-cli/creditcoin-js/creditcoin-js-v${creditcoin_js_version}.tgz
COPY --from=cli-builder /creditcoin-cli/scripts/cc-cli/creditcoin-cli-v${cli_version}.tgz /creditcoin-cli/scripts/cli/creditcoin-cli-v${cli_version}.tgz
RUN apt-get update && apt-get install curl -y --no-install-recommends && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /creditcoin-cli/scripts/cli
RUN npm install -g creditcoin-cli-v${cli_version}.tgz && \
    useradd --home-dir /creditcoin-node --create-home creditcoin

USER creditcoin

EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
SHELL ["/bin/bash", "-c"]
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY --from=builder /creditcoin-node/chainspecs /
ENTRYPOINT [ "/bin/creditcoin-node" ]
