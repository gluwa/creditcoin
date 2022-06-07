import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Option } from '@polkadot/types';
import {
    Collectedcoins,
    UnverifiedCollectedcoins,
    CollectedcoinsId,
    Blockchain,
    ExternalAddress,
    EventReturnJoinType,
} from '../model';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { TxCallback } from '..';
import { createCollectCoins, createUnverifiedCollectCoins } from '../transforms';
import { PalletCreditcoinCollectedCoins } from '@polkadot/types/lookup';

export type CollectCoinsEventKind = 'CollectCoinsRegistered' | 'CollectCoinsMinted' | 'CollectCoinsFailed';

export type CollectCoinsEvent = {
    collectCoinsId: CollectedcoinsId;
    collectCoins?: Collectedcoins;
    unverifiedCollectCoins?: UnverifiedCollectedcoins;

    waitForVerification: (timeout?: number) => Promise<Collectedcoins>;
};

export const createCollectCoinsId = (txHash: string) => {
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
    onFail: TxCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .requestCollectCoins(evmAddress, txHash)
        .signAndSend(collector, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

type CollectCoinsRegisteredEvent = EventReturnJoinType<CollectedcoinsId, UnverifiedCollectedcoins>;

const persistedCollectCoins = (api: ApiPromise, collectCoinsId: CollectedcoinsId, timeout = 20_000) => {
    return new Promise<Collectedcoins>((resolve, reject) => {
        let timer: NodeJS.Timeout | undefined;
        api.query.creditcoin
            .collectCoins(collectCoinsId, (result: Option<PalletCreditcoinCollectedCoins>) => {
                if (!timer) timer = setTimeout(() => reject(new Error('CollectCoins verification timed out')), timeout);
                if (result.isSome) {
                    clearTimeout(timer);
                    const object = createCollectCoins(result.unwrap());
                    resolve(object);
                }
            })
            .catch((reason) => reject(reason));
    });
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
        'PalletCreditcoinUnverifiedCollectCoins',
        createUnverifiedCollectCoins,
    ) as CollectCoinsRegisteredEvent;

    const collectCoinsEventData = {
        collectCoinsId: itemId,
        unverifiedCollectCoins: item,
        waitForVerification: (timeout = 180_000) => persistedCollectCoins(api, itemId, timeout),
    };

    return collectCoinsEventData;
};

export const requestCollectCoinsAsync = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
) => {
    return new Promise<CollectCoinsEvent>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) =>
            resolve(createCollectCoinsRegisteredEvent(api, result, 'CollectCoinsRegistered'));
        requestCollectCoins(api, evmAddress, collector, txHash, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
