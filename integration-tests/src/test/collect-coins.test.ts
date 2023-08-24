import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/lib/examples/setup-authority';
import { createCollectedCoinsId } from 'creditcoin-js/lib/extrinsics/request-collect-coins';
import { AddressRegistered, createAddressId } from 'creditcoin-js/lib/extrinsics/register-address';
import { creditcoinApi, providers, Wallet } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData, tryRegisterAddress } from 'creditcoin-js/lib/testUtils';
import { testIf } from '../utils';

describe('CollectCoins', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { keyring, blockchain } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
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
        let deployerWallet: Wallet;
        let deployerRegAddr: AddressRegistered;

        beforeAll(async () => {
            const {
                api,
                utils: { signAccountId },
            } = ccApi;

            collector = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');

            /* eslint-disable @typescript-eslint/naming-convention */
            const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsGCreContract', {
                address: (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
                chain: blockchain,
            });

            await api.tx.sudo
                .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
                .signAndSend(collector, { nonce: -1 });

            const provider = new providers.JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
            deployerWallet = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
            deployerRegAddr = await tryRegisterAddress(
                ccApi,
                deployerWallet.address,
                blockchain,
                signAccountId(deployerWallet, collector.address),
                collector,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            );
        }, 300_000);

        testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;

            const { partialFee } = await api.tx.creditcoin
                .requestCollectCoins(evmAddress, badHash)
                .paymentInfo(authority, { nonce: -1 });
            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });

        it('000 - with mixed up Ethereum addresses should throw IncorrectSender error', async (): Promise<void> => {
            const {
                extrinsics: { requestCollectCoins },
                utils: { signAccountId },
            } = ccApi;

            // register a second Ethereum Wallet with the same Creditcoin account
            const secondWallet = Wallet.createRandom();
            const secondRegAddr = await tryRegisterAddress(
                ccApi,
                secondWallet.address,
                blockchain,
                signAccountId(secondWallet, collector.address),
                collector,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            );
            // the two external addresses must be different
            expect(secondRegAddr.item.externalAddress).not.toBe(deployerRegAddr.item.externalAddress);

            // send a collect coins transaction using the 2nd Ethereum address
            // and the burn tx hash from the 1st Ethereum address.
            // IMPORTANT: Both Ethereum wallets are registered to collector on Creditcoin.
            const collectCoinsEvent = await requestCollectCoins(
                secondRegAddr.item.externalAddress,
                collector,
                (global as any).CREDITCOIN_CTC_BURN_TX_HASH,
            );

            // eventhough collector (a Creditcoin account) has control over both Ethereum wallets
            // they can't collect coins using a burn tx hash which was sent from a their 1st wallet
            await expect(collectCoinsEvent.waitForVerification(800_000)).rejects.toThrow(/IncorrectSender/);
        }, 900_000);

        // WARNING: this scenario should always be executed *AFTER* the one above because
        // they use the same burn tx hash value ! If this is executed first the above one
        // will fail with CollectCoinsAlreadyRegistered instead of the expected IncorrectSender !!!
        it('001 - end-to-end', async (): Promise<void> => {
            const {
                extrinsics: { requestCollectCoins },
            } = ccApi;

            const collectCoinsEvent = await requestCollectCoins(
                deployerRegAddr.item.externalAddress,
                collector,
                (global as any).CREDITCOIN_CTC_BURN_TX_HASH,
            );

            const collectCoinsVerified = await collectCoinsEvent.waitForVerification(800_000).catch();
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
            await expect(collectCoinsEvent.waitForVerification(800_000)).rejects.toThrow(/TransactionNotFound/);
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

            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
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
                expect(partialFee.toBigInt()).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
            },
        );
    });
});
