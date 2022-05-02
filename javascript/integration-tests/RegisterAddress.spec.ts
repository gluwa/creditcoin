// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

import { POINT_01_CTC } from '../src/constants';
import { randomEthWallet, ethOwnershipProof } from '../src/utils';
import * as testUtils from './test-utils';

describe('RegisterAddress', (): void => {
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
            const wallet = randomEthWallet();
            const unsubscribe = api.tx.creditcoin
                .registerAddress('Ethereum', wallet.address, ethOwnershipProof(api, wallet, alice.address))
                .signAndSend(alice, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    testUtils.extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
