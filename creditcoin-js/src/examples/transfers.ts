import { ApiPromise } from '@polkadot/api';

import { creditcoinApi } from '../creditcoin-api';
import { CreditcoinApi } from '../';

async function main(ccApi: CreditcoinApi) {
    const { api, extrinsics } = ccApi;

    const latest = await api.rpc.chain.getHeader();
    for (let i = 179000; i < latest.number.toNumber(); i++) {
        const hash = await api.rpc.chain.getBlockHash(i);
        const apiAt = await api.at(hash);
        console.log(`${i}: ${(await apiAt.query.creditcoin.unverifiedTransfers.entries()).length}`);
    }
}

async function outerMain() {
    const api = await creditcoinApi('ws://127.0.0.1:9944');

    try {
        await main(api);
    } finally {
        await api.api.disconnect();
    }
}

outerMain().catch(console.error);
