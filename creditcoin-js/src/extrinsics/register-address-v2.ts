import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Address, AddressId, Blockchain, EventReturnJoinType, SignatureType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents } from './common';
import { TxCallback, TxFailureCallback } from '../types';
import { createAddress } from '../transforms';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';

export type AddressRegisteredV2 = EventReturnJoinType<AddressId, Address>;

const blockchainToString = (blockchain: Blockchain): string => {
    return blockchain.toLowerCase();
};

export const createAddressId = (blockchain: Blockchain, externalAddress: string) => {
    const blockchainBytes = Buffer.from(blockchainToString(blockchain));
    const key = u8aConcat(blockchainBytes, u8aToU8a(externalAddress));
    return blake2AsHex(key);
};

export const registerAddressV2 = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: Blockchain,
    ownershipProof: string,
    signatureType: SignatureType,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    api.tx.creditcoin.offchainAddress;
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerAddressV2(blockchain, externalAddress, ownershipProof, signatureType)
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processAddressRegisteredV2 = (api: ApiPromise, result: SubmittableResult): AddressRegisteredV2 => {
    return processEvents(
        api,
        result,
        'AddressRegistered',
        'PalletCreditcoinAddress',
        createAddress,
    ) as AddressRegisteredV2;
};

export const registerAddressV2Async = async (
    api: ApiPromise,
    externalAddress: string,
    blockchain: Blockchain,
    ownershipProof: string,
    signatureType: SignatureType,
    signer: KeyringPair,
): Promise<AddressRegisteredV2> => {
    return new Promise<AddressRegisteredV2>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(processAddressRegisteredV2(api, result));
        registerAddressV2(
            api,
            externalAddress,
            blockchain,
            ownershipProof,
            signatureType,
            signer,
            onSuccess,
            reject,
        ).catch((reason) => reject(reason));
    });
};
