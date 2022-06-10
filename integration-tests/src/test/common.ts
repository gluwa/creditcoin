import { Wallet } from 'ethers';
import { Guid } from 'js-guid';
import { BN } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';
import { Option } from '@polkadot/types';
import { PalletCreditcoinAddress } from '@polkadot/types/lookup';
import { Blockchain, LoanTerms, DealOrderId } from 'creditcoin-js/model';
import { CreditcoinApi } from 'creditcoin-js/types';
import { createAddress } from 'creditcoin-js/transforms';
import { ethConnection } from 'creditcoin-js/examples/ethereum';
import { AddressRegistered, createAddressId } from 'creditcoin-js/extrinsics/register-address';
import { Keyring } from '@polkadot/api';

type TestData = {
    loanTerms: LoanTerms;
    blockchain: Blockchain;
    expirationBlock: number;
    keyring: Keyring;
    createWallet: (who: string) => Wallet;
};
export const testData: TestData = {
    loanTerms: {
        amount: new BN(1_000),
        interestRate: {
            ratePerPeriod: 100,
            decimals: 4,
            period: {
                secs: 60 * 60 * 24,
                nanos: 0,
            },
            interestType: 'Simple',
        },
        termLength: {
            secs: 60 * 60 * 24 * 30,
            nanos: 0,
        },
    } as LoanTerms,
    blockchain: (global as any).CREDITCOIN_ETHEREUM_NAME as Blockchain,
    expirationBlock: 10_000_000,
    createWallet: (global as any).CREDITCOIN_CREATE_WALLET
        ? (global as any).CREDITCOIN_CREATE_WALLET
        : Wallet.createRandom, // eslint-disable-line
    keyring: new Keyring({ type: 'sr25519' }),
};

export const addAskAndBidOrder = async (ccApi: CreditcoinApi, lender: KeyringPair, borrower: KeyringPair) => {
    const {
        extrinsics: { addAskOrder, addBidOrder, registerAddress },
        utils: { signAccountId },
    } = ccApi;

    const { blockchain, expirationBlock, loanTerms } = testData;
    const lenderWallet = Wallet.createRandom();
    const borrowerWallet = Wallet.createRandom();

    const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
        registerAddress(lenderWallet.address, blockchain, signAccountId(lenderWallet, lender.address), lender),
        registerAddress(borrowerWallet.address, blockchain, signAccountId(borrowerWallet, borrower.address), borrower),
    ]);
    const askGuid = Guid.newGuid();
    const bidGuid = Guid.newGuid();

    const [askOrderAdded, bidOrderAdded] = await Promise.all([
        addAskOrder(lenderRegAddr.itemId, loanTerms, expirationBlock, askGuid, lender),
        addBidOrder(borrowerRegAddr.itemId, loanTerms, expirationBlock, bidGuid, borrower),
    ]);

    return [askOrderAdded.itemId, bidOrderAdded.itemId];
};

export const lendOnEth = async (
    lenderWallet: Wallet,
    borrowerWallet: Wallet,
    dealOrderId: DealOrderId,
    loanTerms: LoanTerms,
) => {
    const { lend, waitUntilTip } = await ethConnection(
        (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
        (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET ? undefined : lenderWallet,
        (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
    );

    // Lender lends to borrower on ethereum
    const [tokenAddress, lendTxHash, lendBlockNumber] = await lend(
        lenderWallet,
        borrowerWallet.address,
        dealOrderId[1],
        loanTerms.amount,
    );

    // wait 15 blocks on Ethereum
    await waitUntilTip(lendBlockNumber + 15);

    return [tokenAddress, lendTxHash];
};

export const tryRegisterAddress = async (
    ccApi: CreditcoinApi,
    externalAddress: string,
    blockchain: Blockchain,
    ownershipProof: string,
    signer: KeyringPair,
    checkForExisting = false,
): Promise<AddressRegistered> => {
    const {
        api,
        extrinsics: { registerAddress },
    } = ccApi;

    if (checkForExisting) {
        const existingAddressId = createAddressId(blockchain, externalAddress);
        const result = await api.query.creditcoin.addresses<Option<PalletCreditcoinAddress>>(existingAddressId);

        if (result.isSome) {
            return {
                itemId: existingAddressId,
                item: createAddress(result.unwrap()),
            } as AddressRegistered;
        }
    }

    return registerAddress(externalAddress, blockchain, ownershipProof, signer);
};
