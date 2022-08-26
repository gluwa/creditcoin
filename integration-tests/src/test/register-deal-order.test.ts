// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid, LoanTerms } from 'creditcoin-js';
import { POINT_01_CTC } from '../constants';
import { KeyringPair } from 'creditcoin-js';
import { createCreditcoinLoanTerms } from 'creditcoin-js/lib/transforms';
import { signLoanParams } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { loanTermsWithCurrency, testData, tryRegisterAddress } from './common';
import { extractFee } from '../utils';

describe('RegisterDealOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;

    let borrowerAddressId: string;
    let lenderAddressId: string;
    let loanTerms: LoanTerms;
    const { blockchain, expirationBlock, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob');
        loanTerms = await loanTermsWithCurrency(ccApi);
    }, 60000);

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const {
            utils: { signAccountId },
        } = ccApi;
        const lenderWallet = createWallet('lender');
        const borrowerWallet = createWallet('borrower');
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            tryRegisterAddress(
                ccApi,
                lenderWallet.address,
                blockchain,
                signAccountId(lenderWallet, lender.address),
                lender,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
            tryRegisterAddress(
                ccApi,
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
        ]);

        borrowerAddressId = borrowerRegAddr.itemId;
        lenderAddressId = lenderRegAddr.itemId;
    }, 60000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const signedParams = signLoanParams(api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerDealOrder(
                    lenderAddressId,
                    borrowerAddressId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    askGuid.toString(),
                    bidGuid.toString(),
                    { Sr25519: borrower.publicKey }, // eslint-disable-line  @typescript-eslint/naming-convention
                    { Sr25519: signedParams }, // eslint-disable-line  @typescript-eslint/naming-convention
                )
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 60000);
});
