import { KeyringPair } from '@polkadot/keyring/types';
import { createCreditcoinLoanTerms } from 'creditcoin-js/transforms';
import { AUTHORITY_SURI } from 'creditcoin-js/examples/setup-authority';
import { AddressRegistered } from 'creditcoin-js/extrinsics/register-address';
import { createFundingTransferId } from 'creditcoin-js/extrinsics/register-transfers';
import { POINT_01_CTC } from '../constants';
import { signAccountId } from 'creditcoin-js/utils';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData } from './common';
import { extractFee } from '../utils';

describe('FailTransfer', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { blockchain, expirationBlock, loanTerms, keyring } = testData;

    beforeAll(async () => {
        process.env.NODE_ENV = 'test';
        ccApi = await creditcoinApi('ws://127.0.0.1:9944');
        authority = keyring.createFromUri(AUTHORITY_SURI);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const transferId = createFundingTransferId(blockchain, '0xffffffffffffffffffffffffffffffffffffffff');
        const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TransferFailed');

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .failTransfer(transferId, cause)
                .signAndSend(authority, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((reason) => reject(reason));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
