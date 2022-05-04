import { main } from 'creditcoin-js/examples/loan-cycle';

describe('Full Loan Cycle', (): void => {
    beforeAll(() => {
        process.env.NODE_ENV = 'test';
    });

    it('works as expected', async (): Promise<void> => {
        await expect(main()).resolves.toBeUndefined();
    }, 900000);
});
