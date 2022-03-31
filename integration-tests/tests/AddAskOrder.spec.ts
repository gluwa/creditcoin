// [object Object]
// SPDX-License-Identifier: Apache-2.0

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';
import { Guid } from 'js-guid';

import { randomEthAddress } from '../../polkadotjs-examples/src/utils';
import { registerAddressAsync } from '../../polkadotjs-examples/src/examples/register-address';

import { POINT_01_CTC } from '../src/constants';

describe('AddAskOrder', (): void => {
    let api;
    let lender;
    let loanTerms;
    let lenderRegAddr;
    let askGuid;

    const blockchain = 'Ethereum';
    const expirationBlock = 5;

    beforeEach(async () => {
        process.env.NODE_ENV = 'test';

        const provider = new WsProvider('ws://127.0.0.1:9944');

        api = await ApiPromise.create({ provider });

        const keyring = new Keyring({ type: `sr25519` });
        lender = keyring.addFromUri('//Alice', { name: 'Alice' });
        const lenderAddress = randomEthAddress();
        loanTerms = api.createType<PalletCreditcoinLoanTerms>('PalletCreditcoinLoanTerms', {
            amount: 1_000,
            interestRate: 100,
            maturity: 10,
        });

        lenderRegAddr = await registerAddressAsync(api, lenderAddress, blockchain, lender);
        expect(lenderRegAddr?.addressId).toBeTruthy();

        askGuid = Guid.newGuid().toString();
    });

    afterEach(async () => {
        await api.disconnect();
    });

    it('fee is min 0.01 CTC', (): void => {
        return new Promise(async (resolve) => {
            const unsubscribe = await api.tx.creditcoin
                .addAskOrder(lenderRegAddr?.addressId, loanTerms, expirationBlock, askGuid)
                .signAndSend(lender, {nonce: -1}, ({ status, events, dispatchError }) => {

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
