import { KeyringPair } from 'creditcoin-js';
import { POINT_01_CTC } from '../constants';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData } from './common';
import { extractFee, testIf } from '../utils';
import { createCreditcoinBlockchain } from 'creditcoin-js/lib/transforms';

describe('SetCollectCoinsContract', (): void => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;

    const { keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = keyring.addFromUri('//Alice');
        }
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        /* eslint-disable @typescript-eslint/naming-convention */
        const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsGCreContract', {
            address: '0xa3EE21C306A700E682AbCdfe9BaA6A08F3820419',
            chain: createCreditcoinBlockchain(api, testData.blockchain),
        });

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.sudo
                .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
                .signAndSend(sudoSigner, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
