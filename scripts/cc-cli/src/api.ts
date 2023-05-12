
import { ApiPromise, WsProvider } from 'creditcoin-js';
import { cryptoWaitReady } from '@polkadot/util-crypto';
// Create new API instance
export async function newApi(url: string) {
    if (!url) {
        url = 'ws://localhost:9944';
    }
    // console.log("Connecting to node at:", url);
    const api = await ApiPromise.create({
        provider: new WsProvider(url)
    });
    cryptoWaitReady();
    return api;
}
