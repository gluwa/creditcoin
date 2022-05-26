// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { WebSocket } from 'ws';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('Creditcoin RPC', (): void => {
    let api: ApiPromise;

    beforeEach(async () => {
        const provider = new WsProvider((global as any).CREDITCOIN_API_URL);

        api = await ApiPromise.create({
            provider,
            rpc: {
                creditcoin: {
                    getEvents: {
                        params: [
                            {
                                isOptional: true,
                                name: 'at',
                                type: 'Hash',
                            },
                        ],
                        description: 'Get events in a json format',
                        type: 'Vec<Json>',
                    },
                    eventsSubscribe: {
                        params: [],
                        description: 'Subscribe to events',
                        type: 'Subscription<Json>',
                        pubsub: ['events', 'eventsSubscribe', 'eventsUnsubscribe'],
                    },
                    hashrate: {
                        params: [],
                        description: 'Get hashrate',
                        type: 'Json',
                    },
                },
            },
        });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('getEvents() should return some events', (): void => {
        return (api.rpc as any).creditcoin.getEvents().then((result: any) => {
            // in case of failures should be easy to spot what went wrong
            console.log(`**** RESULT=${result.toString() as string}`);

            expect(result.isEmpty).toBeFalsy();
            expect(result.toJSON()).toEqual(expect.anything());
        });
    });

    it('eventsSubscribe() should receive events', (done): void => {
        expect.assertions(1);

        let subscriptionId: string;
        const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

        ws.on('open', () => {
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
                    const parsedData = JSON.parse(utf8Str);
                    expect(parsedData).toBeTruthy();
                    ws.close();
                }
            })
            .on('close', () => done());
    });

    it('eventsUnsubscribe() should return true', (done): void => {
        expect.assertions(1);

        let subscriptionId: string;
        const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

        ws.on('open', () => {
            const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsSubscribe' };
            ws.send(JSON.stringify(rpc));
        })
            .on('message', (data) => {
                const utf8Str = data.toString('utf-8');
                // console.log('decoded-message', utf8Str);

                if (!subscriptionId) {
                    subscriptionId = JSON.parse(utf8Str).result;
                    // unsubscribe
                    const rpc = {
                        id: 1,
                        jsonrpc: '2.0',
                        method: 'creditcoin_eventsUnsubscribe',
                        params: [subscriptionId],
                    };

                    ws.send(JSON.stringify(rpc));
                } else {
                    const parsedData = JSON.parse(utf8Str);
                    expect(parsedData.result).toBe(true);
                    ws.close();
                }
            })
            .on('close', () => done());
    });

    it('eventsUnsubscribe() handles invalid subscription id', (done): void => {
        expect.assertions(1);

        let firstTime = true;
        const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

        ws.on('open', () => {
            const rpc = { id: 1, jsonrpc: '2.0', method: 'creditcoin_eventsSubscribe' };
            ws.send(JSON.stringify(rpc));
        })
            .on('message', (data) => {
                const utf8Str = data.toString('utf-8');
                // console.log('decoded-message', utf8Str);

                if (firstTime) {
                    firstTime = false;

                    // unsubscribe
                    const rpc = {
                        id: 1,
                        jsonrpc: '2.0',
                        method: 'creditcoin_eventsUnsubscribe',
                        params: ['invalid-id'],
                    };

                    ws.send(JSON.stringify(rpc));
                } else {
                    const parsedData = JSON.parse(utf8Str);
                    expect(parsedData.error.message).toBe('Invalid subscription id.');
                    ws.close();
                }
            })
            .on('close', () => done());
    });

    it('hashrate() should return a valid hashrate', async () => {
        type HashrateStats = {
            elapsed: Elapsed;
            hash_count: number; // eslint-disable-line
            rate: number;
        };

        type Elapsed = {
            nanos: number;
            secs: number;
        };
        const rate = await (api.rpc as any).creditcoin.hashrate();
        const rateObj: HashrateStats = JSON.parse(rate);
        expect(rateObj.rate).toBeGreaterThan(0);
        expect(rateObj.elapsed.secs).toBeGreaterThan(0);
        expect(rateObj.elapsed.nanos).toBeGreaterThan(0);
        expect(rateObj.hash_count).toBeGreaterThan(0); // eslint-disable-line
    });
});
