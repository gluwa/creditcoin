FROM ctc/ci-linux:latest AS builder
ENV DEBIAN_FRONTEND=noninteractive
WORKDIR /creditcoin-node
COPY Cargo.toml .
COPY Cargo.lock .
ADD node /creditcoin-node/node
ADD pallets /creditcoin-node/pallets
ADD primitives /creditcoin-node/primitives
ADD runtime /creditcoin-node/runtime
ADD sha3pow /creditcoin-node/sha3pow
ADD chainspecs /creditcoin-node/chainspecs
RUN cargo build --release

FROM ubuntu:latest
EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY chainspecs .
ENTRYPOINT [ "/bin/creditcoin-node" ]