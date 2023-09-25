import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { ExternalAddress, CollectCoinsContract } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction } from './common';
import { TxCallback, TxFailureCallback } from '..';
import { PalletCreditcoinCollectCoinsBurnDetails } from '@polkadot/types/lookup';
import { CollectCoinsEvent, createCollectCoinsRegisteredEvent } from './request-collect-coins';

export const createTokenContract = (
    api: ApiPromise,
    contract: CollectCoinsContract,
): PalletCreditcoinCollectCoinsBurnDetails => {
    const toType = () => {
        switch (contract.kind) {
            case 'GCRE':
                // eslint-disable-next-line @typescript-eslint/naming-convention
                return { GCRE: [contract.evmAddress, contract.txHash] };
            case 'GATE':
                // eslint-disable-next-line @typescript-eslint/naming-convention
                return { GATE: [contract.evmAddress, contract.txHash] };
        }
    };

    return api.createType('PalletCreditcoinCollectCoinsBurnDetails', toType());
};

// eslint-disable-next-line @typescript-eslint/naming-convention
export const GATEContract = (evmAddress: ExternalAddress, txHash: string): CollectCoinsContract => {
    return { kind: 'GATE', evmAddress, txHash };
};

// eslint-disable-next-line @typescript-eslint/naming-convention
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
