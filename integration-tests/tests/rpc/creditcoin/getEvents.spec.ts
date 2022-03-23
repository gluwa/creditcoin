// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, WsProvider } from '@polkadot/api';
import type { Vec } from '@polkadot/types-codec';
import type { PalletCreditcoinAskOrderId, Transfer } from '../../../src/interfaces/lookup';

describe('Creditcoin RPC', (): void => {
  let api: ApiPromise;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({
      provider: provider,
      types: {
        // WARNING: copied from node/rpc/src/friendly.rs
        // Will crash if we receive an event whose data type isn't defined here
        // or references other unknown data types
        friendly__Event: {
          _enum: {
            ctcTransfer: {
              from: 'AccountId',
              to: 'AccountId',
              amount: String,
            },
            ctcDeposit: {
              into: 'AccountId',
              amount: String,
            },
            ctcWithdraw: {
              from: 'AccountId',
              amount: String,
            },
            rewardIssued: {
              to: 'AccountId',
              amount: String,
            },
            addressRegistered: {
              address_id: 'AddressId<Hash>',
              address: 'Address<AccountId>',
            },
            transferRegistered: {
              transfer_id: 'TransferId<Hash>',
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
            },
            transferVerified: {
              transfer_id: 'TransferId<Hash>',
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
            },
            transferProcessed: {
              transfer_id: 'TransferId<Hash>',
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
            },
            askOrderAdded: {
              ask_id: 'AskOrderId<BlockNumber, Hash>',
              ask: 'AskOrder<AccountId, BlockNumber, Hash, Moment>',
            },
            bidOrderAdded: {
              bid_id: 'BidOrderId<BlockNumber, Hash>',
              bid: 'BidOrder<AccountId, BlockNumber, Hash, Moment>',
            },
            offerAdded: {
              offer_id: 'OfferId<BlockNumber, Hash>',
              offer: 'Offer<AccountId, BlockNumber, Hash>',
            },
            dealOrderAdded: {
              deal_id: 'DealOrderId<BlockNumber, Hash>',
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
            },
            dealOrderFunded: {
              deal_id: 'DealOrderId<BlockNumber, Hash>',
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
            },
            dealOrderClosed: {
              deal_id: 'DealOrderId<BlockNumber, Hash>',
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
            },
            loanExempted: {
                deal_id: 'DealOrderId<BlockNumber, Hash>',
                exempt_transfer_id: 'TransferId<Hash>',
            },
            legacyWalletClaimed: {
              new_account_id: 'AccountId',
              legacy_sighash: 'LegacySighash',
              claimed_balance: String,
            },
          }
        },
      },
      rpc: {
        creditcoin: {
          getEvents: {
            params: [
              {
                name: 'at',
                type: 'Hash',
                isOptional: true
              }
            ],
            type: 'Vec<friendly__Event>'
          }
        }
      }
    });
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('getEvents() should return some events', (): void => {
    return api.rpc.creditcoin.getEvents().then(result => {
      // in case of failures should be easy to spot what went wrong
      console.log(`**** RESULT=${result}`);

      expect(result.isEmpty).toBeFalsy();
      expect(result.toJSON()).toEqual(expect.anything());
    });
  });

});
