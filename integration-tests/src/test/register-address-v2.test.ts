import { Blockchain, KeyringPair, Wallet, creditcoinApi } from 'creditcoin-js';
import { createAddressId } from 'creditcoin-js/lib/extrinsics/register-address';
import { checkAddress, testData } from 'creditcoin-js/lib/testUtils';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { signAccountId, personalSignAccountId } from 'creditcoin-js/lib/utils';
import { extractFee } from '../utils';
import { account } from '@polkadot/api-derive/balances';


describe('RegisterAddressV2', () => {
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
                .registerAddressV2(blockchain, wallet.address, signAccountId(api, wallet, lender.address), "EthSign")
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    });

    // it('registerAddressV2 EthSign works as expected', async (): Promise<void> => {
    //     const {
    //         api,
    //         extrinsics: { registerAddressV2 },
    //     } = ccApi;

    //     const lenderWallet = Wallet.createRandom();
    //     const accountId = signAccountId(api, lenderWallet, lender.address)
    //     const lenderRegAddr = await registerAddressV2(
    //         lenderWallet.address,
    //         blockchain,
    //         accountId,
    //         "EthSign",
    //         lender,
    //     );

    //     // manually constructed address is the same as returned by Creditcoin
    //     const addressId = createAddressId(blockchain, lenderWallet.address);
    //     expect(addressId).toBe(lenderRegAddr.itemId);

    //     // manually constructed address should be reported as registered
    //     const result = await checkAddress(ccApi, addressId);
    //     expect(result).toBeDefined();
    // }, 100000);

    it('registerAddressV2 PersonalSign works as expected', async (): Promise<void> => {
        const {
            api,
            extrinsics: { registerAddressV2 },
        } = ccApi;

        const lenderWallet = Wallet.createRandom();
        const accountId = await personalSignAccountId(api, lenderWallet, lender.addressRaw);

        const lenderRegAddr = await registerAddressV2(
            lenderWallet.address,
            blockchain,
            accountId,
            "PersonalSign",
            lender,
        );

        // manually constructed address is the same as returned by Creditcoin
        const addressId = createAddressId(blockchain, lenderWallet.address);
        expect(addressId).toBe(lenderRegAddr.itemId);

        // manually constructed address should be reported as registered
        const result = await checkAddress(ccApi, addressId);
        expect(result).toBeDefined();
    }, 100000);

});
