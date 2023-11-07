import { ApiPromise, WsProvider, Keyring, KeyringPair, Wallet, POINT_01_CTC } from 'creditcoin-js';
import { setupAuthority } from 'creditcoin-js/lib/examples/setup-authority';
import { deployCtcContract } from 'creditcoin-js/lib/ctc-deploy';
import { GluwaCreditVestingToken } from 'creditcoin-js/lib/examples/ctc/typechain';

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

declare global {
    var CREDITCOIN_CTC_CONTRACT: GluwaCreditVestingToken;
}

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
        const wsPort = process.env.CREDITCOIN_WS_PORT || '9944';
        (global as any).CREDITCOIN_API_URL = `ws://127.0.0.1:${wsPort}`;
    }

    if ((global as any).CREDITCOIN_METRICS_BASE === undefined) {
        const metricsPort = process.env.CREDITCOIN_METRICS_PORT || '9615';
        (global as any).CREDITCOIN_METRICS_BASE = `http://127.0.0.1:${metricsPort}`;
    }

    if ((global as any).CREDITCOIN_USES_FAST_RUNTIME === undefined) {
        (global as any).CREDITCOIN_USES_FAST_RUNTIME = true;
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

    if ((global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES === undefined) {
        (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES = true;
    }

    if ((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY === undefined) {
        // Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
        (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY =
            '0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe';
    }

    // Note: in case address is defined will attach to already deployed contract
    const contract = await deployCtcContract(
        (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
        (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
        (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY,
    );
    global.CREDITCOIN_CTC_CONTRACT = contract;
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
