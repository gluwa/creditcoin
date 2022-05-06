import { setupAuthority } from 'creditcoin-js/examples/setup-authority';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

const setup = async () => {
    const api = await ApiPromise.create({
        provider: new WsProvider('ws://127.0.0.1:9944'),
    });
    const alice = new Keyring({ type: 'sr25519' }).addFromUri('//Alice');
    await setupAuthority(api, alice);
    await api.disconnect();
};

export default setup;
