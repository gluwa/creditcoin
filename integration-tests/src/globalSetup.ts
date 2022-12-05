import { ApiPromise, WsProvider, Wallet, Keyring, KeyringPair, CHAINS } from 'creditcoin-js';
import { setupAuthority } from 'creditcoin-js/lib/examples/setup-authority';
import { main as deployCtcContract } from './ctc-deploy';

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

const setup = async () => {
    process.env.NODE_ENV = 'test';

    if ((global as any).CREDITCOIN_CREATE_WALLET === undefined) {
        (global as any).CREDITCOIN_CREATE_WALLET = Wallet.createRandom; // eslint-disable-line
    }

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

    if ((global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL = true;
    }

    if ((global as any).CREDITCOIN_ETHEREUM_CHAIN === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_CHAIN = CHAINS.hardhat;
    }

    if ((global as any).CREDITCOIN_ETHEREUM_NODE_URL === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_NODE_URL = 'http://localhost:8545';
    }

    if ((global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET === undefined) {
        (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET = true;
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
    await deployCtcContract((global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS);
    (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS = process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS;

    if ((global as any).CREDITCOIN_CTC_BURN_TX_HASH === undefined) {
        // Note: burn is always called inside deployCtcContract() !!!
        (global as any).CREDITCOIN_CTC_BURN_TX_HASH = process.env.CREDITCOIN_CTC_BURN_TX_HASH;
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
