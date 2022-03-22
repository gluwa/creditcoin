import { ApiPromise, Keyring, WsProvider, } from '@polkadot/api';

export const registerAddress = async () => {
    const provider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: `sr25519` });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob' });

    const unsubscribe = await api.tx.creditcoin
        .registerAddress('Ethereum', '0x3C6a6762f969B36bb1a6DBD598A5DC9800284D77')
        //the nonce can be set manually when sending a transaction 
        //using {nonce:-1} will get the next nonce, including transactions in the transaction pool
        .signAndSend(bob, { nonce: -1 }, (({ status, events }) => {
            console.log(`current status is ${status}`);
            if (status.isInBlock) {
                events.forEach(({ event }) => {
                    const types = event.typeDef;
                    event.data.forEach((data, index) => {
                        console.log(`pallet: ${event.section} event name: ${event.method}`)
                        console.log("event types", types[index].type + '; event data:' + data.toString());
                    });
                });
                unsubscribe();
            }
        }))
}
