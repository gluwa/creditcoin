import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Wallet } from 'ethers';
import { signAccountId } from 'creditcoin-js/utils';
import { POINT_01_CTC } from '../constants';
import { extractFee } from '../utils';

describe('RegisterAddress', () => {
    let api: ApiPromise;
    let alice: KeyringPair;

    beforeAll(async () => {
        process.env.NODE_ENV = 'test';

        api = await ApiPromise.create({
            provider: new WsProvider('ws://127.0.0.1:9944'),
        });
        alice = new Keyring({ type: 'sr25519' }).addFromUri('//Alice');
    });

    afterAll(async () => await api.disconnect());

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        return new Promise((resolve, reject) => {
            const wallet = Wallet.createRandom();
            const unsubscribe = api.tx.creditcoin
                .registerAddress('Ethereum', wallet.address, signAccountId(api, wallet, alice.address))
                .signAndSend(alice, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
