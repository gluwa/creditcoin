import { creditcoinApi, KeyringPair } from 'creditcoin-js';
import { AUTHORITY_SURI } from 'creditcoin-js/lib/examples/setup-authority';
import { createFundingTransferId } from 'creditcoin-js/lib/extrinsics/register-transfers';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, testIf } from '../utils';

describe('FailTransfer', (): void => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;

    const { blockchain, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            authority = keyring.createFromUri(AUTHORITY_SURI);
        }
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const transferId = createFundingTransferId(blockchain, '0xffffffffffffffffffffffffffffffffffffffff');
        const cause = api.createType('PalletCreditcoinOcwErrorsVerificationFailureCause', 'TaskFailed');

        // eslint-disable-next-line @typescript-eslint/naming-convention
        const taskId = api.createType('PalletCreditcoinTaskId', { VerifyTransfer: transferId });

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .failTask(1000, taskId, cause)
                .signAndSend(authority, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual((global as any).CREDITCOIN_MINIMUM_TXN_FEE);
        });
    });
});
