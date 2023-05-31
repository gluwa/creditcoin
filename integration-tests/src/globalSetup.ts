import { ApiPromise, WsProvider, Keyring, KeyringPair, Wallet, POINT_01_CTC } from 'creditcoin-js';
import { performTransfer } from 'creditcoin-js/lib/examples/ethereum';
import { setupAuthority } from 'creditcoin-js/lib/examples/setup-authority';
import { main as deployCtcContract } from './ctc-deploy';
import { JsonRpcProvider } from '@ethersproject/providers';

const createSigner = (keyring: Keyring, who: 'lender' | 'borrower' | 'sudo'): KeyringPair => {
    switch (who) {
        case 'lender':
            return keyring.addFromUri('//Alice');
        case 'borrower':
            return keyring.addFromUri('//Bob');
        case 'sudo':
            return keyring.addFromUri('//Alice');
        default:
            throw new Error(`Unexpected value "${who}"`); // eslint-disable-line
    }
};

// provide fixed wallets, which are constant during the same test session
// because we'll need to fund them with fake $G-CRE
const createWallet = (who: 'lender' | 'borrower') => {
    const privateKeys = {
        // Private key for Account #1: from gluwa/hardhat-dev (10000 ETH)
        'lender': '0xdcb7118c9946a39cd40b661e0d368e4afcc3cc48d21aa750d8164ca2e44961c4',
        // Private key for Account #2: from gluwa/hardhat-dev (10000 ETH)
        'borrower': '0x2d7aaa9b78d759813448eb26483284cd5e4344a17dede2ab7f062f0757113a28',
    }

    const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
    return new Wallet(privateKeys[who] , provider);
};

const setup = async () => {
    process.env.NODE_ENV = 'test';

    if ((global as any).CREDITCOIN_CREATE_SIGNER === undefined) {
        (global as any).CREDITCOIN_CREATE_SIGNER = createSigner; // eslint-disable-line
    }

    // WARNING: when setting global variables `undefined' means no value has been assigned
    // to this variable up to now so we fall-back to the defaults.
    // WARNING: don't change the comparison expression here b/c some variables are actually
    // configured to have a true or false value in different environments!

    if ((global as any).CREDITCOIN_API_URL === undefined) {
        (global as any).CREDITCOIN_API_URL = 'ws://127.0.0.1:9944';
    }

    if ((global as any).CREDITCOIN_MINIMUM_TXN_FEE === undefined) {
        (global as any).CREDITCOIN_MINIMUM_TXN_FEE = POINT_01_CTC;
    }

    if ((global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL = true;
    }
    if ((global as any).CREDITCOIN_ETHEREUM_CHAIN === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_CHAIN = 'Ethereum';
    }

    if ((global as any).CREDITCOIN_ETHEREUM_NODE_URL === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_NODE_URL = 'http://127.0.0.1:8545';
    }

    if ((global as any).CREDITCOIN_CREATE_WALLET === undefined) {
        (global as any).CREDITCOIN_CREATE_WALLET = createWallet;
    }

    if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY === undefined) {
        (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY = true;
    }

    if ((global as any).CREDITCOIN_NETWORK_LONG_NAME === undefined) {
        (global as any).CREDITCOIN_NETWORK_LONG_NAME = 'Development';
    }

    if ((global as any).CREDITCOIN_NETWORK_SHORT_NAME === undefined) {
        (global as any).CREDITCOIN_NETWORK_SHORT_NAME = 'dev';
    }

    if ((global as any).CREDITCOIN_METRICS_BASE === undefined) {
        (global as any).CREDITCOIN_METRICS_BASE = 'http://127.0.0.1:9615';
    }

    if ((global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES === undefined) {
        (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES = false;
    }

    if ((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY === undefined) {
        // Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
        (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY =
            '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';
    }

    // Note: in case address is defined will attach to already deployed contract
    (global as any).CREDITCOIN_CTC_TOKEN = await deployCtcContract((global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS);
    (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS = process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS;

    if ((global as any).CREDITCOIN_CTC_BURN_TX_HASH === undefined) {
        // Note: burn is always called inside deployCtcContract() !!!
        (global as any).CREDITCOIN_CTC_BURN_TX_HASH = process.env.CREDITCOIN_CTC_BURN_TX_HASH;
    }

    // fund lender & borrower on hardhat/Ethereum testnet with fake $G-CRE from the
    // contract that was deployed above so they can have tokens to lend/repay
    if ((global as any).CREDITCOIN_FUND_WITH_GCRE_ON_ETHEREUM !== false) {
        const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
        const deployer = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
        // ^^^ this is the same wallet that had just deployed the ctcToken

        const lender = createWallet('lender');
        const borrower = createWallet('borrower');
        console.log('**** CONFIG ******');
        console.log('**** deloyer=', deployer);
        console.log('**** lender=', lender);
        console.log('**** borrower=', borrower);

        await performTransfer((global as any).CREDITCOIN_CTC_TOKEN, deployer, lender.address, 1_000_000);

        await performTransfer((global as any).CREDITCOIN_CTC_TOKEN, deployer, borrower.address, 1_000_000);
    }

    const api = await ApiPromise.create({
        provider: new WsProvider((global as any).CREDITCOIN_API_URL),
    });
    if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
        const keyring = new Keyring({ type: 'sr25519' });
        const sudo = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'sudo');
        await setupAuthority(api, sudo);
    }
    await api.disconnect();
};

export default setup;
