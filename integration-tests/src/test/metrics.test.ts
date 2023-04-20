// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import axios from 'axios';

// todo: we need to update this test for other metrics
test.skip('Hashrate prometheus metric works', async () => {
    const metricsBase: string = (global as any).CREDITCOIN_METRICS_BASE;
    const { data } = await axios.get<string>(`${metricsBase}/metrics`);
    expect(data).toContain('creditcoin_node_hash_count');

    const shortName: string = (global as any).CREDITCOIN_NETWORK_SHORT_NAME;
    const re = new RegExp(`creditcoin_node_hash_count\\{chain="${shortName}"\\} (\\d+)`);

    const match = data.match(re);
    expect(match).not.toBeNull();
    if (match) {
        // so TS sees the match is non-null
        const value = parseInt(match[1], 10);

        // the nodes dedicated to serving RPCs don't mine blocks
        // and therefore don't produce any hashes
        if (shortName === 'creditcoin_testnet') {
            expect(value).toBe(0);
        } else {
            expect(value).toBeGreaterThan(0);
        }
    }
});

test('Nonce metrics are returned', async () => {
    const metricsBase: string = (global as any).CREDITCOIN_METRICS_BASE;
    const { data } = await axios.get<string>(`${metricsBase}/metrics`);

    expect(data).toContain('authority_offchain_nonce');
    expect(data).toContain('authority_onchain_nonce');
});
