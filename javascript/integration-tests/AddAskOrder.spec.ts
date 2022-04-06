// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import type { Balance } from '@polkadot/types/interfaces';

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { POINT_01_CTC } from '../src/constants';
import { registerAddressAsync, RegisteredAddress } from '../src/examples/register-address';
import { randomEthAddress } from '../src/utils';

describe('AddAskOrder', (): void => {
    let api: ApiPromise;
    let lender: KeyringPair;
    let loanTerms: PalletCreditcoinLoanTerms;
    let lenderRegAddr: RegisteredAddress;
    let askGuid: string;

    const blockchain = 'Ethereum';
    const expirationBlock = 10_000;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: 'sr25519' });

        lender = keyring.addFromUri('//Alice', { name: 'Alice' });
        const lenderAddress = randomEthAddress();

        loanTerms = api.createType<PalletCreditcoinLoanTerms>('PalletCreditcoinLoanTerms', {
            amount: 1_000,
            interestRate: 100,
            maturity: 10,
        });

        const addr = await registerAddressAsync(api, lenderAddress, blockchain, lender);

        expect(addr).toBeTruthy();

        if (addr) {
            lenderRegAddr = addr;
            expect(lenderRegAddr.addressId).toBeTruthy();
            askGuid = Guid.newGuid().toString();
        } else {
            throw new Error("Lender address wasn't registered successfully");
        }
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addAskOrder(lenderRegAddr.addressId, loanTerms, expirationBlock, askGuid)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    expect(dispatchError).toBeFalsy();

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
    });
});
