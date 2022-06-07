import { fullLoanCycleExample } from 'creditcoin-js/examples/loan-cycle';

describe('Full Loan Cycle', (): void => {
    it('works as expected', async (): Promise<void> => {
        await expect(fullLoanCycleExample((global as any).CREDITCOIN_API_URL).resolves.toBeUndefined();
    }, 9000000);
});
