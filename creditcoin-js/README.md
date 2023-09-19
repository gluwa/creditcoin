# creditcoin-js

## Getting started

### Preqrequisites

Creditcoin-js requires the following to be installed:

-   [Node.js](https://nodejs.org/en/)
-   [TypeScript](https://www.typescriptlang.org/)

### Install

Adding Creditcoin-JS to your project is easy. Install it by using your favorite package manager:

```shell
yarn add creditcoin-js
```

This will install the latest release version, which should allow you to interact with Creditcoin's main network and your own local chains that use the latest Creditcoin binaries.

## Errors & Troubleshooting

If after following the build process you run into errors where credicoin-js isn't reflecting the changes in the rust code you may need to clear your cache. The following command (run from root directory) can help:

```shell
cd creditcoin-js && rm -rf lib && yarn install && yarn build && yarn pack && cd ../integration-tests/ && yarn cache clean && rm -rf node_modules && yarn upgrade creditcoin-js && yarn install
```
