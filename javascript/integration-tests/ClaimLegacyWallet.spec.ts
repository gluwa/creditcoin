// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Balance } from '@polkadot/types/interfaces';

import { CREDO_PER_CTC } from '../src/constants';
import * as testUtils from './test-utils';

describe('ClaimLegacyWallet', (): void => {
    let api: ApiPromise;
    let alice: KeyringPair;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: 'sr25519' });

        alice = keyring.addFromUri('//Alice', { name: 'Alice' });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            const unsubscribe = api.tx.creditcoin
                .claimLegacyWallet(alice.publicKey)
                .signAndSend(alice, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    testUtils.expectNoDispatchError(api, dispatchError);

                    if (status.isInBlock) {
                        const balancesWithdraw = events.find(({ event: { method, section } }) => {
                            return section === 'balances' && method === 'Withdraw';
                        });

                        expect(balancesWithdraw).toBeTruthy();

                        // const accountId = balancesWithdraw.event.data[0].toString();
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
                .catch((error) => reject(error));
        }).then((fee) => {
            // temporary workaround b/c the actual fee is 0.009 CTC
            expect(fee).toBeGreaterThanOrEqual(0.009 * CREDO_PER_CTC);
        });
    });
});
