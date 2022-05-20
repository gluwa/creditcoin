import { KeyringPair } from '@polkadot/keyring/types';
import { POINT_01_CTC } from '../constants';
import { AskOrderId, BidOrderId } from 'creditcoin-js/model';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { addAskAndBidOrder, testData } from './common';
import { extractFee } from '../utils';

describe('AddOffer', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let askOrderId: AskOrderId;
    let bidOrderId: BidOrderId;

    const { expirationBlock, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        [askOrderId, bidOrderId] = await addAskAndBidOrder(ccApi, lender, borrower);
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
