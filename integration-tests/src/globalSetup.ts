import { setupAuthority } from 'creditcoin-js/examples/setup-authority';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

const setup = async () => {
    process.env.NODE_ENV = 'test';

    if (!(global as any).CREDITCOIN_API_URL) {
        (global as any).CREDITCOIN_API_URL = 'ws://127.0.0.1:9944';
    }

    if (!(global as any).CREDITCOIN_METRICS_BASE) {
        (global as any).CREDITCOIN_METRICS_BASE = 'http://127.0.0.1:9615';
    }

    if (!(global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
        (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = true;
    }

    const api = await ApiPromise.create({
        provider: new WsProvider((global as any).CREDITCOIN_API_URL),
    });
    if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
        const alice = new Keyring({ type: 'sr25519' }).addFromUri('//Alice');
        await setupAuthority(api, alice);
    }
    await api.disconnect();
};

export default setup;
