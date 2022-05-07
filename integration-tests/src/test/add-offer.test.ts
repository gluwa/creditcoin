import { Guid } from 'js-guid';
import { KeyringPair } from '@polkadot/keyring/types';
import { POINT_01_CTC } from '../constants';
import { AskOrderId, BidOrderId, Blockchain, LoanTerms } from 'creditcoin-js/model';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';
import { extractFee } from '../utils';
import { signAccountId } from 'creditcoin-js/utils';

describe('AddOffer', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let askOrderId: AskOrderId;
    let bidOrderId: BidOrderId;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData;

    beforeAll(async () => {
        process.env.NODE_ENV = 'test';
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob', { name: 'Bob' });
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';
        const {
            api,
            extrinsics: { registerAddress, addAskOrder, addBidOrder },
        } = ccApi;

        const lenderWallet = createWallet();
        const borrowerWallet = createWallet();

        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            registerAddress(lenderWallet.address, blockchain, signAccountId(api, lenderWallet, lender.address), lender),
            registerAddress(
                borrowerWallet.address,
                blockchain,
                signAccountId(api, borrowerWallet, borrower.address),
                borrower,
            ),
        ]);

        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();

        const [askOrderAdded, bidOrderAdded] = await Promise.all([
            addAskOrder(lenderRegAddr.itemId, loanTerms, expirationBlock, askGuid, lender),
            addBidOrder(borrowerRegAddr.itemId, loanTerms, expirationBlock, bidGuid, borrower),
        ]);

        askOrderId = askOrderAdded.itemId;
        bidOrderId = bidOrderAdded.itemId;
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
                    extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 90000);
});
