import { Wallet } from 'ethers';
import { BN } from '@polkadot/util';
import { Blockchain, LoanTerms } from 'creditcoin-js/model';
import { Keyring } from '@polkadot/api';

type TestData = {
    loanTerms: LoanTerms;
    blockchain: Blockchain;
    expirationBlock: number;
    keyring: Keyring;
    createWallet: () => Wallet;
};
export const testData: TestData = {
    loanTerms: {
        amount: new BN(1_000),
        interestRate: {
            ratePerPeriod: 100,
            decimals: 4,
            period: {
                secs: 60 * 60 * 24,
                nanos: 0,
            },
            interestType: 'Simple',
        },
        termLength: {
            secs: 60 * 60 * 24 * 30,
            nanos: 0,
        },
    } as LoanTerms,
    blockchain: 'Ethereum' as Blockchain,
    expirationBlock: 10_000,
    createWallet: Wallet.createRandom,
    keyring: new Keyring({ type: 'sr25519' }),
};
