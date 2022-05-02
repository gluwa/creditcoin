// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import * as testUtils from './test-utils';
import { createFundingTransferId } from 'credal-js/lib/extrinsics/register-transfers';
import { POINT_01_CTC } from '../src/constants';

describe('FailTransfer', (): void => {
    let api: ApiPromise;
    let authority: KeyringPair;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');
        api = await ApiPromise.create({ provider });
        const keyring = new Keyring({ type: 'sr25519' });

        const lender = keyring.addFromUri('//Alice', { name: 'Alice' });

        authority = await testUtils.setupAuthority(api, lender);
    });

    it('fee is min 0.01 CTC', async () => {
        const transferId = createFundingTransferId('Ethereum', '0xffffffffffffffffffffffffffffffffffffffff');
        const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TransferFailed');
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .failTransfer(transferId, cause)
                .signAndSend(authority, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    testUtils.extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
