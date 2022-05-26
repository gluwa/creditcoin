import { KeyringPair } from '@polkadot/keyring/types';
import { POINT_01_CTC } from '../constants';
import { OfferId } from 'creditcoin-js/model';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { addAskAndBidOrder, testData } from './common';
import { extractFee } from '../utils';

describe('AddDealOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let offerId: OfferId;

    const { expirationBlock, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const [askOrderId, bidOrderId] = await addAskAndBidOrder(ccApi, lender, borrower);
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
    }, 30000);
});
