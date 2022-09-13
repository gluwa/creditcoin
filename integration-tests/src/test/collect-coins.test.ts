import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/lib/examples/setup-authority';
import { createCollectedCoinsId } from 'creditcoin-js/lib/extrinsics/request-collect-coins';
import { createAddressId } from 'creditcoin-js/lib/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData, registerCtcDeployerAddress } from './common';
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

        it('end-to-end', async (): Promise<void> => {
            const {
                api,
                extrinsics: { requestCollectCoins },
            } = ccApi;

            /* eslint-disable @typescript-eslint/naming-convention */
            const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsGCreContract', {
                address: (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
                chain: blockchain,
            });

            const collector = keyring.addFromUri('//Alice');

            await api.tx.sudo
                .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
                .signAndSend(collector, { nonce: -1 });

            const deployerRegAddr = await registerCtcDeployerAddress(
                ccApi,
                (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY,
            );

            const collectCoinsEvent = await requestCollectCoins(
                deployerRegAddr.item.externalAddress,
                collector,
                (global as any).CREDITCOIN_CTC_BURN_TX_HASH,
            );
            const collectCoinsVerified = await collectCoinsEvent.waitForVerification().catch();
            expect(collectCoinsVerified).toBeTruthy();
        }, 900000);
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
