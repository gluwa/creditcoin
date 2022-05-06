// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import axios from 'axios';

const METRICS_BASE = 'http://127.0.0.1:9615';

test('Hashrate prometheus metric works', async () => {
    const { data } = await axios.get<string>(`${METRICS_BASE}/metrics`);
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
