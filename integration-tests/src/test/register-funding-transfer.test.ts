import { KeyringPair, TransferKind } from 'creditcoin-js';

import { Guid } from 'creditcoin-js';
import { POINT_01_CTC } from '../constants';
import { BN } from '@polkadot/util';

import { signLoanParams } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { createCreditcoinTransferKind } from 'creditcoin-js/lib/transforms';
import { testData, lendOnEth, tryRegisterAddress, setupEth, loanTermsWithCurrency } from './common';
import { extractFee } from '../utils';
import { Wallet } from 'creditcoin-js';
import { testCurrency } from 'creditcoin-js/lib/examples/ethereum';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';

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

    const { blockchain, expirationBlock, createWallet, keyring } = testData;

    const setup = async () => {
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const eth = await setupEth(lenderWallet);
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
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob');
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

    it('failure event is emitted if transfer is invalid', async (): Promise<void> => {
        const { dealOrder, loanTerms, eth } = await setup();

        // wrong amount
        const badLoanTerms = { ...loanTerms, amount: new BN(1) };
        const dealOrderId = dealOrder.dealOrder.itemId;
        const { api } = ccApi;

        const failureTxHash = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            badLoanTerms,
            eth,
        );

        const { transferId } = await ccApi.extrinsics.registerFundingTransfer(
            ethless,
            dealOrderId,
            failureTxHash,
            lender,
        );

        return new Promise((resolve, reject): void => {
            api.query.system
                .events((events: any) => {
                    // Loop through the Vec<EventRecord>
                    events.forEach((record: any) => {
                        // Extract the phase, event and the event types
                        const { event } = record;
                        if (api.events.creditcoin.TransferFailedVerification.is(event)) {
                            const failedTransferId = event.data[0].toString();
                            if (failedTransferId === transferId) {
                                const failureCause = event.data[1] as any;
                                expect(failureCause.isIncorrectAmount).toBeTruthy();
                                resolve();
                            }
                        }
                    });
                })
                .catch(reject);
        });
    }, 12000000);
});
