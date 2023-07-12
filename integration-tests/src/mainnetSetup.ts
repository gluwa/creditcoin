import { providers, Keyring, KeyringPair, Wallet } from 'creditcoin-js';
import { default as globalSetup } from './globalSetup';

const createSigner = (keyring: Keyring, who: 'lender' | 'borrower' | 'sudo'): KeyringPair => {
    switch (who) {
        case 'lender':
            const lenderSeed = process.env.LENDER_SEED;
            if (lenderSeed === undefined) {
                throw new Error('LENDER_SEED environment variable is required');
            }
            return keyring.addFromUri(lenderSeed!); // eslint-disable-line
        case 'borrower':
            const borrowerSeed = process.env.BORROWER_SEED;
            if (borrowerSeed === undefined) {
                throw new Error('BORROWER_SEED environment variable is required');
            }
            return keyring.addFromUri(borrowerSeed!); // eslint-disable-line
        case 'sudo':
            const sudoSeed = process.env.SUDO_SEED;
            if (sudoSeed === undefined) {
                throw new Error('SUDO_SEED environment variable is required');
            }
            return keyring.addFromUri(sudoSeed!); // eslint-disable-line
        default:
            throw new Error(`Unexpected value "${who}"`); // eslint-disable-line
    }
};

const createWallet = (who: 'lender' | 'borrower') => {
    // IMPORTANT: both of these wallets start with 0.1 ETH

    const lenderPrivateKey = process.env.LENDER_PRIVATE_KEY;
    if (lenderPrivateKey === undefined) {
        throw new Error('LENDER_PRIVATE_KEY environment variable is required');
    }

    const borrowerPrivateKey = process.env.BORROWER_PRIVATE_KEY;
    if (borrowerPrivateKey === undefined) {
        throw new Error('BORROWER_PRIVATE_KEY environment variable is required');
    }

    const privateKey = who === 'lender' ? lenderPrivateKey : borrowerPrivateKey;
    const provider = new providers.JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);

    return new Wallet(privateKey, provider);
};

const setup = async () => {
    (global as any).CREDITCOIN_API_URL = 'wss://rpc.mainnet.creditcoin.network/ws';
    (global as any).CREDITCOIN_USES_FAST_RUNTIME = false;
    (global as any).CREDITCOIN_CREATE_SIGNER = createSigner;
    (global as any).CREDITCOIN_CREATE_WALLET = createWallet;

    (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL = false;
    (global as any).CREDITCOIN_ETHEREUM_NAME = 'Ethereum';
    const ethereumNodeUrl = process.env.ETHEREUM_NODE_URL;
    if (ethereumNodeUrl === undefined) {
        throw new Error('ETHEREUM_NODE_URL environment variable is required');
    }
    (global as any).CREDITCOIN_ETHEREUM_NODE_URL = ethereumNodeUrl;
    (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET = false;

    (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = false;
    (global as any).CREDITCOIN_NETWORK_LONG_NAME = 'Creditcoin';
    (global as any).CREDITCOIN_NETWORK_SHORT_NAME = 'creditcoin';
    (global as any).CREDITCOIN_METRICS_BASE = 'http://main-rpc-creditcoin-rpc-2.westus.cloudapp.azure.com:9615';
    (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES = true;

    // https://etherscan.io/token/0xa3ee21c306a700e682abcdfe9baa6a08f3820419
    (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS = '0xa3EE21C306A700E682AbCdfe9BaA6A08F3820419';
    // we need a new tx hash every time so we call .burn() in globalSetup()! See ctc-deploy.ts
    (global as any).CREDITCOIN_CTC_BURN_TX_HASH = undefined;

    if (process.env.LENDER_PRIVATE_KEY === undefined) {
        throw new Error('LENDER_PRIVATE_KEY environment variable is required');
    }
    (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY = process.env.LENDER_PRIVATE_KEY;

    await globalSetup();
};

export default setup;
