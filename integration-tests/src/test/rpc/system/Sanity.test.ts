// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, WsProvider } from '@polkadot/api';

describe('System RPC sanity test', (): void => {
    let api: ApiPromise;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('rpc.system.chain() works', async (): Promise<void> => {
        const result = await api.rpc.system.chain();
        expect(result.toString()).toBe('Development');
    });

    it('rpc.system.name() works', async (): Promise<void> => {
        const result = await api.rpc.system.name();
        expect(result.toString()).toBe('Creditcoin Node');
    });

    it('rpc.system.version() works', async (): Promise<void> => {
        const result = await api.rpc.system.version();
        expect(result.toString()).toEqual(expect.anything());
    });
});
