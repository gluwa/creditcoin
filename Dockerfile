FROM gluwa/ci-linux:production AS builder
ENV DEBIAN_FRONTEND=noninteractive
RUN source ~/.cargo/env && rustup default nightly && rustup update nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
WORKDIR /creditcoin-node
COPY Cargo.toml .
COPY Cargo.lock .
ADD node /creditcoin-node/node
ADD pallets /creditcoin-node/pallets
ADD primitives /creditcoin-node/primitives
ADD runtime /creditcoin-node/runtime
ADD sha3pow /creditcoin-node/sha3pow
ADD chainspecs /creditcoin-node/chainspecs
RUN source ~/.cargo/env && cargo build --release

FROM ubuntu:20.04
EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY chainspecs .
ENTRYPOINT [ "/bin/creditcoin-node" ]