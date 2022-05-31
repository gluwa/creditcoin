import { KeyringPair } from '@polkadot/keyring/types';
import { AUTHORITY_SURI } from 'creditcoin-js/examples/setup-authority';
import { createCollectcoinsId } from 'creditcoin-js/extrinsics/request-collectcoins';
import { createAddressId } from 'creditcoin-js/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { Blockchain, creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';

describe('CollectCoins', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { keyring } = testData;
    const evmAddress = '0xffffffffffffffffffffffffffffffffffffffff';
    const badHash = '0xbad';
    const blockchain: Blockchain = 'Ethereum';
    const addressId = createAddressId(blockchain, evmAddress);

    beforeAll(async () => {
        process.env.NODE_ENV = 'test';
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        authority = keyring.createFromUri(AUTHORITY_SURI);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    it('fail; fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const collectcoinsId = createCollectcoinsId(evmAddress);
        const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TaskFailed');

        const { partialFee } = await api.tx.creditcoin
            .failCollectCoins(collectcoinsId, cause, 1000)
            .paymentInfo(authority, { nonce: -1 });

        expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
    });

    it('request; fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        const { partialFee } = await api.tx.creditcoin
            .requestCollectCoins(evmAddress, badHash)
            .paymentInfo(authority, { nonce: -1 });
        expect(partialFee.toBigInt()).toBeGreaterThanOrEqual(POINT_01_CTC);
    });

    it('persist; fee is min 0.01 CTC but bypassed by OCW', async (): Promise<void> => {
        const { api } = ccApi;
        const collectcoins = {
            to: addressId,
            amount: 1000,
            txHash: badHash,
        };

        const { partialFee } = await api.tx.creditcoin
            .persistCollectCoins(collectcoins, 1000)
            .paymentInfo(authority, { nonce: -1 });
        expect(partialFee.toBigInt()).toEqual(BigInt(0));
    });
});
