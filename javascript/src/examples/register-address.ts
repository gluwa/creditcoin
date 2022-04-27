// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { GenericEventData } from '@polkadot/types/';
import { PalletCreditcoinAddress } from '@polkadot/types/lookup';

import { handleTransaction, TxOnFail, TxOnSuccess, ownershipProof } from '../utils';

type Blockchain = 'Ethereum' | 'Rinkeby' | 'Luniverse' | 'Bitcoin' | 'Other';

type Address = {
    accountId: string;
    blockchain: Blockchain;
    externalAddress: string;
};

export type RegisteredAddress = {
    addressId: string;
    address: Address;
};

export const registerAddress = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: string,
    signer: KeyringPair,
    account: string,
    onSuccess: TxOnSuccess,
    onFail: TxOnFail,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerAddress(blockchain, externalAddress, ownershipProof(api, signer, account))
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processRegisteredAddress = (api: ApiPromise, result: SubmittableResult): RegisteredAddress | undefined => {
    const { events } = result;
    const addressRegistered = events.find(({ event }) => event.method === 'AddressRegistered');

    const getData = (data: GenericEventData) => {
        const addressId = data[0].toString();
        const { blockchain, owner, value } = api.createType<PalletCreditcoinAddress>(
            'PalletCreditcoinAddress',
            data[1],
        );
        const address = { accountId: owner.toString(), blockchain: blockchain.type, externalAddress: value.toString() };

        return { address, addressId };
    };

    return addressRegistered && getData(addressRegistered.event.data);
};

export const registerAddressAsync = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: string,
    signer: KeyringPair,
    account: string,
) => {
    return new Promise<RegisteredAddress | undefined>((resolve, reject) => {
        const onFail = () => resolve(undefined);
        const onSuccess = (result: SubmittableResult) => resolve(processRegisteredAddress(api, result));

        registerAddress(api, externalAddress, blockchain, signer, account, onSuccess, onFail).catch((reason) =>
            reject(reason),
        );
    });
};
