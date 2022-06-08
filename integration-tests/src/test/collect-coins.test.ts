import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/examples/setup-authority';
import { createCollectCoinsId } from 'creditcoin-js/extrinsics/request-collect-coins';
import { createAddressId } from 'creditcoin-js/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';

describe('CollectCoins', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { keyring, blockchain } = testData;
    const evmAddress = '0xffffffffffffffffffffffffffffffffffffffff';
    const badHash = '0xbad';
    const addressId = createAddressId(blockchain, evmAddress);

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        authority = keyring.createFromUri(AUTHORITY_SURI);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    describe('request', (): void => {
        it('fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;

            const { partialFee } = await api.tx.creditcoin
                .requestCollectCoins(evmAddress, badHash)
                .paymentInfo(authority, { nonce: -1 });
            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

    describe('fail', (): void => {
        it('fee is min 0.01 CTC', async (): Promise<void> => {
            const { api } = ccApi;
            const collectCoinsId = createCollectCoinsId(evmAddress);
            const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TaskFailed');

            const { partialFee } = await api.tx.creditcoin
                .failCollectCoins(collectCoinsId, cause, 1000)
                .paymentInfo(authority, { nonce: -1 });

            expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

    describe('persist', (): void => {
        it('fee is min 0.01 CTC but bypassed by OCW', async (): Promise<void> => {
            const { api } = ccApi;
            const collectCoins = {
                to: addressId,
                amount: 1000,
                txHash: badHash,
            };

            const { partialFee } = await api.tx.creditcoin
                .persistCollectCoins(collectCoins, 1000)
                .paymentInfo(authority, { nonce: -1 });
            expect(partialFee.toBigInt()).toEqual(BigInt(0));
        });
    });
});
