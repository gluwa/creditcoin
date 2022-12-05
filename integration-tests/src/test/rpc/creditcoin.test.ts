// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { WebSocket } from 'ws';
import { ApiPromise, WsProvider } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { testData } from 'creditcoin-js/lib/testUtils';

describe('Creditcoin RPC', (): void => {
    let api: ApiPromise;
    const { keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
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
                task: {
                    getOffchainNonceKey: {
                        params: [
                            {
                                name: 'account_id',
                                type: 'String',
                            },
                        ],
                        description: 'Offchain nonce-key',
                        type: 'Json',
                    },
                },
            },
        });
    });

    afterAll(async () => {
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

    it('getOffchainNonceKey() should return error when input is not a valid hex string', (done): void => {
        const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

        ws.on('open', () => {
            const rpc = { id: 1, jsonrpc: '2.0', method: 'task_getOffchainNonceKey', params: ['0xThisIsNotValid'] };
            ws.send(JSON.stringify(rpc));
        })
            .on('message', (data) => {
                const utf8Str = data.toString('utf-8');

                const error = JSON.parse(utf8Str).error;
                expect(error.message).toContain('Not a valid hex-string or SS58 address');
                ws.close();
            })
            .on('close', () => done());
    });

    it('getOffchainNonceKey() should work when passed a valid AccountId', async () => {
        const lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');

        const rawResponse = await (api.rpc as any).task.getOffchainNonceKey(lender.address);
        const parsedResponse = JSON.parse(rawResponse);

        expect(parsedResponse).toBeTruthy();
        expect(parsedResponse).not.toHaveProperty('error');
    });
});
