import { Blockchain, KeyringPair, Wallet, creditcoinApi } from 'creditcoin-js';
import {
    createAddressId,
    ethSignSignature,
    createCreditCoinOwnershipProof,
} from 'creditcoin-js/lib/extrinsics/register-address-v2';
import { checkAddress, testData } from 'creditcoin-js/lib/testUtils';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { extractFee } from '../utils';
import { registerAddressV2Example } from 'creditcoin-js/lib/examples/register-address-v2';

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
            const accountId = signAccountId(api, wallet, lender.address);
            const ownershipProof = ethSignSignature(accountId);
            const proof = createCreditCoinOwnershipProof(api, ownershipProof);

            const unsubscribe = api.tx.creditcoin
                .registerAddressV2(blockchain, wallet.address, proof)
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    });

    it('registerAddressV2 EthSign works as expected', async (): Promise<void> => {
        const {
            api,
            extrinsics: { registerAddressV2 },
        } = ccApi;

        const lenderWallet = Wallet.createRandom();
        const accountId = signAccountId(api, lenderWallet, lender.address);
        const proof = ethSignSignature(accountId);

        const lenderRegAddr = await registerAddressV2(lenderWallet.address, blockchain, proof, lender);

        // manually constructed address is the same as returned by Creditcoin
        const addressId = createAddressId(blockchain, lenderWallet.address);
        expect(addressId).toBe(lenderRegAddr.itemId);

        // manually constructed address should be reported as registered
        const result = await checkAddress(ccApi, addressId);
        expect(result).toBeDefined();
    });

    it('registerAddressV2 PersonalSign works as expected', async (): Promise<void> => {
        const lenderWallet = Wallet.createRandom();

        const lenderRegAddr = await registerAddressV2Example(ccApi, lenderWallet, lender, blockchain);

        // manually constructed address is the same as returned by Creditcoin
        const addressId = createAddressId(blockchain, lenderWallet.address);
        expect(addressId).toBe(lenderRegAddr.itemId);

        // manually constructed address should be reported as registered
        const result = await checkAddress(ccApi, addressId);
        expect(result).toBeDefined();
    });
});
