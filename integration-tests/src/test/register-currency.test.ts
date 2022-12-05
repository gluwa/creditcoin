import { KeyringPair, creditcoinApi, POINT_01_CTC } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, testIf } from '../utils';

describe('RegisterCurrency', (): void => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;

    const { keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        }
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        /* eslint-disable @typescript-eslint/naming-convention */
        const currency = api.createType('PalletCreditcoinPlatformCurrency', {
            Evm: api.createType('PalletCreditcoinPlatformEvmCurrencyType', {
                SmartContract: ['0x0000000000000000000000000000000000000000', ['Ethless']],
            }),
        });

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.sudo
                .sudo(api.tx.creditcoin.registerCurrency(currency))
                .signAndSend(sudoSigner, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
