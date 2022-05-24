// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import type { Balance } from '@polkadot/types/interfaces';

import { Blockchain, DealOrderId, LoanTerms, TransferKind } from 'credal-js/lib/model';
import { createCreditcoinTransferKind } from 'credal-js/lib/transforms';
import { signLoanParams } from 'credal-js/lib/extrinsics/register-deal-order';

import { POINT_01_CTC } from '../src/constants';
import { randomEthAddress } from '../src/utils';
import * as testUtils from './test-utils';

describe('RegisterFundingTransfer', (): void => {
    let api: ApiPromise;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrderId: DealOrderId;
    let transferKind: TransferKind;
    let txHash: string;

    const blockchain: Blockchain = 'Ethereum';
    const expirationBlock = 10_000;
    const loanTerms: LoanTerms = {
        amount: BigInt(1_000),
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
        const lenderWallet = randomEthAddress();
        const lenderRegAddr = await testUtils.registerAddress(api, lenderWallet.address, blockchain, lender);

        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
        const borrowerWallet = randomEthAddress();
        const borrowerRegAddr = await testUtils.registerAddress(api, borrowerWallet.address, blockchain, borrower);

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
