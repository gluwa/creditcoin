import { fullLoanCycleExample } from 'creditcoin-js/examples/loan-cycle';

describe('Full Loan Cycle', (): void => {
    it('works as expected', async (): Promise<void> => {
        await expect(fullLoanCycleExample()).resolves.toBeUndefined();
    }, 900000);
});
