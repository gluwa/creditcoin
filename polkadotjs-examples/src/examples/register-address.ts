import { ApiPromise, Keyring, WsProvider, } from '@polkadot/api';
import {handleTransaction, } from '../utils';

export const registerAddress = async () => {
    const provider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: `sr25519` });
    const bob = keyring.addFromUri('//Bob', { name: 'Bob' });

    const unsubscribe : () => void = await api.tx.creditcoin
        .registerAddress('Ethereum', '0x3C6a6762f969B36bb1a6DBD598A5DC9800284D77')
        //the nonce can be set manually when sending a transaction 
        //using {nonce:-1} will get the next nonce, including transactions in the transaction pool
        .signAndSend(bob, { nonce: -1 }, ((result) => handleTransaction(api,unsubscribe,result)));
}
