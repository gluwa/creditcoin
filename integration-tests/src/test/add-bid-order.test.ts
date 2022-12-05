import { Guid, LoanTerms, KeyringPair, POINT_01_CTC, creditcoinApi } from 'creditcoin-js';
import { ethConnection, testCurrency } from 'creditcoin-js/lib/examples/ethereum';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';
import { Blockchain } from 'creditcoin-js/lib/model';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { loanTermsWithCurrency, testData, tryRegisterAddress } from 'creditcoin-js/lib/testUtils';
import { createCreditcoinLoanTerms } from 'creditcoin-js/lib/transforms';

import { extractFee } from '../utils';

describe('AddBidOrder', () => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let borrowerRegAddr: AddressRegistered;
    let bidGuid: Guid;
    let loanTerms: LoanTerms;

    const { blockchain, expirationBlock, createWallet, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        borrower = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');

        const eth = await ethConnection(
            (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
            (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
            undefined,
        );
        const currency = testCurrency(eth.testTokenAddress);
        loanTerms = await loanTermsWithCurrency(
            ccApi,
            currency,
            (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'sudo'),
        );
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const borrowerWallet = createWallet('borrower');

        borrowerRegAddr = await tryRegisterAddress(
            ccApi,
            borrowerWallet.address,
            blockchain,
            signAccountId(ccApi.api, borrowerWallet, borrower.address),
            borrower,
            (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
        );
        bidGuid = Guid.newGuid();
    }, 60000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        return new Promise((resolve, reject) => {
            const unsubscribe = api.tx.creditcoin
                .addBidOrder(
                    borrowerRegAddr.itemId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    bidGuid.toString(),
                )
                .signAndSend(borrower, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
