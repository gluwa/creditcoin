// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { Balance } from '@polkadot/types/interfaces';
import { BN } from '@polkadot/util';

import { Blockchain, DealOrderId, LoanTerms } from 'credal-js/lib/model';

import { POINT_01_CTC } from '../src/constants';
import { randomEthWallet } from '../src/utils';
import * as testUtils from './test-utils';

describe('Exempt', (): void => {
    let api: ApiPromise;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrderId: DealOrderId;

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
        const lenderAddress = randomEthWallet().address;

        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
        const borrowerAddress = randomEthWallet().address;
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            testUtils.registerAddress(api, lenderAddress, blockchain, lender),
            testUtils.registerAddress(api, borrowerAddress, blockchain, borrower),
        ]);

        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();

        const [askOrderAdded, bidOrderAdded] = await Promise.all([
            testUtils.addAskOrder(api, lenderRegAddr.addressId, loanTerms, expirationBlock, askGuid, lender),
            testUtils.addBidOrder(api, borrowerRegAddr.addressId, loanTerms, expirationBlock, bidGuid, borrower),
        ]);

        const offer = await testUtils.addOffer(
            api,
            askOrderAdded.askOrderId,
            bidOrderAdded.bidOrderId,
            expirationBlock,
            lender,
        );

        const dealOrder = await testUtils.addDealOrder(api, offer.offerId, expirationBlock, borrower);
        dealOrderId = dealOrder.dealOrderId;
    }, 240000);

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .exempt(dealOrderId)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    testUtils.expectNoDispatchError(api, dispatchError);

                    if (status.isInBlock) {
                        const balancesWithdraw = events.find(({ event: { method, section } }) => {
                            return section === 'balances' && method === 'Withdraw';
                        });

                        expect(balancesWithdraw).toBeTruthy();

                        if (balancesWithdraw) {
                            const fee = (balancesWithdraw.event.data[1] as Balance).toBigInt();

                            const unsub = await unsubscribe;

                            if (typeof unsub === 'function') {
                                unsub();
                                resolve(fee);
                            } else {
                                reject(new Error('Subscription failed'));
                            }
                        } else {
                            reject(new Error("Fee wasn't found"));
                        }
                    }
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 60000);
});
