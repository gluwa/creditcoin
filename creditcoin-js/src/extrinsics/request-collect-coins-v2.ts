import { ApiPromise, SubmittableResult } from '@polkadot/api';
import {
    ExternalAddress,
    CollectCoinsContract,
} from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, } from './common';
import { TxCallback, TxFailureCallback, } from '..';
import { PalletCreditcoinCollectCoinsTokenContract } from '@polkadot/types/lookup';
import { CollectCoinsEvent, createCollectCoinsRegisteredEvent } from './request-collect-coins';

export const createTokenContract = (
    api: ApiPromise,
    contract: CollectCoinsContract,
): PalletCreditcoinCollectCoinsTokenContract => {
    const toType = () => {
        switch (contract.kind) {
            case 'GCRE':
                return { GCRE: [contract.evmAddress, contract.txHash] }; // eslint-disable-line @typescript-eslint/naming-convention
            case 'GATE':
                return { GATE: [contract.evmAddress, contract.txHash] }; // eslint-disable-line @typescript-eslint/naming-convention 
        }
    };

    return api.createType('PalletCreditcoinCollectCoinsTokenContract', toType());
};

export const GATEContract = (evmAddress: ExternalAddress, txHash: string): CollectCoinsContract => { // eslint-disable-line @typescript-eslint/naming-convention
    return { kind: 'GATE', evmAddress, txHash };
};

export const GCREContract = (evmAddress: ExternalAddress, txHash: string): CollectCoinsContract => { // eslint-disable-line @typescript-eslint/naming-convention
    return { kind: 'GCRE', evmAddress, txHash };
};

export const requestCollectCoinsV2 = async (
    api: ApiPromise,
    contract: CollectCoinsContract,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const formattedContract = createTokenContract(api, contract);

    const unsubscribe: () => void = await api.tx.creditcoin
        .requestCollectCoinsV2(formattedContract)
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
