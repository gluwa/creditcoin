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

## Usage

### Import

Importing the library into your project:

```typescript
import { creditcoinApi } from 'creditcoin-js';

const { api } = await CreditcoinApi('ws://localhost:9944');

// don't forget to disconnect when you are done
await api.disconnect();
```

### Using the API

The API is a collection of modules that provide access to the various functions of the Creditcoin blockchain.

```typescript
const { api, extrinsics, utils } = await CreditcoinApi('ws://localhost:9944');
```

### Creating transactions

```typescript
const { api } = await CreditcoinApi('ws://localhost:9944');

const tx = api.
    .tx
    .balances
    .transfer(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "1000000000000000"  // CTC amount in microunits
                            // (1 CTC = 1e18 microunits)
    )
```

### Signing & sending

```typescript
import { Keyring } from 'creditcoin-js';

const keyring = new Keyring({ type: 'sr25519' });
const alice = keyring.addFromUri('//Alice');

await tx.signAndSend(alice);
```

### Batching transactions

```typescript
const tx1 = api.tx.balances.transfer(addrBob, 10);
const tx2 = api.tx.balances.transfer(addrCharlie, 10);
const txs = [tx1, tx2];

const batch_tx = api.tx.utility.batch(txs);

await batch_tx.signAndSend(alice);
```

### Registering External Addresses
```typescript
import { personalSignSignature } from 'creditcoin-js/lib/extrinsics/register-address-v2';
import { personalSignAccountId } from 'creditcoin-js/lib/utils';
import { Wallet } from "creditcoin-js";

const { extrinsics: { registerAddressV2 }} = ccApi;

// The ethers wallet that we will be registering
const ethSigner = Wallet.random();
const externalAddress = ethSigner.address;

// Assume creditcoinAddress is a keyring pair
const accountId = creditcoinAddress.addressRaw;

// Create a proof of ownership by signing your creditcoin address with your ethereum private key
const signature = await personalSignAccountId(api, ethSigner, creditcoinAddress);
const proof = personalSignSignature(signature);

// The blockchain that the external address belongs to
const blockchain = "Ethereum";

const result = await registerAddressV2(externalAddress, blockchain, proof, lender);
```

### Swap GCRE -> CTC
```typescript
import { GCREContract } from 'creditcoin-js/lib/extrinsics/request-collect-coins-v2';

const { extrinsics: { requestCollectCoinsV2 } } = ccApi;

// Create a wrapper that holds the details for the burned tokens
// externalAddress is the address of the burner and must be previously registered
const burnDetails = GCREContract(externalAddress, burnTxHash);

// Submit the swap request, adding it to the task queue of the off chain worker
const collectCoins = await requestCollectCoinsV2(burnDetails, creditcoinSigner);

// Wait for the offchain worker to finish processing this request
// Under the hood waitForVerification tracks CollectedCoinsMinted and CollectedCoinsFailedVerification events using the TaskId as a unique key
// 900_000 (milliseconds) comes from an assumed 60 block task timeout deadline and assumed 15 second blocktime (check the constants provided by the runtime in production code)
const collectCoinVerified = await collectCoins.waitForVerification(900_000);
```

### Reading Runtime Constants
```typescript
import { U64, U32 } from "@polkadot/types-codec";

const { api } = ccApi;

const expectedBlockTime = (api.consts.babe.expectedBlockTime as U64).toNumber()
const unverifiedTaskTimeout = (api.consts.creditcoin.unverifiedTaskTimeout as u32).toNumber();

const taskTimeout = expectedBlockTime * unverifiedTaskTimeout;
```


### Setting Offchain Local Storage (Ethereum RPC URI)
```typescript
function u8aToHex = (bytes: Uint8Array): string {
    return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
};

const rpcUri = u8aToHex(api.createType('String', 'http://localhost:8545').toU8a());
await api.rpc.offchain.localStorageSet('PERSISTENT', 'ethereum-rpc-uri', rpcUri);
```

### Submitting Sudo Calls (Set CollectCoins Contract)
```typescript
// The blockchain the contract lives one
const blockchain = "Ethereum";

// Address for Ethereum Gluwa Creditcoin Vesting Token 
const contractAddress = "0xa3EE21C306A700E682AbCdfe9BaA6A08F3820419";

const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
    address: contractAddress
    chain: blockchain,
});

// sudoSigner is a keyring pair with sudo privileges
await api.tx.sudo
    .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
    .signAndSend(sudoSigner, { nonce: -1 });
```

## Development

### Build

To build the project, run the following command from the root directory:

```shell
yarn build
```

### Updating Type definitions

Creditcoin-JS uses actual chain metadata to generate the API types and augmented endpoints. When the Creditcoin blockchain gets updated and includes new extrinsics or storage fields in it’s pallets, Creditcoin-JS must regenerate its types to include the newly available methods.

1. Fetch Chain Metadata

This process begins with pulling the current metadata from a running creditcoin-node by making an RPC call:

```shell
curl -H "Content-Type: application/json" -d '{"id":"1", "jsonrpc":"2.0", "method": "state_getMetadata", "params":[]}' http://localhost:9933
```

2. Add Metadata to creditcoin.json

The full JSON output must be saved into a creditcoin.json file as specified by the generation scripts included in package.json.

3. Generate Types

The types can be generated by running the following command:

```shell
yarn build:types
```

## Errors & Troubleshooting

If after following the build process you run into errors where credicoin-js isn't reflecting the changes in the rust code you may need to clear your cache. The following command (run from root directory) can help:

```shell
cd creditcoin-js && rm -rf lib && yarn install && yarn build && yarn pack && cd ../integration-tests/ && yarn cache clean && rm -rf node_modules && yarn upgrade creditcoin-js && yarn install
```
