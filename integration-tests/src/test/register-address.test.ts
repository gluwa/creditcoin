import { ApiPromise, Keyring, WsProvider, KeyringPair, Wallet, POINT_01_CTC } from 'creditcoin-js';
import { createCreditcoinBlockchain } from 'creditcoin-js/lib/transforms';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { extractFee } from '../utils';

describe('RegisterAddress', () => {
    let api: ApiPromise;
    let alice: KeyringPair;

    beforeAll(async () => {
        api = await ApiPromise.create({
            provider: new WsProvider((global as any).CREDITCOIN_API_URL),
        });
        alice = new Keyring({ type: 'sr25519' }).addFromUri('//Alice');
    });

    afterAll(async () => await api.disconnect());

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            const wallet = Wallet.createRandom();
            const unsubscribe = api.tx.creditcoin
                .registerAddress(
                    createCreditcoinBlockchain(api, (global as any).CREDITCOIN_ETHEREUM_CHAIN).toJSON(),
                    wallet.address,
                    signAccountId(api, wallet, alice.address),
                )
                .signAndSend(alice, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
