// Copyright 2022-2023 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { WebSocket } from 'ws';
import { ApiPromise, WsProvider } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { testData } from 'creditcoin-js/lib/testUtils';

import { testIf } from '../../utils';

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

    testIf(
        (global as any).CREDITCOIN_API_URL === 'ws://127.0.0.1:9944',
        'getOffchainNonceKey() should return error when input is not a valid hex string',
        (done: any): void => {
            const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

            ws.on('open', () => {
                const rpc = { id: 1, jsonrpc: '2.0', method: 'task_getOffchainNonceKey', params: ['0xThisIsNotValid'] };
                ws.send(JSON.stringify(rpc));
            })
                .on('message', (data) => {
                    // eslint-disable-next-line @typescript-eslint/no-base-to-string
                    const utf8Str = data.toString('utf-8');

                    const error = JSON.parse(utf8Str).error;
                    expect(error.message).toContain('Not a valid hex-string or SS58 address');
                    ws.close();
                })
                .on('close', () => done());
        },
    );

    testIf(
        (global as any).CREDITCOIN_API_URL === 'ws://127.0.0.1:9944',
        'after 5:00 mins of inactivity connection will still be open and able to transmit messages',
        (done: any): void => {
            const ws = new WebSocket((global as any).CREDITCOIN_API_URL);

            ws.on('open', () => {
                setTimeout(() => {
                    const rpc = {
                        id: 1,
                        jsonrpc: '2.0',
                        method: 'task_getOffchainNonceKey',
                        params: ['0xThisIsNotValid'],
                    };
                    // connection is still open
                    expect(ws.readyState).toEqual(WebSocket.OPEN);

                    ws.send(JSON.stringify(rpc));
                }, 300_000); // wait for 5 min before sending a message
            })
                .on('message', (data) => {
                    // eslint-disable-next-line @typescript-eslint/no-base-to-string
                    const utf8Str = data.toString('utf-8');

                    const error = JSON.parse(utf8Str).error;
                    expect(error.message).toContain('Not a valid hex-string or SS58 address');
                    ws.close();
                })
                .on('close', () => done());
        },
        350_000,
    );

    testIf(
        (global as any).CREDITCOIN_API_URL === 'ws://127.0.0.1:9944',
        'getOffchainNonceKey() should work when passed a valid AccountId',
        async () => {
            const lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');

            const rawResponse = await (api.rpc as any).task.getOffchainNonceKey(lender.address);
            const parsedResponse = JSON.parse(rawResponse);

            expect(parsedResponse).toBeTruthy();
            expect(parsedResponse).not.toHaveProperty('error');
        },
    );
});
