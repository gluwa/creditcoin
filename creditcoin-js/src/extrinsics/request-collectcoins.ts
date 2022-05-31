import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { Option } from '@polkadot/types';
import {
    Collectcoins,
    UnverifiedCollectcoins,
    CollectcoinsId,
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
import { PalletCreditcoinCollectCoins } from '@polkadot/types/lookup';

export type CollectcoinsEventKind = 'CollectCoinsRegistered' | 'CollectCoinsMinted' | 'CollectCoinsFailed';

export type CollectcoinsEvent = {
    collectcoinsId: CollectcoinsId;
    collectcoins?: Collectcoins;
    unverifiedCollectcoins?: UnverifiedCollectcoins;

    waitForVerification: (timeout?: number) => Promise<Collectcoins>;
};

export const createCollectcoinsId = (txHash: string) => {
    const blockchain: Blockchain = 'Ethereum';
    const blockchainBytes = Buffer.from(blockchain.toLowerCase());
    const key = u8aConcat(blockchainBytes, u8aToU8a(txHash));
    return blake2AsHex(key);
};

export { Collectcoins };

export const requestCollectcoins = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .requestCollectcoins(evmAddress, txHash)
        .signAndSend(collector, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

type CollectCoinsRegisteredEvent = EventReturnJoinType<CollectcoinsId, UnverifiedCollectcoins>;

const persistedCollectCoins = (api: ApiPromise, collectcoinsId: CollectcoinsId, timeout = 20_000) => {
    return new Promise<Collectcoins>((resolve, reject) => {
        let timer: NodeJS.Timeout | undefined;
        api.query.creditcoin
            .collectCoins(collectcoinsId, (result: Option<PalletCreditcoinCollectCoins>) => {
                if (!timer) timer = setTimeout(() => reject(new Error('Collectcoins verification timed out')), timeout);
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
    kind: CollectcoinsEventKind,
): CollectcoinsEvent => {
    const { itemId, item } = processEvents(
        api,
        result,
        kind,
        'PalletCreditcoinUnverifiedCollectCoins',
        createUnverifiedCollectCoins,
    ) as CollectCoinsRegisteredEvent;

    const collectcoinsEventData = {
        collectcoinsId: itemId,
        unverifiedCollectcoins: item,
        waitForVerification: (timeout = 180_000) => persistedCollectCoins(api, itemId, timeout),
    };

    return collectcoinsEventData;
};

export const requestCollectcoinsAsync = async (
    api: ApiPromise,
    evmAddress: ExternalAddress,
    collector: KeyringPair,
    txHash: string,
) => {
    return new Promise<CollectcoinsEvent>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) =>
            resolve(createCollectCoinsRegisteredEvent(api, result, 'CollectCoinsRegistered'));
        requestCollectcoins(api, evmAddress, collector, txHash, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
