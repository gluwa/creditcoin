import { Blockchain, KeyringPair, Wallet, POINT_01_CTC, creditcoinApi } from 'creditcoin-js';
import { createAddressId } from 'creditcoin-js/lib/extrinsics/register-address';
import { createCreditcoinBlockchain } from 'creditcoin-js/lib/transforms';
import { checkAddress, testData } from 'creditcoin-js/lib/testUtils';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { extractFee } from '../utils';

describe('RegisterAddress', () => {
    let ccApi: CreditcoinApi;
    let lender: KeyringPair;

    const { blockchain, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
    });

    afterAll(async () => await ccApi.api.disconnect());

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        return new Promise((resolve, reject) => {
            const wallet = Wallet.createRandom();
            const unsubscribe = api.tx.creditcoin
                .registerAddress(
                    createCreditcoinBlockchain(api, (global as any).CREDITCOIN_ETHEREUM_CHAIN).toJSON(),
                    wallet.address,
                    signAccountId(api, wallet, lender.address),
                )
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

    it('createAddressId works as expected', async (): Promise<void> => {
        const {
            api,
            extrinsics: { registerAddress },
        } = ccApi;

        const lenderWallet = Wallet.createRandom();
        const lenderRegAddr = await registerAddress(
            lenderWallet.address,
            blockchain,
            signAccountId(api, lenderWallet, lender.address),
            lender,
        );

        // manually constructed address is the same as returned by Creditcoin
        const addressId = createAddressId(blockchain, lenderWallet.address);
        expect(addressId).toBe(lenderRegAddr.itemId);

        // manually constructed address should be reported as registered
        const result = await checkAddress(ccApi, addressId);
        expect(result).toBeDefined();
    });
});
