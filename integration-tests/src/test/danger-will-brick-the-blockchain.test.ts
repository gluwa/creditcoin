import { U64 } from '@polkadot/types-codec';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';

describe('Events that WILL brick the blockchain', (): void => {
    let ccApi: CreditcoinApi;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    test('EPOCH_DURATION has changed', () => {
        let expectedValue = 2880;

        if ((global as any).CREDITCOIN_USES_FAST_RUNTIME === true) {
            expectedValue = 15;
        }

        const epochDuration = (ccApi.api.consts.babe.epochDuration as U64).toNumber();
        expect(epochDuration).toEqual(expectedValue);
    });

    test('Block time has changed', () => {
        let expectedValue = 15000;

        if ((global as any).CREDITCOIN_USES_FAST_RUNTIME === true) {
            expectedValue = 5000;
        }

        const blockTime = (ccApi.api.consts.babe.expectedBlockTime as U64).toNumber();
        expect(blockTime).toEqual(expectedValue);
    });

    test('Minimum period has changed', () => {
        // blockTime / 2
        let expectedValue = 7500;

        if ((global as any).CREDITCOIN_USES_FAST_RUNTIME === true) {
            expectedValue = 2500;
        }

        const blockTime = (ccApi.api.consts.timestamp.minimumPeriod as U64).toNumber();
        expect(blockTime).toEqual(expectedValue);
    });
});
