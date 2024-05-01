# hadolint global ignore=DL3008,DL3009,DL3016,SC3044,SC3046,DL4006,SC2086,SC3009

FROM ubuntu:24.04 as runtime-base
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    update-ca-certificates && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs --no-install-recommends && \
    npm install -g yarn

RUN useradd --home-dir /creditcoin-node --create-home creditcoin
USER creditcoin
SHELL ["/bin/bash", "-c"]
WORKDIR /creditcoin-node


FROM runtime-base AS devel-base
COPY --chown=creditcoin:creditcoin . /creditcoin-node/


FROM devel-base as rust-builder
USER 0
RUN apt-get install -y --no-install-recommends \
    cmake pkg-config libssl-dev git build-essential clang libclang-dev protobuf-compiler
USER creditcoin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/sh -s -- -y

COPY --chown=creditcoin:creditcoin . /creditcoin-node/
# shellcheck source=/dev/null
RUN source ~/.cargo/env && \
    source ./ci/env && \
    rustup default $RUSTC_VERSION && \
    rustup update $RUSTC_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUSTC_VERSION && \
    source ~/.cargo/env && \
    cargo build --release


FROM devel-base AS cli-builder
RUN pushd ~/creditcoin-js && \
    yarn install && yarn build && yarn pack && \
    popd && \
    pushd ~/scripts/cc-cli && \
    yarn upgrade creditcoin-js && yarn build && yarn pack && \
    popd


FROM runtime-base
EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944 9933 9615
ENTRYPOINT [ "/bin/creditcoin-node" ]

COPY --from=cli-builder  --chown=creditcoin:creditcoin /creditcoin-node/creditcoin-js/creditcoin-js-v*.tgz /creditcoin-node/creditcoin-js/
COPY --from=cli-builder  --chown=creditcoin:creditcoin /creditcoin-node/scripts/cc-cli/creditcoin-cli-v*.tgz /creditcoin-node/scripts/cc-cli/
COPY --from=rust-builder --chown=creditcoin:creditcoin /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY --from=rust-builder --chown=creditcoin:creditcoin /creditcoin-node/chainspecs /

USER 0
RUN npm install -g /creditcoin-node/scripts/cc-cli/creditcoin-cli-v*.tgz
USER creditcoin
RUN mkdir /creditcoin-node/data
