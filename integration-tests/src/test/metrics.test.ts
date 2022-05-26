// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import axios from 'axios';

test('Hashrate prometheus metric works', async () => {
    const metricsBase: string = (global as any).CREDITCOIN_METRICS_BASE;
    const { data } = await axios.get<string>(`${metricsBase}/metrics`);
    expect(data).toContain('creditcoin_node_hash_count');
    const re = /creditcoin_node_hash_count\{chain="dev"\} (\d+)/;
    const match = data.match(re);
    expect(match).not.toBeNull();
    if (match) {
        // so TS sees the match is non-null
        const value = parseInt(match[1], 10);
        expect(value).toBeGreaterThan(0);
    }
});
