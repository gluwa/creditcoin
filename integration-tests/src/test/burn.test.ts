import { Blockchain, KeyringPair, creditcoinApi, BN } from 'creditcoin-js';
import { forElapsedBlocks, testData } from 'creditcoin-js/lib/testUtils';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { extractFee, describeIf } from '../utils';

describeIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'burn', () => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;
    let wallet: KeyringPair;

    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { keyring } = testingData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        wallet = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');
    });

    afterAll(async () => await ccApi.api.disconnect());

    it('burn_all works as expected', async (): Promise<void> => {
        const {
            api,
            extrinsics: {},
        } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(wallet.address, new BN('1000000000000000000', 10), 0))
            .signAndSend(sudoSigner, { nonce: -1 });
        await forElapsedBlocks(api);

        const starting = await api.derive.balances.all(wallet.address);
        expect(starting.freeBalance.isZero()).toBe(false);

        await api.tx.creditcoin.burnAll(sudoSigner.address).signAndSend(wallet);
        await forElapsedBlocks(api);

        const ending = await api.derive.balances.all(wallet.address);
        expect(ending.freeBalance.isZero()).toBe(true);
    }, 100_000);

    it('burn works as expected', async (): Promise<void> => {
        const burner = keyring.addFromMnemonic(mnemonicGenerate(12));

        const {
            api,
            extrinsics: {},
        } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(burner.address, new BN('1000000000000000000', 10), 0))
            .signAndSend(sudoSigner, { nonce: -1 });
        await forElapsedBlocks(api);

        const starting = await api.derive.balances.all(burner.address);
        expect(starting.freeBalance.isZero()).toBe(false);

        await api.tx.creditcoin.burn(100, sudoSigner.address).signAndSend(burner);
        await forElapsedBlocks(api);

        const ending = await api.derive.balances.all(burner.address);
        expect(starting.freeBalance.gt(ending.freeBalance)).toBe(true);
    }, 100_000);

    it('burn_all fee is min 0.01 CTC', async (): Promise<void> => {
        const {
            api,
            extrinsics: {},
        } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(sudoSigner.address, new BN('1000000000000000000', 10), 0))
            .signAndSend(sudoSigner, { nonce: -1 });
        await forElapsedBlocks(api);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.sudo
                .sudo(api.tx.creditcoin.burnAll(sudoSigner.address))
                .signAndSend(sudoSigner, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    }, 150_000);

    it('burn fee is min 0.01 CTC', async (): Promise<void> => {
        const burner = keyring.addFromMnemonic(mnemonicGenerate(12));

        const {
            api,
            extrinsics: {},
        } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(sudoSigner.address, new BN('1000000000000000000', 10), 0))
            .signAndSend(sudoSigner, { nonce: -1 });
        await forElapsedBlocks(api);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.sudo
                .sudo(api.tx.creditcoin.burn(100, burner.address))
                .signAndSend(sudoSigner, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    }, 150_000);
});
