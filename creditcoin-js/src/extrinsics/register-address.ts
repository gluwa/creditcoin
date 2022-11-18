import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Address, AddressId, Blockchain, CHAINS, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents } from './common';
import { TxCallback, TxFailureCallback } from '../types';
import { createAddress, createCreditcoinBlockchain } from '../transforms';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';

export type AddressRegistered = EventReturnJoinType<AddressId, Address>;

const assertUnreachable = (_x: never): never => {
    throw new Error("Didn't expect to get here");
};

const blockchainToString = (blockchain: Blockchain): string => {
    switch (blockchain) {
        case CHAINS.ethereum:
            return 'ethereum';
        case CHAINS.rinkeby:
            return 'rinkeby';
        case CHAINS.luniverse:
            return 'luniverse';
    }
    switch (blockchain.platform) {
        case 'Evm':
            return `evm-${blockchain.chainId.toString()}`;
        default:
            return assertUnreachable(blockchain.platform);
    }
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
        .registerAddress(createCreditcoinBlockchain(api, blockchain), externalAddress, ownershipProof)
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
