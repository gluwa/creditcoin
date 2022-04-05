// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import type { Balance } from '@polkadot/types/interfaces';

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinLoanTerms,
} from '@polkadot/types/lookup';

import { POINT_01_CTC } from '../src/constants';

import { randomEthAddress } from '../src/utils';
import * as testUtils from './test-utils';

describe('AddOffer', (): void => {
    let api: ApiPromise;
    let borrower, lender: KeyringPair;
    let loanTerms: PalletCreditcoinLoanTerms;
    let askGuid, bidGuid: string;
    let askOrderId: PalletCreditcoinAskOrderId;
    let bidOrderId: PalletCreditcoinBidOrderId;

    const blockchain = 'Ethereum';
    const expirationBlock = 10_000;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: 'sr25519' });

        lender = keyring.addFromUri('//Alice', { name: 'Alice' });
        const lenderAddress = randomEthAddress();
        const lenderRegAddr = await testUtils.registerAddress(api, lenderAddress, blockchain, lender);
        askGuid = Guid.newGuid().toString();

        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
        const borrowerAddress = randomEthAddress();
        const borrowerRegAddr = await testUtils.registerAddress(api, borrowerAddress, blockchain, borrower);
        bidGuid = Guid.newGuid().toString();

        loanTerms = api.createType<PalletCreditcoinLoanTerms>('PalletCreditcoinLoanTerms', {
            amount: 1_000,
            interestRate: 100,
            maturity: 10,
        });

        askOrderId = await testUtils.addAskOrder(
            api,
            lenderRegAddr.addressId,
            loanTerms,
            expirationBlock,
            askGuid,
            lender,
        );
        bidOrderId = await testUtils.addBidOrder(
            api,
            borrowerRegAddr.addressId,
            loanTerms,
            expirationBlock,
            bidGuid,
            borrower,
        );
    }, 210000);

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addOffer(askOrderId, bidOrderId, expirationBlock)
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
    }, 90000);
});
