import { KeyringPair, POINT_01_CTC, creditcoinApi } from 'creditcoin-js';
import { AskOrderId, BidOrderId, Blockchain, LoanTerms } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { addAskAndBidOrder, loanTermsWithCurrency, testData } from 'creditcoin-js/lib/testUtils';
import { ethConnection, testCurrency } from 'creditcoin-js/lib/examples/ethereum';

import { extractFee } from '../utils';

describe('AddOffer', () => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let askOrderId: AskOrderId;
    let bidOrderId: BidOrderId;
    let loanTerms: LoanTerms;

    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { expirationBlock, keyring } = testingData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        borrower = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');

        const eth = await ethConnection(
            (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
            (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
            undefined,
        );
        const currency = testCurrency(eth.testTokenAddress);
        loanTerms = await loanTermsWithCurrency(ccApi, currency);
    }, 60000);

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        [askOrderId, bidOrderId] = await addAskAndBidOrder(ccApi, lender, borrower, loanTerms, testingData);
    }, 210000);

    afterEach(async () => {
        await ccApi.api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addOffer(askOrderId, bidOrderId, expirationBlock)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 90000);
});
