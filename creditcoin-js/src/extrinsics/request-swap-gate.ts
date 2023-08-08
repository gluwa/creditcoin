import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { EventReturnJoinType, ExternalAddress, SwapGATEId, SwappedGATE, UnverifiedSwapGATE } from '../model';
import { TxCallback, TxFailureCallback, VerificationError } from '../types';
import { handleTransaction, listenForVerificationOutcome, processEvents } from './common';
import { KeyringPair } from '@polkadot/keyring/types';
import { createBurnedGATE, createUnverifiedBurnGATE } from '../transforms';

export type SwapGATEEventKind = 'BurnGATERegistered' | 'BurnGATEFailedVerification' | 'BurnGATEMinted';

export type SwapGATEEvent = {
    swapGATEId: SwapGATEId;
    swappedGATE?: SwappedGATE;
    unverifiedSwapGATE?: UnverifiedSwapGATE;

    waitForVerification: (timeout?: number) => Promise<SwappedGATE>;
};

export const requestSwapGATE = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .requestBurnGate(evmAddress, txHash)
        .signAndSend(collector, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

type SwapGATERegisteredEvent = EventReturnJoinType<SwapGATEId, UnverifiedSwapGATE>;

const persistedSwapGATE = (api: ApiPromise, swapGateId: SwapGATEId, timeout = 20_000) => {
    return listenForVerificationOutcome(
        api,
        {
            successEvent: api.events.creditcoin.BurnedGATEMinted,
            failEvent: api.events.creditcoin.BurnGATEFailedVerification,
            processSuccessEvent: async ([id]) => {
                if (id.toString() == swapGateId) {
                    const result = await api.query.creditcoin.burnedGATE(swapGateId);
                    return createBurnedGATE(result.unwrap());
                }
            },
            processFailEvent: ([id, cause]) => {
                if (id.toString() == swapGateId) {
                    return new VerificationError(`Swap GATE ${swapGateId} failed: ${cause.toString()}`, cause);
                }
            },
        },
        timeout,
    );
};

const createBurnGATERegisteredEvent = (
    api: ApiPromise,
    result: SubmittableResult,
    kind: SwapGATEEventKind,
): SwapGATEEvent => {
    const { itemId, item } = processEvents(
        api,
        result,
        kind,
        'PalletCreditcoinCollectCoinsUnverifiedBurnGATE',
        createUnverifiedBurnGATE,
    ) as SwapGATERegisteredEvent;

    const swapGATEEventData = {
        swapGATEId: itemId,
        unverifiedSwapGATE: item,
        waitForVerification: (timeout = 180_000) => persistedSwapGATE(api, itemId, timeout),
    };

    return swapGATEEventData;
};

export const requestSwapGATEAsync = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
) => {
    return new Promise<SwapGATEEvent>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) =>
            resolve(createBurnGATERegisteredEvent(api, result, 'BurnGATERegistered'));
        requestSwapGATE(api, evmAddress, collector, txHash, onSuccess, reject).catch((reason) => {
            reject(reason);
        });
    });
};
