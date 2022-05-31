FROM rust:slim-bullseye AS builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y cmake \
    pkg-config \
    libssl-dev \
    git \
    build-essential \
    clang \
    libclang-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*
RUN rustup default stable
RUN rustup update
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
WORKDIR /creditcoin-node
COPY Cargo.toml .
COPY Cargo.lock .
ADD node /creditcoin-node/node
ADD pallets /creditcoin-node/pallets
ADD primitives /creditcoin-node/primitives
ADD runtime /creditcoin-node/runtime
ADD sha3pow /creditcoin-node/sha3pow
ADD chainspecs /creditcoin-node/chainspecs
RUN cargo fetch
RUN rm -rf /creditcoin-node/node
RUN rm -rf /creditcoin-node/pallets
RUN rm -rf /creditcoin-node/primitives
RUN rm -rf /creditcoin-node/runtime
RUN rm -rf /creditcoin-node/sha3pow
RUN rm -rf /creditcoin-node/chainspecs