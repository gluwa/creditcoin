// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';

import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { registerAddressAsync } from './examples/register-address';
import { registerDealOrderAsync, signLoanParams } from './examples/register-deal-order';
import { randomEthAddress } from './utils';

const main = async () => {
    let api: ApiPromise | undefined;
    try {
        const provider = new WsProvider('ws://127.0.0.1:9944');
        api = await ApiPromise.create({ provider });
        const keyring = new Keyring({ type: 'sr25519' });
        const lender = keyring.addFromUri('//Alice', { name: 'Alice' });
        const borrower = keyring.addFromUri('//Bob', { name: 'Bob' });

        const lenderAddress = randomEthAddress();
        const borrowerAddress = randomEthAddress();

        const askGuid = Guid.newGuid().toString();
        const bidGuid = Guid.newGuid().toString();
        const expBlock = 5;
        const blockchain = 'Ethereum';
        const loanTerms = api.createType<PalletCreditcoinLoanTerms>('PalletCreditcoinLoanTerms', {
            amount: 1_000,
            interestRate: {
                ratePerPeriod: 100,
                decimals: 4,
                period: {
                    secs: 60 * 60,
                    nanos: 0,
                },
            },
            termLength: {
                secs: 60 * 60,
                nanos: 0,
            },
        });

        const signedParams = signLoanParams(api, borrower, expBlock, askGuid, bidGuid, loanTerms);

        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            registerAddressAsync(api, lenderAddress, blockchain, lender),
            registerAddressAsync(api, borrowerAddress, blockchain, borrower),
        ]);

        console.log('Lender registered: ', lenderRegAddr);
        console.log('Borrower registered: ', borrowerRegAddr);

        const regDealOrderCompleted = await registerDealOrderAsync(
            api,
            lenderRegAddr?.addressId || '',
            borrowerRegAddr?.addressId || '',
            loanTerms,
            expBlock,
            askGuid,
            bidGuid,
            borrower.publicKey,
            signedParams,
            lender,
        );

        console.log(`Deal order registered: ${regDealOrderCompleted.toString()}`);
    } finally {
        if (api) {
            await api.disconnect();
        }
    }
};

main().catch(console.error);
