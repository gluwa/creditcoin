import { Wallet } from 'ethers';
import { Guid } from 'js-guid';
import { BN } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';
import { Blockchain, LoanTerms } from 'creditcoin-js/model';
import { CreditcoinApi } from 'creditcoin-js/types';
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

export const addAskAndBidOrder = async (ccApi: CreditcoinApi, lender: KeyringPair, borrower: KeyringPair) => {
    const {
        extrinsics: { addAskOrder, addBidOrder, registerAddress },
        utils: { signAccountId },
    } = ccApi;

    const { blockchain, expirationBlock, loanTerms } = testData;
    const lenderWallet = Wallet.createRandom();
    const borrowerWallet = Wallet.createRandom();

    const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
        registerAddress(lenderWallet.address, blockchain, signAccountId(lenderWallet, lender.address), lender),
        registerAddress(borrowerWallet.address, blockchain, signAccountId(borrowerWallet, borrower.address), borrower),
    ]);
    const askGuid = Guid.newGuid();
    const bidGuid = Guid.newGuid();

    const [askOrderAdded, bidOrderAdded] = await Promise.all([
        addAskOrder(lenderRegAddr.itemId, loanTerms, expirationBlock, askGuid, lender),
        addBidOrder(borrowerRegAddr.itemId, loanTerms, expirationBlock, bidGuid, borrower),
    ]);

    return [askOrderAdded.itemId, bidOrderAdded.itemId];
};
