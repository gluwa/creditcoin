import { KeyringPair } from 'creditcoin-js';
import { Guid } from 'creditcoin-js';
import { POINT_01_CTC } from '../constants';
import { BN } from 'creditcoin-js';
import { signLoanParams, DealOrderRegistered } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi, VerificationError } from 'creditcoin-js/lib/types';
import { createCreditcoinTransferKind } from 'creditcoin-js/lib/transforms';
import { testData, lendOnEth, tryRegisterAddress } from './common';
import { extractFee } from '../utils';
import { Wallet } from 'creditcoin-js';

describe('RegisterFundingTransfer', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrder: DealOrderRegistered;
    let testTokenAddress: string;
    let txHash: string;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData;

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
            api,
            extrinsics: { registerDealOrder },
            utils: { signAccountId },
        } = ccApi;
        lenderWallet = createWallet('lender');
        borrowerWallet = createWallet('borrower');
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
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
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const signedParams = signLoanParams(api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        dealOrder = await registerDealOrder(
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

        [testTokenAddress, txHash] = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            loanTerms,
        );
    }, 900000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const ccTransferKind = createCreditcoinTransferKind(api, {
            kind: 'Ethless',
            contractAddress: testTokenAddress,
        });

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerFundingTransfer(ccTransferKind, dealOrder.dealOrder.itemId, txHash)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    }, 300000);

    it('emits a failure event if transfer is invalid', async (): Promise<void> => {
        // wrong amount
        const badLoanTerms = { ...loanTerms, amount: new BN(1) };
        const dealOrderId = dealOrder.dealOrder.itemId;

        const [failureTokenAddress, failureTxHash] = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            badLoanTerms,
        );

        const { waitForVerification } = await ccApi.extrinsics.registerFundingTransfer(
            { kind: 'Ethless', contractAddress: failureTokenAddress },
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
