// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { WebSocket } from 'ws';
import { ApiPromise, WsProvider } from 'creditcoin-js';

describe('Creditcoin RPC', (): void => {
    let api: ApiPromise;

    beforeEach(async () => {
        const provider = new WsProvider((global as any).CREDITCOIN_API_URL);

        api = await ApiPromise.create({
            provider,
            rpc: {
                creditcoin: {
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

        const shortName: string = (global as any).CREDITCOIN_NETWORK_SHORT_NAME;

        // the nodes dedicated to serving RPCs don't mine blocks
        // and therefore don't produce any hashes
        if (shortName === 'creditcoin_testnet') {
            expect(rateObj.rate).toBe(0);
            expect(rateObj.hash_count).toBe(0); // eslint-disable-line
        } else {
            expect(rateObj.rate).toBeGreaterThan(0);
            expect(rateObj.hash_count).toBeGreaterThan(0); // eslint-disable-line
        }
        expect(rateObj.elapsed.secs).toBeGreaterThan(0);
        expect(rateObj.elapsed.nanos).toBeGreaterThan(0);
    });
});
