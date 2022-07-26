import { Guid, LoanTerms } from 'creditcoin-js';
import { KeyringPair } from 'creditcoin-js';
import { createCreditcoinLoanTerms } from 'creditcoin-js/lib/transforms';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { loanTermsWithCurrency, testData, tryRegisterAddress } from './common';

import { extractFee } from '../utils';

describe('AddBidOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let borrowerRegAddr: AddressRegistered;
    let bidGuid: Guid;
    let loanTerms: LoanTerms;

    const { blockchain, expirationBlock, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        borrower = keyring.addFromUri('//Bob');
        loanTerms = await loanTermsWithCurrency(ccApi);
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
