import { KeyringPair } from '@polkadot/keyring/types';

import { Guid } from 'js-guid';
import { POINT_01_CTC } from '../constants';
import { BN } from '@polkadot/util';
import { AskOrderId, BidOrderId, TransferKind } from 'creditcoin-js/model';

import { signLoanParams, DealOrderRegistered } from 'creditcoin-js/extrinsics/register-deal-order';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { createCreditcoinTransferKind } from 'creditcoin-js/transforms';
import { testData, lendOnEth } from './common';
import { extractFee } from '../utils';
import { Wallet } from 'ethers';
import { createFundingTransferId } from 'creditcoin-js/extrinsics/register-transfers';

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
        process.env.NODE_ENV = 'test';
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';
        const {
            api,
            extrinsics: { registerAddress, registerDealOrder },
            utils: { signAccountId },
        } = ccApi;
        lenderWallet = createWallet();
        borrowerWallet = createWallet();
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            registerAddress(lenderWallet.address, blockchain, signAccountId(lenderWallet, lender.address), lender),
            registerAddress(
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
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
    }, 60000);

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
                    extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 30000);

    it('failure event is emitted if transfer is invalid', async (): Promise<void> => {
        // wrong amount
        const badLoanTerms = { ...loanTerms, amount: new BN(1) };
        const dealOrderId = dealOrder.dealOrder.itemId;
        const { api } = ccApi;

        const [testTokenAddress, txHash] = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            badLoanTerms,
        );

        const transferId = createFundingTransferId(blockchain, txHash);
        await ccApi.extrinsics.registerFundingTransfer(
            { kind: 'Ethless', contractAddress: testTokenAddress },
            dealOrderId,
            txHash,
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
    }, 60000);
});
