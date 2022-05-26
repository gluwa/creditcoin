import { setupAuthority } from 'creditcoin-js/examples/setup-authority';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

const setup = async () => {
    process.env.NODE_ENV = 'test';

    const api = await ApiPromise.create({
        provider: new WsProvider((global as any).CREDITCOIN_API_URL),
    });
    const alice = new Keyring({ type: 'sr25519' }).addFromUri('//Alice');
    await setupAuthority(api, alice);
    await api.disconnect();
};

export default setup;
