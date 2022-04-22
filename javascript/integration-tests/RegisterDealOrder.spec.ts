// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { Balance } from '@polkadot/types/interfaces';
import { BN } from '@polkadot/util';

import { Blockchain, LoanTerms } from 'credal-js/lib/model';
import { createCreditcoinLoanTerms } from 'credal-js/lib/transforms';
import { AddressRegistered } from 'credal-js/lib/extrinsics/register-address';
import { signLoanParams } from 'credal-js/lib/extrinsics/register-deal-order';

import { POINT_01_CTC } from '../src/constants';
import { randomEthWallet } from '../src/utils';
import * as testUtils from './test-utils';

describe('RegisterDealOrder', (): void => {
    let api: ApiPromise;
    let borrower: KeyringPair;
    let lender: KeyringPair;

    let borrowerRegAddr: AddressRegistered;
    let lenderRegAddr: AddressRegistered;

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
        lenderRegAddr = await testUtils.registerAddress(api, lenderAddress, blockchain, lender);

        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
        const borrowerAddress = randomEthWallet().address;
        borrowerRegAddr = await testUtils.registerAddress(api, borrowerAddress, blockchain, borrower);
    }, 60000);

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const signedParams = signLoanParams(api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerDealOrder(
                    lenderRegAddr.addressId,
                    borrowerRegAddr.addressId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    askGuid.toString(),
                    bidGuid.toString(),
                    { Sr25519: borrower.publicKey }, // eslint-disable-line  @typescript-eslint/naming-convention
                    { Sr25519: signedParams }, // eslint-disable-line  @typescript-eslint/naming-convention
                )
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
    }, 30000);
});
