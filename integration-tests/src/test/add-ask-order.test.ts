import { Blockchain, Guid } from 'creditcoin-js';
import { KeyringPair } from 'creditcoin-js';
import { createCreditcoinLoanTerms } from 'creditcoin-js/lib/transforms';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData, tryRegisterAddress } from 'creditcoin-js/lib/testUtils';
import { extractFee } from '../utils';

describe('AddAskOrder', (): void => {
    let ccApi: CreditcoinApi;
    let lender: KeyringPair;
    let lenderRegAddr: AddressRegistered;
    let askGuid: Guid;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const lenderWallet = createWallet('lender');

        lenderRegAddr = await tryRegisterAddress(
            ccApi,
            lenderWallet.address,
            blockchain,
            signAccountId(ccApi.api, lenderWallet, lender.address),
            lender,
            (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
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
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    });
});
