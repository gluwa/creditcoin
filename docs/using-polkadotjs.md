# Using PolkadotJs with Creditcoin

## Install Dependencies

Install PolkadotJs API:

``` shell
yarn add @polkadot/api
```

## Generating Types from Metadata

If you want to generate types from the chain's metadata you can do the following (adapted from [PolkadotJs Typegen Example](https://polkadot.js.org/docs/api/examples/promise/typegen)):

Install `@polkadot/typegen` and `ts-node`:

``` shell
yarn add -D @polkadot/typegen ts-node
```

To download the chain's metadata:

``` shell
curl -H "Content-Type: application/json" -d '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' https://<mainnet | testnet>.creditcoin.network
```

This will return something like:

``` json
{"jsonrpc":"2.0","result":"0x6d6574610b6c185379737....","id":1}
```

Copy the output to `creditcoin.json`.

In your `src` folder, create an `interfaces` folder and place an empty `definitions.ts` file in it.

In your `packages.json` add the following to the `scripts` section:

``` json
"scripts": {
    "build:types": "yarn generate:defs && yarn generate:meta",
    "generate:defs": "ts-node --skip-project node_modules/.bin/polkadot-types-from-defs --package <your project name>/interfaces --input ./src/interfaces --endpoint ./creditcoin.json",
    "generate:meta": "ts-node --skip-project node_modules/.bin/polkadot-types-from-chain --package <your project name>/interfaces  --endpoint ./creditcoin.json --output ./src/interfaces"
  }
```

Add the following to your `tsconfig.json` `compilerOptions` section:

``` json
"paths": {
    "@polkadot/api/augment": [
    "./src/interfaces/augment-api.ts"
    ],
    "@polkadot/types/augment": [
    "./src/interfaces/augment-types.ts"
    ],
    "@polkadot/types/lookup": [
    "./src/interfaces/types-lookup.ts"
    ],
}
```

To generate the types run:

``` shell
yarn build:types
```

## Code Examples

Many of these examples are adapted from the [PolkadotJs API](https://polkadot.js.org/docs/api).

### Query basic chain information

``` javascript
import { ApiPromise, WsProvider, } from '@polkadot/api';

const main = async () => {
  // Initialise the provider to connect to the either mainnet or testnet rpc node
  const provider = new WsProvider('wss://<mainnet | testnet>.creditcoin.network');

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);

  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

main().catch(console.error);
```

### Subscribe and unsubscribe to new blocks

``` javascript
import { ApiPromise, WsProvider, } from '@polkadot/api';

const main = async () => {
  // Initialise the provider to connect to the either mainnet or testnet rpc node
  const provider = new WsProvider('wss://<mainnet | testnet>.creditcoin.network');

  // Create the API and wait until ready
  const api = await ApiPromise.create({ provider });

  // Subscribe to chain updates and log the current block number on update.
  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    console.log(`Chain is at block: #${header.number}`);
  });

  // In this example we're calling the unsubscribe() function that is being
  // returned by the api call function after 2 minutes.
  setTimeout(() => {
    unsubscribe();
    console.log('Unsubscribed');
  }, 120000);
}

main().catch(console.error);
```

### More Examples

More examples can be found in the [creditcoin-js](../creditcoin-js) project.
