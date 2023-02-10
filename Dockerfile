# hadolint global ignore=DL3008,DL4006,SC2086

FROM ubuntu:22.04 as builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
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

FROM ubuntu:22.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get upgrade -y && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*
RUN useradd --home-dir /creditcoin-node --create-home creditcoin
USER creditcoin

EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
SHELL ["/bin/bash", "-c"]
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
ENTRYPOINT [ "/bin/creditcoin-node" ]
