
# Building the `creditcoin-node` from source

_Note on development platforms: development is easiest from a unix environment (whether that be linux, macOS, or WSL). It should be possible to
develop natively on Windows, [however that is unsupported](https://github.com/gluwa/creditcoin/security/advisories/GHSA-cx5c-xwcv-vhmq)._

## Build prerequisites

### Install the Rust toolchain

You'll need a working Rust installation, if you don't have one already available (NOTE: you'll want to install and manage your Rust toolchain with `rustup`,
not by installing your distro's `rust` package which will most likely be too old).

For the installation you can refer to [these instructions](https://www.rust-lang.org/tools/install).

Once you have a working rust installation, you'll need to add the `wasm32-unknown-unknown` target and install the nightly toolchain

```bash
rustup update
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --nightly
```

Note: So that you don't have to specify the toolchain every build, you can set `nightly` as your default toolchain while working on creditcoin.

You can also check
[./ci/env](https://github.com/gluwa/creditcoin/blob/dev/ci/env)
for the toolchain being used in our CI in case you run into issues with newer version before we notice them.

```bash
# set nightly as default for your creditcoin checkout
rustup override set nightly
```

### System build dependencies

You'll need a few system dependencies (some extra dependencies may be required depending on the platform, but
all platforms require the following):

- Clang
- Protobuf compiler
- CMake
- OpenSSL

#### MacOS setup

macOS comes with a suitable `clang` installation, so you don't need to worry about it.
For installing the remaining dependencies the easiest method is using [homebrew](https://brew.sh).

Assuming you have homebrew installed, you can run the following in the terminal:

```bash
brew update
brew install cmake openssl protobuf
```

#### Ubuntu/Debian setup

Run the following in the terminal

```bash
sudo apt update
# May prompt for location information
sudo apt install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl protobuf-compiler
```

## Clone the creditcoin repo

If you haven't already, you'll want to clone the creditcoin repo and `cd` into the resulting clone:

```bash
git clone https://github.com/gluwa/creditcoin
cd creditcoin
```

## Build the `creditcoin-node`

You should now be able to build the `creditcoin-node`. It's a normal Rust project, so you can perform a release build
(99% of the time you'll want a release build, a debug build is often too slow to be useful).

```bash
cargo build --release
```

Assuming the build completed without errors, you should have a `creditcoin-node` binary in your target directory.
To make sure the binary works, you can try taking a look at the `--help`:

```bash
./target/release/creditcoin-node --help
```
