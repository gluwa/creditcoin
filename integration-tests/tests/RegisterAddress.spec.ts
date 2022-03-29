// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

const CREDO_PER_CTC = 1_000_000_000_000_000_000;
const POINT_01_CTC = 0.01 * CREDO_PER_CTC;

describe('RegisterAddress', (): void => {
    let api;
    let alice;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: `sr25519` });
        alice = keyring.addFromUri('//Alice', { name: 'Alice' });
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', (): void => {
        return new Promise(async (resolve) => {
            const unsubscribe = await api.tx.creditcoin
                .registerAddress('Ethereum', '0x3C6a6762f969B36bb1a6DBD598A5DC9800284D77')
                .signAndSend(alice, {nonce: -1}, ({ status, events, dispatchError }) => {

                    if (status.isInBlock) {
                        let balances_Withdraw = events.find(({
                            event: {
                                section,
                                method
                            }
                        }) => {
                            return section === 'balances' && method === 'Withdraw'
                        });

                        expect(balances_Withdraw).toBeTruthy();

                        let _acountId = balances_Withdraw.event.data[0].toString();
                        let fee = balances_Withdraw.event.data[1].toBigInt();

                        unsubscribe();
                        resolve(fee);
                    }
                })
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });

});
