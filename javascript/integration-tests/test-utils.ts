// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { Guid } from 'js-guid';
import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

import { Blockchain, LoanTerms } from 'credal-js/lib/model';
import { addAskOrderAsync, AskOrderAdded } from 'credal-js/lib/extrinsics/add-ask-order';
import { addBidOrderAsync, BidOrderAdded } from 'credal-js/lib/extrinsics/add-bid-order';
import { registerAddressAsync, AddressRegistered } from 'credal-js/lib/extrinsics/register-address';

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
    blockchain: Blockchain,
    signer: KeyringPair,
): Promise<AddressRegistered> => {
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
    loanTerms: LoanTerms,
    expirationBlock: number,
    askGuid: Guid,
    signer: KeyringPair,
): Promise<AskOrderAdded> => {
    const result = await addAskOrderAsync(api, externalAddress, loanTerms, expirationBlock, askGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('askOrder not found');
    }
};

export const addBidOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: LoanTerms,
    expirationBlock: number,
    bidGuid: Guid,
    signer: KeyringPair,
): Promise<BidOrderAdded> => {
    const result = await addBidOrderAsync(api, externalAddress, loanTerms, expirationBlock, bidGuid, signer);
    expect(result).toBeTruthy();

    if (result) {
        return result;
    } else {
        throw new Error('bidOrder not found');
    }
};
