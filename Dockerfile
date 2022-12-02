FROM gluwa/ci-linux:production AS builder
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /creditcoin-node
COPY ci/env .
RUN source ~/.cargo/env && \
    source ./env && \
    rustup default $RUSTC_VERSION && \
    rustup update $RUSTC_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUSTC_VERSION

COPY Cargo.toml .
COPY Cargo.lock .
ADD node /creditcoin-node/node
ADD pallets /creditcoin-node/pallets
ADD primitives /creditcoin-node/primitives
ADD runtime /creditcoin-node/runtime
ADD sha3pow /creditcoin-node/sha3pow
ADD chainspecs /creditcoin-node/chainspecs
ADD test /creditcoin-node/test
RUN apt-get update && apt-get install -y protobuf-compiler
RUN source ~/.cargo/env && cargo build --release

FROM ubuntu:20.04
EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY chainspecs .
ENTRYPOINT [ "/bin/creditcoin-node" ]
