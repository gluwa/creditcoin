// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, KeyringPair, WsProvider } from '@polkadot/api';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { POINT_01_CTC } from '../src/constants';
import { registerAddressAsync } from '../src/examples/register-address';
import { randomEthAddress } from '../src/utils';

describe('AddAskOrder', (): void => {
  let api: ApiPromise;
  let lender: KeyringPair;
  let loanTerms: PalletCreditcoinLoanTerms;
  let lenderRegAddr: RegisteredAddress;
  let askGuid: Guid;

  const blockchain = 'Ethereum';
  const expirationBlock = 5;

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
      maturity: 10
    });

    lenderRegAddr = await registerAddressAsync(api, lenderAddress, blockchain, lender);
    expect(lenderRegAddr).toBeTruthy();
    expect(lenderRegAddr.addressId).toBeTruthy();

    askGuid = Guid.newGuid().toString();
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('fee is min 0.01 CTC', (): void => {
    return new Promise(async (resolve) => {
      const unsubscribe = await api.tx.creditcoin
        .addAskOrder(lenderRegAddr.addressId, loanTerms, expirationBlock, askGuid)
        .signAndSend(lender, { nonce: -1 }, ({ dispatchError, events, status }) => {
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
      expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
    });
  });
});
