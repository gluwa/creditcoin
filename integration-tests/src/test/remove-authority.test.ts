import { common, creditcoinApi, ApiPromise, Balance, KeyringPair } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { testData } from 'creditcoin-js/lib/testUtils';
import { addAuthorityAsync } from 'creditcoin-js/lib/extrinsics/add-authority';

import { testIf } from '../utils';

const { expectNoEventError, expectNoDispatchError } = common;

const globals = global as any;

describe('RemoveAuthority', (): void => {
    let api: ApiPromise;
    let sudoSigner: KeyringPair;
    let authority: KeyringPair;

    const { keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        api = (await creditcoinApi((global as any).CREDITCOIN_API_URL)).api;
        if (globals.CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
            authority = keyring.addFromUri('//Auth');
            await addAuthorityAsync(api, authority.address, sudoSigner);
        }
    });

    afterAll(async () => {
        await api.disconnect();
    });

    testIf((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY, 'fee is 0', async (): Promise<void> => {
        const accountId = authority.address;
        const sudoCall = api.tx.sudo.sudo(api.tx.creditcoin.removeAuthority(accountId));
        const predicate = (fee: unknown) => expect(fee).toEqual(BigInt(0));

        return new Promise((resolve, _reject) => {
            const unsubscribe = sudoCall.signAndSend(
                sudoSigner,
                { nonce: -1 },
                async ({ dispatchError, events, status }) => {
                    expectNoDispatchError(api, dispatchError);
                    if (!status.isInBlock) return;
                    (await unsubscribe)();

                    events.forEach((event) => expectNoEventError(api, event));
                    const netFee = events
                        .filter(({ event: { section } }) => {
                            return section === 'balances';
                        })
                        .map(({ event: { method, data } }) => {
                            const transform = (x: any) => (x[1] as Balance).toBigInt();
                            if (method === 'Withdraw') return -transform(data);
                            else if (method === 'Deposit') return transform(data);
                            else throw new Error('Unhandled balances event');
                        })
                        .reduce((prev, curr, _index, _array) => {
                            return prev + curr;
                        }, BigInt(0));

                    resolve(netFee);
                },
            );
        }).then(predicate);
    });
});
