import { BN, Keyring, creditcoinApi } from 'src';

export const transferExample = async () => {
    // Connect to a local node
    const { api } = await creditcoinApi('ws://localhost:9944');

    // Create a keyring with Alice
    const alice = new Keyring({
        type: 'sr25519',
    }).addFromUri('//Alice');

    // Must enter amount in microunits
    const microunitsPerCTC = new BN('1000000000000000000'); // 1 CTC = 10^18 microunits
    const amount = new BN('1').mul(microunitsPerCTC);

    // Create each transfer transaction
    const bobAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const bobTx = api.tx.balances.transfer(bobAddress, amount.toString());

    const charlieAddress = '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y';
    const charlieTx = api.tx.balances.transfer(charlieAddress, amount.toString());

    // Create a batch transaction
    // The whole transaction will rollback and fail if any of the calls failed
    const batchTx = api.tx.utility.batchAll([bobTx, charlieTx]);

    // Sign and send the transaction
    const hash = await batchTx.signAndSend(alice);

    // Log the hash
    console.log('Batch TX sent with hash', hash.toHex());

    // Disconnect from the local node
    await api.disconnect();
};

if (require.main === module) {
    transferExample().catch(console.error);
}
