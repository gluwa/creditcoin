import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/lib/examples/setup-authority';
import { createCollectedCoinsId } from 'creditcoin-js/lib/extrinsics/request-collect-coins';
import { AddressRegistered, createAddressId } from 'creditcoin-js/lib/extrinsics/register-address';
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
        let collector: KeyringPair;
        let deployerRegAddr: AddressRegistered;

        beforeAll(async () => {
            const { api } = ccApi;

            collector = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');

            /* eslint-disable @typescript-eslint/naming-convention */
            const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsGCreContract', {
                address: (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
                chain: blockchain,
            });

            await api.tx.sudo
                .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
                .signAndSend(collector, { nonce: -1 });

            deployerRegAddr = await registerCtcDeployerAddress(
                ccApi,
                (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY,
            );
        }, 300_000);

        testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;

            const { partialFee } = await api.tx.creditcoin
                .requestCollectCoins(evmAddress, badHash)
                .paymentInfo(authority, { nonce: -1 });
            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
        });

        it('end-to-end', async (): Promise<void> => {
            const {
                extrinsics: { requestCollectCoins },
            } = ccApi;

            const collectCoinsEvent = await requestCollectCoins(
                deployerRegAddr.item.externalAddress,
                collector,
                (global as any).CREDITCOIN_CTC_BURN_TX_HASH,
            );
            const collectCoinsVerified = await collectCoinsEvent.waitForVerification(600_000).catch();
            expect(collectCoinsVerified).toBeTruthy();

            // try again - should fail
            await expect(
                requestCollectCoins(
                    deployerRegAddr.item.externalAddress,
                    collector,
                    (global as any).CREDITCOIN_CTC_BURN_TX_HASH,
                ),
            ).rejects.toThrow(
                'creditcoin.CollectCoinsAlreadyRegistered: The coin collection has already been registered',
            );
        }, 900_000);

        it('should throw TransactionNotFound when txHash not found', async (): Promise<void> => {
            const {
                extrinsics: { requestCollectCoins },
            } = ccApi;

            const collectCoinsEvent = await requestCollectCoins(
                deployerRegAddr.item.externalAddress,
                collector,
                '0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
            );
            await expect(collectCoinsEvent.waitForVerification()).rejects.toThrow(/TransactionNotFound/);
        }, 900_000);
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
