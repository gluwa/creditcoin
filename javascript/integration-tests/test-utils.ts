// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinLoanTerms,
} from '@polkadot/types/lookup';

import { addAskOrderAsync } from '../src/examples/add-ask-order';
import { addBidOrderAsync } from '../src/examples/add-bid-order';
import { registerAddressAsync, RegisteredAddress } from '../src/examples/register-address';

export const expectNoDispatchError = (api: ApiPromise, dispatchError: any): void => {
    if (dispatchError) {
        if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            expect(`${section}.${name}: ${docs.join(' ')}`).toBe('');
        } else {
            expect(dispatchError.toString()).toBe('');
        }
    }
};

export const registerAddress = async (
    api: ApiPromise,
    ethAddress: string,
    blockchain: string,
    signer: KeyringPair,
): Promise<RegisteredAddress> => {
    const addr = await registerAddressAsync(api, ethAddress, blockchain, signer);
    expect(addr).toBeTruthy();

    if (addr) {
        expect(addr.addressId).toBeTruthy();
        return addr;
    } else {
        throw new Error("Address wasn't registered successfully");
    }
};

export const addAskOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    askGuid: string,
    signer: KeyringPair,
): Promise<PalletCreditcoinAskOrderId> => {
    const result = await addAskOrderAsync(api, externalAddress, loanTerms, expirationBlock, askGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('askOrderId not found');
    }
};

export const addBidOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    bidGuid: string,
    signer: KeyringPair,
): Promise<PalletCreditcoinBidOrderId> => {
    const result = await addBidOrderAsync(api, externalAddress, loanTerms, expirationBlock, bidGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('bidOrderId not found');
    }
};
