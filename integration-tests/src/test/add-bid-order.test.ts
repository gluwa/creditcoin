import { Guid } from 'js-guid';
import { KeyringPair } from '@polkadot/keyring/types';
import { createCreditcoinLoanTerms } from 'creditcoin-js/transforms';
import { AddressRegistered } from 'creditcoin-js/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { signAccountId } from 'creditcoin-js/utils';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';

import { extractFee } from '../utils';

describe('AddBidOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let borrowerRegAddr: AddressRegistered;
    let bidGuid: Guid;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData;

    beforeAll(async () => {
        process.env.NODE_ENV = 'test';
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        borrower = keyring.addFromUri('//Alice');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const borrowerWallet = createWallet();

        borrowerRegAddr = await ccApi.extrinsics.registerAddress(
            borrowerWallet.address,
            blockchain,
            signAccountId(ccApi.api, borrowerWallet, borrower.address),
            borrower,
        );
        bidGuid = Guid.newGuid();
    });

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
                    extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
