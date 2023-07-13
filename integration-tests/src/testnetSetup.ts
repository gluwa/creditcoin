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
        default:
            throw new Error(`Unexpected value "${who}"`); // eslint-disable-line
    }
};

const createWallet = (who: 'lender' | 'borrower') => {
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
    (global as any).CREDITCOIN_SWITCH_TO_POS_ALREADY_CALLED = true;
    (global as any).CREDITCOIN_API_URL = 'wss://rpc.testnet.creditcoin.network/ws';
    (global as any).CREDITCOIN_USES_FAST_RUNTIME = false;
    (global as any).CREDITCOIN_CREATE_SIGNER = createSigner;
    (global as any).CREDITCOIN_CREATE_WALLET = createWallet;

    (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL = false;
    (global as any).CREDITCOIN_ETHEREUM_NAME = 'Rinkeby';
    const ethereumNodeUrl = process.env.ETHEREUM_NODE_URL;
    if (ethereumNodeUrl === undefined) {
        throw new Error('ETHEREUM_NODE_URL environment variable is required');
    }
    (global as any).CREDITCOIN_ETHEREUM_NODE_URL = ethereumNodeUrl;
    (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET = false;

    (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = false;
    (global as any).CREDITCOIN_NETWORK_LONG_NAME = 'Creditcoin PoS Testnet';
    (global as any).CREDITCOIN_NETWORK_SHORT_NAME = 'creditcoin_pos_testnet';
    (global as any).CREDITCOIN_METRICS_BASE = 'http://test-rpc-creditcoin-rpc-2.eastus.cloudapp.azure.com:9615';
    (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES = true;

    // This is on Goerli, https://goerli.etherscan.io/address/0x833cc7c2598D80d327767De33B22ac426f4248e2
    (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS = '0x833cc7c2598D80d327767De33B22ac426f4248e2';
    // we need a new tx hash every time so we call .burn() in globalSetup()! See ctc-deploy.ts
    (global as any).CREDITCOIN_CTC_BURN_TX_HASH = undefined;

    if (process.env.LENDER_PRIVATE_KEY === undefined) {
        throw new Error('LENDER_PRIVATE_KEY environment variable is required');
    }
    (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY = process.env.LENDER_PRIVATE_KEY;

    await globalSetup();
};

export default setup;
