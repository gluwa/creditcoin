// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN } from '@polkadot/util';

import { creditcoinApi } from 'creditcoin-js';
import { Blockchain, LoanTerms } from 'creditcoin-js/model';
import { createCreditcoinLoanTerms } from 'creditcoin-js/transforms';
import { AddressRegistered } from 'creditcoin-js/extrinsics/register-address';

import { POINT_01_CTC } from '../constants';
import { Wallet } from 'ethers';
import { extractFee } from '../utils';
import { signAccountId } from 'creditcoin-js/utils';

describe('AddAskOrder', (): void => {
    let api: ApiPromise;
    let lender: KeyringPair;
    let lenderRegAddr: AddressRegistered;
    let askGuid: Guid;

    const keyring = new Keyring({ type: 'sr25519' });
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
            interestType: 'Simple',
        },
        termLength: {
            secs: 60 * 60 * 24 * 30,
            nanos: 0,
        },
    };

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';
        const ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        api = ccApi.api;
        lender = keyring.addFromUri('//Alice');
        const lenderWallet = Wallet.createRandom();

        lenderRegAddr = await ccApi.extrinsics.registerAddress(
            lenderWallet.address,
            blockchain,
            signAccountId(api, lenderWallet, lender.address),
            lender,
        );
        askGuid = Guid.newGuid();
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addAskOrder(
                    lenderRegAddr.itemId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    askGuid.toString(),
                )
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
