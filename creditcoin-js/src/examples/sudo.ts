import { BN, Keyring, creditcoinApi } from 'src';

export const transferExample = async () => {
    // Connect to a local node
    const { api } = await creditcoinApi('ws://localhost:9944');

    // Create a keyring with the Sudo account
    // Default SUDO for local devnets is Alice
    const alice = new Keyring({
        type: 'sr25519',
    }).addFromUri('//Alice');

    // Create a transaction to set the balance of Bob

    // Must enter amount in microunits
    const microunitsPerCTC = new BN('1000000000000000000'); // 1 CTC = 10^18 microunits
    const freeAmount = new BN('123').mul(microunitsPerCTC);
    const bondedAmount = new BN('456').mul(microunitsPerCTC);

    // Create a set balance TX
    // It will set the balance of Bob to 123 CTC and 456 bonded CTC
    const bobAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const setBalanceTx = api.tx.balances.setBalance(bobAddress, freeAmount.toString(), bondedAmount.toString());

    // Wrap it in a sudo transaction
    const sudoTx = api.tx.sudo.sudo(setBalanceTx);

    // Sign and send the sudo transaction
    const hash = await sudoTx.signAndSend(alice);

    // Log the hash
    console.log('Transfer sent with hash', hash.toHex());

    // Disconnect from the local node
    await api.disconnect();
};

if (require.main === module) {
    transferExample().catch(console.error);
}
