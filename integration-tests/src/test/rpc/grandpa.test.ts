// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, WsProvider } from 'creditcoin-js';

describe('Grandpa RPC sanity test', (): void => {
    let api: ApiPromise;

    beforeEach(async () => {
        const provider = new WsProvider((global as any).CREDITCOIN_API_URL);

        api = await ApiPromise.create({ provider });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('rpc.grandpa.roundState() works', async (): Promise<void> => {
        const result = await api.rpc.grandpa.roundState();
        expect(result.best.round.toNumber()).toBeGreaterThanOrEqual(0);
    });
});
