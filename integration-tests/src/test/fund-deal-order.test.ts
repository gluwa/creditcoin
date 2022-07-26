import { KeyringPair, TransferKind } from 'creditcoin-js';

import { Guid } from 'creditcoin-js';
import { POINT_01_CTC } from '../constants';

import { signLoanParams, DealOrderRegistered } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { TransferEvent } from 'creditcoin-js/lib/extrinsics/register-transfers';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData, lendOnEth, tryRegisterAddress, setupEth, loanTermsWithCurrency } from './common';
import { extractFee } from '../utils';
import { Wallet } from 'creditcoin-js';
import { testCurrency } from 'creditcoin-js/lib/examples/ethereum';

const ethless: TransferKind = {
    platform: 'Evm',
    kind: 'Ethless',
};

describe('FundDealOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrder: DealOrderRegistered;
    let fundingEvent: TransferEvent;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;

    const { blockchain, expirationBlock, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const {
            api,
            extrinsics: { registerDealOrder, registerFundingTransfer },
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
        const eth = await setupEth(lenderWallet);
        const currency = testCurrency(eth.testTokenAddress);
        const loanTerms = await loanTermsWithCurrency(ccApi, currency);
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

        const fundingTxHash = await lendOnEth(lenderWallet, borrowerWallet, dealOrder.dealOrder.itemId, loanTerms, eth);
        fundingEvent = await registerFundingTransfer(ethless, dealOrder.dealOrder.itemId, fundingTxHash, lender);
        const fundingTransferVerified = await fundingEvent.waitForVerification().catch();
        expect(fundingTransferVerified).toBeTruthy();
    }, 900000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .fundDealOrder(dealOrder.dealOrder.itemId, fundingEvent.transferId)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 900000);
});
