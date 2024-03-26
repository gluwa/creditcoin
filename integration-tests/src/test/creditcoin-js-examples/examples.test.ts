import { CreditcoinApi, creditcoinApi } from 'creditcoin-js';
import { transferExample } from 'creditcoin-js/lib/examples/transfer';

describe('creditcoin js examples', () => {
    let ccApi: CreditcoinApi;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    it('address should have 1 CTC after transfer example', async () => {
        // Bob's address
        const address = '5CuAHBMBQfZhPQcTJwcZkz9R3GHPw29xYX8fWvDhbQ2tLkMQ';
        await transferExample();
        const balance = await ccApi.api.derive.balances.account(address);
        expect(balance.freeBalance).toEqual('1000000000000000000');
    });

    // Adress should have X CTC and X CTC bonded after sudo example
    // Query example should return X CTC from Y address
    // Query derive should return X CTC bonded from Y address
    // Accounts should have X CTC after batch transfer example
});
