import { BN, Keyring, creditcoinApi } from 'src';

export const transferExample = async () => {
    // Connect to a local node
    const { api } = await creditcoinApi((global as any).CREDITCOIN_API_URL);

    // Create a keyring with Alice
    const alice = new Keyring({
        type: 'sr25519',
    }).addFromUri('//Alice');

    const bobAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

    // Must enter amount in microunits
    const microunitsPerCTC = new BN('1000000000000000000'); // 1 CTC = 10^18 microunits
    const amount = new BN('1').mul(microunitsPerCTC);

    // Create a transfer transaction
    const tx = api.tx.balances.transfer(bobAddress, amount.toString());

    // Sign and send the transaction
    const hash = await tx.signAndSend(alice);

    // Log the hash
    console.log('Transfer sent with hash', hash.toHex());

    // Disconnect from the local node
    await api.disconnect();
};

if (require.main === module) {
    transferExample().catch(console.error);
}
