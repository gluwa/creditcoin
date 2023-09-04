# Developer's Guide

You can find the hosted dev-guide at
<https://gluwa.github.io/creditcoin/>

## Requirements
```bash
cargo install mdbook mdbook-mermaid
```

## Build & view
```bash
# to build book
mdbook build

# to host locally for immediate perusing
mdbook serve
```

Also see
[`.github/workflows/deploy-docs.yml`](https://github.com/gluwa/creditcoin/blob/dev/.github/workflows/deploy-docs.yml)
to see how we build and deploy this mdbook.
