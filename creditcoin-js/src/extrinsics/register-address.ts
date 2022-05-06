import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Address, AddressId, Blockchain, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { TxCallback } from '../types';
import { createAddress } from '../transforms';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';

export type AddressRegistered = EventReturnJoinType<AddressId, Address>;

export const createAddressId = (blockchain: Blockchain, externalAddress: string) => {
    const blockchainBytes = Buffer.from(blockchain.toString().toLowerCase());
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
    onFail: TxCallback,
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
) => {
    return new Promise<AddressRegistered>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processAddressRegistered(api, result));
        registerAddress(api, externalAddress, blockchain, ownershipProof, signer, onSuccess, onFail).catch((reason) =>
            reject(reason),
        );
    });
};
