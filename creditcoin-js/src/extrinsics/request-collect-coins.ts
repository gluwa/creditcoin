import { ApiPromise, SubmittableResult } from '@polkadot/api';
import {
    CollectedCoins,
    UnverifiedCollectedCoins,
    CollectedCoinsId,
    ExternalAddress,
    EventReturnJoinType,
} from '../model';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, processEvents, listenForVerificationOutcome } from './common';
import { TxCallback, TxFailureCallback, VerificationError } from '..';
import { createCollectedCoins, createUnverifiedCollectedCoins } from '../transforms';

export type CollectCoinsEventKind = 'CollectCoinsRegistered' | 'CollectedCoinsMinted' | 'CollectCoinsFailed';

export type CollectCoinsEvent = {
    collectedCoinsId: CollectedCoinsId;
    collectedCoins?: CollectedCoins;
    unverifiedCollectedCoins?: UnverifiedCollectedCoins;

    waitForVerification: (timeout?: number) => Promise<CollectedCoins>;
};

export const createCollectedCoinsId = (txHash: string) => {
    const blockchainBytes = Buffer.from('ethereum');
    const key = u8aConcat(blockchainBytes, u8aToU8a(txHash));
    return blake2AsHex(key);
};

export const requestCollectCoins = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .requestCollectCoins(evmAddress, txHash)
        .signAndSend(collector, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

type CollectCoinsRegisteredEvent = EventReturnJoinType<CollectedCoinsId, UnverifiedCollectedCoins>;

const persistedCollectCoins = (api: ApiPromise, collectedCoinsId: CollectedCoinsId, timeout = 20_000) => {
    return listenForVerificationOutcome(
        api,
        {
            successEvent: api.events.creditcoin.CollectedCoinsMinted,
            failEvent: api.events.creditcoin.CollectCoinsFailedVerification,
            processSuccessEvent: async ([id]) => {
                if (id.toString() === collectedCoinsId) {
                    const result = await api.query.creditcoin.collectedCoins(collectedCoinsId);
                    return createCollectedCoins(result.unwrap());
                }
            },
            processFailEvent: ([id, cause]) => {
                if (id.toString() === collectedCoinsId) {
                    return new VerificationError(`CollectCoins ${collectedCoinsId} failed: ${cause.toString()}`, cause);
                }
            },
        },
        timeout,
    );
};

const createCollectCoinsRegisteredEvent = (
    api: ApiPromise,
    result: SubmittableResult,
    kind: CollectCoinsEventKind,
): CollectCoinsEvent => {
    const { itemId, item } = processEvents(
        api,
        result,
        kind,
        'PalletCreditcoinCollectCoinsUnverifiedCollectedCoins',
        createUnverifiedCollectedCoins,
    ) as CollectCoinsRegisteredEvent;

    const collectedCoinsEventData = {
        collectedCoinsId: itemId,
        unverifiedCollectCoins: item,
        waitForVerification: (timeout = 180_000) => persistedCollectCoins(api, itemId, timeout),
    };

    return collectedCoinsEventData;
};

export const requestCollectCoinsAsync = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
) => {
    return new Promise<CollectCoinsEvent>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) =>
            resolve(createCollectCoinsRegisteredEvent(api, result, 'CollectCoinsRegistered'));
        requestCollectCoins(api, evmAddress, collector, txHash, onSuccess, reject).catch((reason) => {
            reject(reason);
        });
    });
};
