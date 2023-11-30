import { KeyringPair, creditcoinApi } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, describeIf } from '../utils';

describeIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'SetGateFaucet', (): void => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;
    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { keyring } = testingData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'sudo');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.sudo
                // for testing purposes I can use any address b/c I'm only interested in the transaction fee
                .sudo(api.tx.creditcoin.setGateFaucet(sudoSigner.address))
                .signAndSend(sudoSigner, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    });
});
