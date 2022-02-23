#Build the image and tag it creditcoin/ci-linux:production
FROM debian:bullseye-slim
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    build-essential \
    clang \
    libclang-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*