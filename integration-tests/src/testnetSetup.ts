import { providers, Wallet } from 'ethers';
import { default as globalSetup } from './globalSetup';

const createWallet = (who: 'lender' | 'borrower') => {
    const ethereumNodeUrl = process.env.ETHEREUM_NODE_URL;
    if (ethereumNodeUrl === undefined) {
        throw new Error('ETHEREUM_NODE_URL environment variable is required');
    }

    const lenderPrivateKey = process.env.LENDER_PRIVATE_KEY;
    if (lenderPrivateKey === undefined) {
        throw new Error('LENDER_PRIVATE_KEY environment variable is required');
    }

    const borrowerPrivateKey = process.env.BORROWER_PRIVATE_KEY;
    if (borrowerPrivateKey === undefined) {
        throw new Error('BORROWER_PRIVATE_KEY environment variable is required');
    }

    const privateKey = who === 'lender' ? lenderPrivateKey : borrowerPrivateKey;
    const provider = new providers.JsonRpcProvider(ethereumNodeUrl);

    return new Wallet(privateKey, provider);
};

const setup = async () => {
    (global as any).CREDITCOIN_API_URL = 'wss://testnet.creditcoin.network';
    (global as any).CREDITCOIN_CREATE_WALLET = createWallet;
    (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = false;
    (global as any).CREDITCOIN_NETWORK_LONG_NAME = 'Creditcoin Testnet';
    (global as any).CREDITCOIN_NETWORK_SHORT_NAME = 'creditcoin_testnet';
    (global as any).CREDITCOIN_METRICS_BASE = 'http://cctn-rpc.francecentral.azurecontainer.io:9615';

    await globalSetup();
};

export default setup;
