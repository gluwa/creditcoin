import { ApiPromise, SubmittableResult } from '@polkadot/api';
import {
    CollectedCoins,
    UnverifiedCollectedCoins,
    CollectedCoinsId,
    ExternalAddress,
    EventReturnJoinType,
    CollectCoinsContract,
} from '../model';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents, listenForVerificationOutcome } from './common';
import { TxCallback, TxFailureCallback, VerificationError } from '..';
import { createCollectedCoins, createUnverifiedCollectedCoins } from '../transforms';
import { PalletCreditcoinCollectCoinsTokenContract } from '@polkadot/types/lookup';
import { CollectCoinsEvent, CollectCoinsEventKind, createCollectCoinsRegisteredEvent } from './request-collect-coins';

export const createTokenContract = (
    api: ApiPromise,
    contract: CollectCoinsContract,
): PalletCreditcoinCollectCoinsTokenContract => {
    const toType = () => {
        switch (contract.kind) {
            case 'GCRE':
                return { GCRE: [contract.evmAddress, contract.txHash] };
            case 'GATE':
                return { GATE: [contract.evmAddress, contract.txHash] };
        }
    };

    return api.createType('PalletCreditcoinCollectCoinsTokenContract', toType());
};

export const GATEContract = (evmAddress: ExternalAddress, txHash: string): CollectCoinsContract => {
    return { kind: 'GATE', evmAddress, txHash };
};

export const GCREContract = (evmAddress: ExternalAddress, txHash: string): CollectCoinsContract => {
    return { kind: 'GCRE', evmAddress, txHash };
};

export const requestCollectCoinsV2 = async (
    api: ApiPromise,
    contract: CollectCoinsContract,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const formatted_contract = createTokenContract(api, contract);

    const unsubscribe: () => void = await api.tx.creditcoin
        .requestCollectCoinsV2(formatted_contract)
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const requestCollectCoinvsV2Async = async (
    api: ApiPromise,
    contract: CollectCoinsContract,
    signer: KeyringPair,
) => {
    return new Promise<CollectCoinsEvent>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) =>
            resolve(createCollectCoinsRegisteredEvent(api, result, 'CollectCoinsRegistered'));

        requestCollectCoinsV2(api, contract, signer, onSuccess, reject).catch((reason) => {
            reject(reason);
        });
    });
};
