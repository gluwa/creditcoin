// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Balance } from '@polkadot/types/interfaces';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { POINT_01_CTC } from '../src/constants';
import { registerAddressAsync, RegisteredAddress } from '../src/examples/register-address';
import { randomEthAddress } from '../src/utils';

describe('AddBidOrder', (): void => {
  let api: ApiPromise;
  let borrower: KeyringPair;
  let loanTerms: PalletCreditcoinLoanTerms;
  let borrowerRegAddr: RegisteredAddress;
  let bidGuid: string;

  const blockchain = 'Ethereum';
  const expirationBlock = 5;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: 'sr25519' });

    borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
    const borrowerAddress = randomEthAddress();

    loanTerms = api.createType<PalletCreditcoinLoanTerms>('PalletCreditcoinLoanTerms', {
      amount: 1_000,
      interestRate: 100,
      maturity: 10
    });

    const result = await registerAddressAsync(api, borrowerAddress, blockchain, borrower);

    expect(result).toBeTruthy();

    if (result) {
      borrowerRegAddr = result;
      expect(borrowerRegAddr.addressId).toBeTruthy();
      bidGuid = Guid.newGuid().toString();
    } else {
      throw new Error("Borrower address wasn't registered successfully");
    }
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('fee is min 0.01 CTC', async (): Promise<void> => {
    return new Promise((resolve, reject) => {
      const unsubscribe = api.tx.creditcoin
        .addBidOrder(borrowerRegAddr.addressId, loanTerms, expirationBlock, bidGuid)
        .signAndSend(borrower, { nonce: -1 }, async ({ dispatchError, events, status }) => {
          expect(dispatchError).toBeFalsy();

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
      expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
    });
  });
});
