# Legacy Account Migration

## How to check an accounts current balance and other information

Account balances migrated from Creditcoin 1.7 at block 1,123,966. Your starting balance on Creditcoin 2.0 should match your Creditcion 1.7 block 1,123,966 balance

In order to check your Creditcoin 2.0 balance you can run the following command:

   ``` shell
   docker run jacogr/polkadot-js-tools api --ws wss://<rpc-node> query.system.account <SS58 Address>
   ```

## How to migrate balances that couldn't be automatically migrated

Some accounts were not automatically migrated because we were unable to match a `sighash` with a `public key`. To check if an account has an un-migrated balance, you can run the following command:

``` shell
docker run jacogr/polkadot-js-tools api --ws wss://<rpc-node> query.creditcoin.legacyWallets <Legacy Sighash>
```

If a legacy wallet with a balance is found this balance can be claimed using the following command:

``` shell
docker run jacogr/polkadot-js-tools api --ws wss://<rpc-node> tx.creditcoin.claimLegacyWallet <Public Key (hex)> --seed <private key or seed phrase> --sign ecdsa
```
