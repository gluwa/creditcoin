import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/examples/setup-authority';
import { createCollectedCoinsId } from 'creditcoin-js/extrinsics/request-collect-coins';
import { createAddressId } from 'creditcoin-js/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';
import { testIf } from '../utils';

describe('CollectCoins', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { keyring, blockchain } = testData;
    const evmAddress = '0xffffffffffffffffffffffffffffffffffffffff';
    const badHash = '0xbad';
    const addressId = createAddressId(blockchain, evmAddress);

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            authority = keyring.createFromUri(AUTHORITY_SURI);
        }
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    describe('request', (): void => {
        testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;

            const { partialFee } = await api.tx.creditcoin
                .requestCollectCoins(evmAddress, badHash)
                .paymentInfo(authority, { nonce: -1 });
            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

    describe('fail', (): void => {
        testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;
            const collectedCoinsId = createCollectedCoinsId(evmAddress);
            const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TaskFailed');

            // eslint-disable-next-line @typescript-eslint/naming-convention
            const taskId = api.createType('PalletCreditcoinTaskId', { CollectCoins: collectedCoinsId });

            const { partialFee } = await api.tx.creditcoin
                .failTask(1000, taskId, cause)
                .paymentInfo(authority, { nonce: -1 });

            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

    describe('persist', (): void => {
        testIf(
            (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY,
            'fee is min 0.01 CTC but bypassed by OCW',
            async (): Promise<void> => {
                const { api } = ccApi;
                const collectedCoins = {
                    to: addressId,
                    amount: 1000,
                    txHash: badHash,
                };
                const collectedCoinsId = createCollectedCoinsId(evmAddress);
                /* eslint-disable @typescript-eslint/naming-convention */
                const taskOutput = api.createType('PalletCreditcoinTaskOutput', {
                    CollectCoins: [collectedCoinsId, collectedCoins],
                });

                const { partialFee } = await api.tx.creditcoin
                    .persistTaskOutput(1000, taskOutput)
                    .paymentInfo(authority, { nonce: -1 });
                /* eslint-enable */
                expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
            },
        );
    });
});
