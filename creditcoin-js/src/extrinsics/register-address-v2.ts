import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Address, AddressId, Blockchain, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents } from './common';
import { TxCallback, TxFailureCallback } from '../types';
import { createAddress } from '../transforms';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { PalletCreditcoinOwnershipProof } from '@polkadot/types/lookup';
import { OwnershipProof } from '../model';

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
    ownershipProof: OwnershipProof,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const proof = createCreditCoinOwnershipProof(api, ownershipProof);
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerAddressV2(blockchain, externalAddress, proof)
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
    ownershipProof: OwnershipProof,
    signer: KeyringPair,
): Promise<AddressRegisteredV2> => {
    return new Promise<AddressRegisteredV2>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(processAddressRegisteredV2(api, result));
        registerAddressV2(api, externalAddress, blockchain, ownershipProof, signer, onSuccess, reject).catch((reason) =>
            reject(reason),
        );
    });
};

export const createCreditCoinOwnershipProof = (
    api: ApiPromise,
    proof: OwnershipProof,
): PalletCreditcoinOwnershipProof => {
    const toType = (): unknown => {
        switch (proof.kind) {
            case 'PersonalSign':
                return { PersonalSign: proof.signature }; // eslint-disable-line  @typescript-eslint/naming-convention
            case 'EthSign':
                return { EthSign: proof.signature }; // eslint-disable-line  @typescript-eslint/naming-convention
        }
    };

    return api.createType('PalletCreditcoinOwnershipProof', toType());
};

export const ethSignSignature = (signature: string): OwnershipProof => {
    return { kind: 'EthSign', signature };
};

export const personalSignSignature = (signature: string): OwnershipProof => {
    return { kind: 'PersonalSign', signature };
};
