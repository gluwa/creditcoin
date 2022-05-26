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

describe('AddAskOrder', (): void => {
    let ccApi: CreditcoinApi;
    let lender: KeyringPair;
    let lenderRegAddr: AddressRegistered;
    let askGuid: Guid;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const lenderWallet = createWallet();

        lenderRegAddr = await ccApi.extrinsics.registerAddress(
            lenderWallet.address,
            blockchain,
            signAccountId(ccApi.api, lenderWallet, lender.address),
            lender,
        );
        askGuid = Guid.newGuid();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addAskOrder(
                    lenderRegAddr.itemId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    askGuid.toString(),
                )
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
