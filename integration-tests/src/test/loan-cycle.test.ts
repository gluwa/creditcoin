import { creditcoinApi } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { fullLoanCycleExample } from 'creditcoin-js/lib/examples/loan-cycle';

import { testData, tryRegisterAddress } from 'creditcoin-js/lib/testUtils';

describe('Full Loan Cycle', (): void => {
    let ccApi: CreditcoinApi;
    let registeredWallets: any;
    const { blockchain, createWallet, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        const {
            utils: { signAccountId },
        } = ccApi;

        const lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        const borrower = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');

        const lenderWallet = createWallet('lender');
        const borrowerWallet = createWallet('borrower');

        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            tryRegisterAddress(
                ccApi,
                lenderWallet.address,
                blockchain,
                signAccountId(lenderWallet, lender.address),
                lender,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
            tryRegisterAddress(
                ccApi,
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
        ]);

        registeredWallets = {
            registeredLender: {
                wallet: lenderWallet,
                keyringPair: lender,
                registeredAddress: lenderRegAddr,
            },
            registeredBorrower: {
                wallet: borrowerWallet,
                keyringPair: borrower,
                registeredAddress: borrowerRegAddr,
            },
        };
    }, 3000000);

    afterAll(async () => await ccApi.api.disconnect());

    it('works as expected', async (): Promise<void> => {
        await expect(
            fullLoanCycleExample(
                (global as any).CREDITCOIN_API_URL,
                registeredWallets,
                (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
                (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
                (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET
                    ? undefined
                    : registeredWallets.registeredLender.wallet,
            ),
        ).resolves.toBeUndefined();
    }, 120000000);
});
