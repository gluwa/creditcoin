import { default as globalSetup } from './globalSetup';

const setup = async () => {
    (global as any).CREDITCOIN_API_URL = 'wss://testnet.creditcoin.network';
    (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = false;
    (global as any).CREDITCOIN_NETWORK_LONG_NAME = 'Creditcoin Testnet';
    (global as any).CREDITCOIN_NETWORK_SHORT_NAME = 'creditcoin_testnet';
    (global as any).CREDITCOIN_METRICS_BASE = 'http://cctn-rpc.francecentral.azurecontainer.io:9615';

    await globalSetup();
};

export default setup;
