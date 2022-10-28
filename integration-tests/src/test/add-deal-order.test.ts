import { creditcoinApi, KeyringPair, POINT_01_CTC } from 'creditcoin-js';
import { Currency, LoanTerms, OfferId } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { ethConnection, testCurrency } from 'creditcoin-js/lib/examples/ethereum';
import { addAskAndBidOrder, loanTermsWithCurrency, testData } from 'creditcoin-js/lib/testUtils';

import { extractFee } from '../utils';

describe('AddDealOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let offerId: OfferId;
    let loanTerms: LoanTerms;
    let currency: Currency;

    const { expirationBlock, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });

        const eth = await ethConnection(
            (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
            (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
            undefined,
        );
        currency = testCurrency(eth.testTokenAddress);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        loanTerms = await loanTermsWithCurrency(ccApi, currency);
        const [askOrderId, bidOrderId] = await addAskAndBidOrder(ccApi, lender, borrower, loanTerms);
        const offer = await ccApi.extrinsics.addOffer(askOrderId, bidOrderId, expirationBlock, lender);
        offerId = offer.itemId;
    }, 210000);

    afterEach(async () => {
        await ccApi.api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addDealOrder(offerId, expirationBlock)
                .signAndSend(borrower, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 240000);
});
