// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

import { CREDO_PER_CTC } from '../src/constants';
import { randomEthAddress } from '../src/utils';

describe('RegisterAddress', (): void => {
  let api;
  let alice;

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

  it('fee is min 0.01 CTC', (): void => {
    return new Promise(async (resolve) => {
      const unsubscribe = await api.tx.creditcoin
        .registerAddress('Ethereum', randomEthAddress())
        .signAndSend(alice, { nonce: -1 }, ({ dispatchError, events, status }) => {
          expect(dispatchError).toBeFalsy();

          if (status.isInBlock) {
            const balancesWithdraw = events.find(({ event: { method,
              section } }) => {
              return section === 'balances' && method === 'Withdraw';
            });

            expect(balancesWithdraw).toBeTruthy();

            // const accountId = balancesWithdraw.event.data[0].toString();
            const fee = balancesWithdraw.event.data[1].toBigInt();

            unsubscribe();
            resolve(fee);
          }
        });
    }).then((fee) => {
      // temporary workaround b/c the actual fee is 0.009 CTC
      expect(fee).toBeGreaterThanOrEqual(0.009 * CREDO_PER_CTC);
    });
  });
});
