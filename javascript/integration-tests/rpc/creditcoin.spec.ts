// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { WebSocket } from 'ws';

import { ApiPromise, WsProvider } from '@polkadot/api';

describe('Creditcoin RPC', (): void => {
  let api: ApiPromise;

  beforeEach(async () => {
    process.env.NODE_ENV = 'test';

    const provider = new WsProvider('ws://127.0.0.1:9944');

    api = await ApiPromise.create({
      provider: provider,
      rpc: {
        creditcoin: {
          getEvents: {
            params: [
              {
                isOptional: true,
                name: 'at',
                type: 'Hash'
              }
            ],
            type: 'Vec<friendly__Event>'
          }
        }
      },
      types: {
        // WARNING: copied from node/rpc/src/friendly.rs
        // Will crash if we receive an event whose data type isn't defined here
        // or references other unknown data types
        friendly__Event: {
          _enum: {
            addressRegistered: {
              address: 'Address<AccountId>',
              address_id: 'AddressId<Hash>'
            },
            askOrderAdded: {
              ask: 'AskOrder<AccountId, BlockNumber, Hash, Moment>',
              ask_id: 'AskOrderId<BlockNumber, Hash>'
            },
            bidOrderAdded: {
              bid: 'BidOrder<AccountId, BlockNumber, Hash, Moment>',
              bid_id: 'BidOrderId<BlockNumber, Hash>'
            },
            ctcDeposit: {
              amount: String,
              into: 'AccountId'
            },
            ctcTransfer: {
              amount: String,
              from: 'AccountId',
              to: 'AccountId'
            },
            ctcWithdraw: {
              amount: String,
              from: 'AccountId'
            },
            dealOrderAdded: {
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
              deal_id: 'DealOrderId<BlockNumber, Hash>'
            },
            dealOrderClosed: {
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
              deal_id: 'DealOrderId<BlockNumber, Hash>'
            },
            dealOrderFunded: {
              deal: 'DealOrder<AccountId, BlockNumber, Hash, Moment>',
              deal_id: 'DealOrderId<BlockNumber, Hash>'
            },
            legacyWalletClaimed: {
              claimed_balance: String,
              legacy_sighash: 'LegacySighash',
              new_account_id: 'AccountId'
            },
            loanExempted: {
              deal_id: 'DealOrderId<BlockNumber, Hash>',
              exempt_transfer_id: 'TransferId<Hash>'
            },
            offerAdded: {
              offer: 'Offer<AccountId, BlockNumber, Hash>',
              offer_id: 'OfferId<BlockNumber, Hash>'
            },
            rewardIssued: {
              amount: String,
              to: 'AccountId'
            },
            transferProcessed: {
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
              transfer_id: 'TransferId<Hash>'
            },
            transferRegistered: {
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
              transfer_id: 'TransferId<Hash>'
            },
            transferVerified: {
              transfer: 'Transfer<AccountId, BlockNumber, Hash>',
              transfer_id: 'TransferId<Hash>'
            }
          }
        }
      }

    });
  });

  afterEach(async () => {
    await api.disconnect();
  });

  it('getEvents() should return some events', (): void => {
    return api.rpc.creditcoin.getEvents().then((result) => {
      // in case of failures should be easy to spot what went wrong
      console.log(`**** RESULT=${result}`);

      expect(result.isEmpty).toBeFalsy();
      expect(result.toJSON()).toEqual(expect.anything());
    });
  });

  it('eventsSubscribe() should receive events', (done): void => {
    expect.assertions(1);

    let subscriptionId;
    const ws = new WebSocket('ws://127.0.0.1:9944');

    ws
      .on('open', () => {
        const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsSubscribe' };

        ws.send(JSON.stringify(rpc));
      })
      .on('message', (data) => {
        const utf8Str = data.toString('utf-8');
        // console.log('decoded-message', utf8Str);

        if (!subscriptionId) {
          subscriptionId = JSON.parse(utf8Str).result;
        } else {
          // assert at least one message is received
          const data = JSON.parse(utf8Str);

          expect(data).toBeTruthy();
          ws.close();
        }
      })
      .on('close', () => done());
  });

  it('eventsUnsubscribe() should return true', (done): void => {
    expect.assertions(1);

    let subscriptionId;
    const ws = new WebSocket('ws://127.0.0.1:9944');

    ws
      .on('open', () => {
        const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsSubscribe' };

        ws.send(JSON.stringify(rpc));
      })
      .on('message', (data) => {
        const utf8Str = data.toString('utf-8');
        // console.log('decoded-message', utf8Str);

        if (!subscriptionId) {
          subscriptionId = JSON.parse(utf8Str).result;
          // unsubscribe
          const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsUnsubscribe', params: [subscriptionId] };

          ws.send(JSON.stringify(rpc));
        } else {
          const data = JSON.parse(utf8Str);

          expect(data.result).toBe(true);
          ws.close();
        }
      })
      .on('close', () => done());
  });

  it('eventsUnsubscribe() handles invalid subscription id', (done): void => {
    expect.assertions(1);

    let firstTime = true;
    const ws = new WebSocket('ws://127.0.0.1:9944');

    ws
      .on('open', () => {
        const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsSubscribe' };

        ws.send(JSON.stringify(rpc));
      })
      .on('message', (data) => {
        const utf8Str = data.toString('utf-8');
        // console.log('decoded-message', utf8Str);

        if (firstTime) {
          firstTime = false;

          // unsubscribe
          const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsUnsubscribe', params: ['invalid-id'] };

          ws.send(JSON.stringify(rpc));
        } else {
          const data = JSON.parse(utf8Str);

          expect(data.error.message).toBe('Invalid subscription id.');
          ws.close();
        }
      })
      .on('close', () => done());
  });
});
