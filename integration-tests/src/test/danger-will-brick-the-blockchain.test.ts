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
        let expectedDuration = 2880;

        if ((global as any).CREDITCOIN_USES_FAST_RUNTIME === true) {
            expectedDuration = 15;
        }

        const epochDuration = (ccApi.api.consts.babe.epochDuration as U64).toNumber();
        expect(epochDuration).toEqual(expectedDuration);
    });
});
