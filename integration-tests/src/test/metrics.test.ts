// Copyright 2022-2023 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import axios from 'axios';
import { testIf } from '../utils';

test('Prometheus metrics work', async () => {
    const metricsBase: string = (global as any).CREDITCOIN_METRICS_BASE;
    const { data } = await axios.get<string>(`${metricsBase}/metrics`);
    expect(data).toContain('substrate_block_height');

    const shortName: string = (global as any).CREDITCOIN_NETWORK_SHORT_NAME;
    const re = new RegExp(`substrate_block_height\\{status="best",chain="${shortName}"\\} (\\d+)`);

    const match = data.match(re);
    expect(match).not.toBeNull();
    if (match) {
        // so TS sees the match is non-null
        const value = parseInt(match[1], 10);
        expect(value).toBeGreaterThan(0);
    }
});

testIf((global as any).CREDITCOIN_API_URL === 'ws://127.0.0.1:9944', 'Nonce metrics are returned', async () => {
    const metricsBase: string = (global as any).CREDITCOIN_METRICS_BASE;
    const { data } = await axios.get<string>(`${metricsBase}/metrics`);

    expect(data).toContain('authority_offchain_nonce');
    expect(data).toContain('authority_onchain_nonce');
});
