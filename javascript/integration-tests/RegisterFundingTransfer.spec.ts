// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { H256 } from '@polkadot/types/interfaces';
import { BN } from '@polkadot/util';

import { Blockchain, DealOrderId, LoanTerms, TransferKind } from 'credal-js/lib/model';
import { createCreditcoinTransferKind } from 'credal-js/lib/transforms';
import { signLoanParams } from 'credal-js/lib/extrinsics/register-deal-order';

import { POINT_01_CTC } from '../src/constants';
import { randomEthWallet } from '../src/utils';
import * as testUtils from './test-utils';
import { signAccountId } from 'credal-js/lib/utils';
import { Wallet } from 'ethers';
import { createFundingTransferId, registerFundingTransferAsync } from 'credal-js/lib/extrinsics/register-transfers';
import { Vec } from '@polkadot/types-codec';
import { FrameSystemEventRecord, PalletCreditcoinOcwErrorsVerificationFailureCause } from '@polkadot/types/lookup';

describe('RegisterFundingTransfer', (): void => {
    let api: ApiPromise;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrderId: DealOrderId;
    let transferKind: TransferKind;
    let txHash: string;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;

    const blockchain: Blockchain = 'Ethereum';
    const expirationBlock = 10_000;
    const loanTerms: LoanTerms = {
        amount: new BN(1_000),
        interestRate: {
            ratePerPeriod: 100,
            decimals: 4,
            period: {
                secs: 60 * 60 * 24,
                nanos: 0,
            },
        },
        termLength: {
            secs: 60 * 60 * 24 * 30,
            nanos: 0,
        },
    };

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');
        api = await ApiPromise.create({ provider });
        const keyring = new Keyring({ type: 'sr25519' });

        lender = keyring.addFromUri('//Alice', { name: 'Alice' });
        lenderWallet = randomEthWallet();
        const lenderRegAddr_ = await testUtils.registerAddress(
            api,
            lenderWallet.address,
            blockchain,
            signAccountId(api, lenderWallet, lender.address),
            lender,
        );

        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
        borrowerWallet = randomEthWallet();
        const borrowerRegAddr_ = testUtils.registerAddress(
            api,
            borrowerWallet.address,
            blockchain,
            signAccountId(api, borrowerWallet, borrower.address),
            borrower,
        );

        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([lenderRegAddr_, borrowerRegAddr_]);

        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const signedParams = signLoanParams(api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        const result = await testUtils.registerDealOrder(
            api,
            lenderRegAddr.addressId,
            borrowerRegAddr.addressId,
            loanTerms,
            expirationBlock,
            askGuid,
            bidGuid,
            borrower.publicKey,
            signedParams,
            lender,
        );
        dealOrderId = result.dealOrder.dealOrderId;

        await testUtils.setupAuthority(api, lender);
        [transferKind, txHash] = await testUtils.prepareEthTransfer(
            lenderWallet,
            borrowerWallet,
            dealOrderId,
            loanTerms,
        );
    }, 300000);

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const ccTransferKind = createCreditcoinTransferKind(api, transferKind);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerFundingTransfer(ccTransferKind, dealOrderId, txHash)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    testUtils.extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 30000);

    it('failure event is emitted if transfer is invalid', async (): Promise<void> => {
        // wrong amount
        const badLoanTerms = { ...loanTerms, amount: new BN(1) };

        const [transferKind, txHash] = await testUtils.prepareEthTransfer(
            lenderWallet,
            borrowerWallet,
            dealOrderId,
            badLoanTerms,
        );

        const transferId = createFundingTransferId(blockchain, txHash);
        await registerFundingTransferAsync(api, transferKind, dealOrderId, txHash, lender);

        return new Promise((resolve, reject): void => {
            api.query.system
                .events((events: Vec<FrameSystemEventRecord>) => {
                    // Loop through the Vec<EventRecord>
                    events.forEach((record) => {
                        // Extract the phase, event and the event types
                        const { event } = record;
                        if (api.events.creditcoin.TransferFailedVerification.is(event)) {
                            const failedTransferId = (event.data[0] as H256).toString();
                            if (failedTransferId === transferId) {
                                const failureCause = event.data[1] as PalletCreditcoinOcwErrorsVerificationFailureCause;
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
