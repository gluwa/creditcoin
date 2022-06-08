// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, WsProvider } from 'creditcoin-js';

describe('System RPC sanity test', (): void => {
    let api: ApiPromise;

    beforeEach(async () => {
        const provider = new WsProvider((global as any).CREDITCOIN_API_URL);

        api = await ApiPromise.create({ provider });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('rpc.system.chain() works', async (): Promise<void> => {
        const result = await api.rpc.system.chain();
        expect(result.toString()).toBe((global as any).CREDITCOIN_NETWORK_LONG_NAME);
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
