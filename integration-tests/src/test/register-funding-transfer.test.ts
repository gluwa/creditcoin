import { creditcoinApi, BN, KeyringPair, TransferKind, Guid, Wallet, POINT_01_CTC } from 'creditcoin-js';
import { signLoanParams } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi, VerificationError } from 'creditcoin-js/lib/types';
import { createCreditcoinTransferKind } from 'creditcoin-js/lib/transforms';
import { testData, lendOnEth, tryRegisterAddress, loanTermsWithCurrency } from 'creditcoin-js/lib/testUtils';
import { ethConnection, testCurrency } from 'creditcoin-js/lib/examples/ethereum';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';

import { extractFee } from '../utils';

const ethless: TransferKind = {
    platform: 'Evm',
    kind: 'Ethless',
};

describe('RegisterFundingTransfer', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;
    let lenderRegAddr: AddressRegistered;
    let borrowerRegAddr: AddressRegistered;

    const { blockchain, expirationBlock, createWallet, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    const setup = async () => {
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const eth = await ethConnection(
            (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
            (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
            (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET ? undefined : lenderWallet,
        );
        const currency = testCurrency(eth.testTokenAddress);
        const loanTerms = await loanTermsWithCurrency(ccApi, currency);
        const signedParams = signLoanParams(ccApi.api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        const dealOrder = await ccApi.extrinsics.registerDealOrder(
            lenderRegAddr.itemId,
            borrowerRegAddr.itemId,
            loanTerms,
            expirationBlock,
            askGuid,
            bidGuid,
            borrower.publicKey,
            signedParams,
            lender,
        );
        return { eth, loanTerms, dealOrder };
    };

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        borrower = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const {
            utils: { signAccountId },
        } = ccApi;
        lenderWallet = createWallet('lender');
        borrowerWallet = createWallet('borrower');
        [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            tryRegisterAddress(
                ccApi,
                lenderWallet.address,
                blockchain,
                signAccountId(lenderWallet, lender.address),
                lender,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
            tryRegisterAddress(
                ccApi,
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
        ]);
    }, 900000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { dealOrder, loanTerms, eth } = await setup();
        const txHash = await lendOnEth(lenderWallet, borrowerWallet, dealOrder.dealOrder.itemId, loanTerms, eth);
        const { api } = ccApi;
        const ccTransferKind = createCreditcoinTransferKind(api, ethless);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerFundingTransfer(ccTransferKind, dealOrder.dealOrder.itemId, txHash)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 300000);

    it('emits a failure event if transfer is invalid', async (): Promise<void> => {
        const { dealOrder, loanTerms, eth } = await setup();

        // wrong amount
        const badLoanTerms = { ...loanTerms, amount: new BN(1) };
        const dealOrderId = dealOrder.dealOrder.itemId;

        const failureTxHash = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            badLoanTerms,
            eth,
        );

        const { waitForVerification } = await ccApi.extrinsics.registerFundingTransfer(
            ethless,
            dealOrderId,
            failureTxHash,
            lender,
        );

        console.log('waiting for verification');
        try {
            await waitForVerification(120000);
        } catch (error) {
            expect(error).toBeInstanceOf(VerificationError);
            if (error instanceof VerificationError) {
                expect(error.cause).toBeDefined();
                if (error.cause) {
                    expect(error.cause.isIncorrectAmount).toBeTruthy();
                    return;
                }
            }
        }

        throw new Error('verification did not fail as expected');
    }, 1200000);
});
