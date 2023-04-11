import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Address, AddressId, Blockchain, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents } from './common';
import { TxCallback, TxFailureCallback } from '../types';
import { createAddress } from '../transforms';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';

export type AddressRegistered = EventReturnJoinType<AddressId, Address>;

const blockchainToString = (blockchain: Blockchain): string => {
    return blockchain.toLowerCase();
};

export const createAddressId = (blockchain: Blockchain, externalAddress: string) => {
    const blockchainBytes = Buffer.from(blockchainToString(blockchain));
    const key = u8aConcat(blockchainBytes, u8aToU8a(externalAddress));
    return blake2AsHex(key);
};

export const registerAddress = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: Blockchain,
    ownershipProof: string,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerAddress(blockchain, externalAddress, ownershipProof)
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processAddressRegistered = (api: ApiPromise, result: SubmittableResult): AddressRegistered => {
    return processEvents(
        api,
        result,
        'AddressRegistered',
        'PalletCreditcoinAddress',
        createAddress,
    ) as AddressRegistered;
};

export const registerAddressAsync = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: Blockchain,
    ownershipProof: string,
    signer: KeyringPair,
): Promise<AddressRegistered> => {
    return new Promise<AddressRegistered>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(processAddressRegistered(api, result));
        registerAddress(api, externalAddress, blockchain, ownershipProof, signer, onSuccess, reject).catch((reason) =>
            reject(reason),
        );
    });
};
