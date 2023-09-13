import { Wallet } from 'ethers';
import { Guid } from 'js-guid';

import { ApiPromise, Keyring } from '@polkadot/api';
import { Option } from '@polkadot/types';
import { BN } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';
import { PalletCreditcoinAddress } from '@polkadot/types/lookup';

import { Blockchain, LoanTerms, DealOrderId } from '../model';
import { CreditcoinApi } from '../types';
import { createAddress } from '../transforms';
import { EthConnection } from '../examples/ethereum';
import { AddressRegistered, createAddressId } from '../extrinsics/register-address';

type CreateWalletFunc = (who: string) => Wallet;

export type TestData = {
    blockchain: Blockchain;
    expirationBlock: number;
    keyring: Keyring;
    createWallet: CreateWalletFunc;
    loanTerms: LoanTerms;
};

export const testData = (ethereumChain: Blockchain, createWalletF: CreateWalletFunc): TestData => {
    return {
        blockchain: ethereumChain,
        expirationBlock: 10_000_000,
        createWallet: createWalletF,
        keyring: new Keyring({ type: 'sr25519' }),
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
        },
    };
};

export const addAskAndBidOrder = async (
    ccApi: CreditcoinApi,
    lender: KeyringPair,
    borrower: KeyringPair,
    loanTerms: LoanTerms,
    testingData: TestData,
    checkForExistingAddress = false,
) => {
    const {
        extrinsics: { addAskOrder, addBidOrder },
        utils: { signAccountId },
    } = ccApi;

    const { blockchain, expirationBlock, createWallet } = testingData;
    const lenderWallet = createWallet('lender');
    const borrowerWallet = createWallet('borrower');

    const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
        tryRegisterAddress(
            ccApi,
            lenderWallet.address,
            blockchain,
            signAccountId(lenderWallet, lender.address),
            lender,
            checkForExistingAddress,
        ),
        tryRegisterAddress(
            ccApi,
            borrowerWallet.address,
            blockchain,
            signAccountId(borrowerWallet, borrower.address),
            borrower,
            checkForExistingAddress,
        ),
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
    connection: EthConnection,
) => {
    const { lend, waitUntilTip } = connection;

    // Lender lends to borrower on ethereum
    const [, lendTxHash, lendBlockNumber] = await lend(
        lenderWallet,
        borrowerWallet.address,
        dealOrderId[1],
        loanTerms.amount,
    );

    // wait 15 blocks on Ethereum
    await waitUntilTip(lendBlockNumber + 15);

    return lendTxHash;
};

export const checkAddress = async (
    ccApi: CreditcoinApi,
    existingAddressId: string,
): Promise<AddressRegistered | undefined> => {
    const { api } = ccApi;

    const result = await api.query.creditcoin.addresses<Option<PalletCreditcoinAddress>>(existingAddressId);

    if (result.isSome) {
        return {
            itemId: existingAddressId,
            item: createAddress(result.unwrap()),
        } as AddressRegistered;
    }

    return undefined;
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
        extrinsics: { registerAddress },
    } = ccApi;

    if (checkForExisting) {
        const existingAddressId = createAddressId(blockchain, externalAddress);
        const result = await checkAddress(ccApi, existingAddressId);
        if (result) {
            return result;
        }
    }

    return registerAddress(externalAddress, blockchain, ownershipProof, signer);
};

export const getCreditcoinBlockNumber = async (api: ApiPromise): Promise<number> => {
    const response = await api.rpc.chain.getBlock();
    return response.block.header.number.toNumber();
};

export const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

// wait until a certain amount of blocks have elapsed
export const forElapsedBlocks = async (api: ApiPromise, opts?: { minBlocks?: number; maxRetries?: number }) => {
    const { maxRetries = 10, minBlocks = 2 } = opts ?? {};
    const initialCreditcoinBlockNumber = await getCreditcoinBlockNumber(api);

    let retriesCount = 0;
    let creditcoinBlockNumber = await getCreditcoinBlockNumber(api);

    // wait a min amount of blocks since the initial call to give time to any pending
    // transactions, e.g. test setup to make it into a block
    while (retriesCount < maxRetries && creditcoinBlockNumber <= initialCreditcoinBlockNumber + minBlocks) {
        await sleep(5000);
        creditcoinBlockNumber = await getCreditcoinBlockNumber(api);
        retriesCount++;
    }
};
