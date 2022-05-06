import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { blake2AsHex } from '@polkadot/util-crypto';
import { DealOrderAdded, DealOrderId, OfferId } from '../model';
import { createDealOrder } from '../transforms';
import { TxCallback } from '../types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { KeyringPair } from '@polkadot/keyring/types';

export const createDealOrderId = (expirationBlock: number, offerId: OfferId): DealOrderId => [
    expirationBlock,
    blake2AsHex(offerId[1]),
];

export const addDealOrder = async (
    api: ApiPromise,
    offerId: OfferId,
    expirationBlock: number,
    borrower: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const ccOfferId = api.createType('PalletCreditcoinOfferId', offerId);
    const unsubscribe: () => void = await api.tx.creditcoin
        .addDealOrder(ccOfferId, expirationBlock)
        .signAndSend(borrower, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const processDealOrderAdded = (api: ApiPromise, result: SubmittableResult): DealOrderAdded => {
    return processEvents(api, result, 'DealOrderAdded', 'PalletCreditcoinDealOrder', createDealOrder) as DealOrderAdded;
};

export const addDealOrderAsync = (api: ApiPromise, offerId: OfferId, expirationBlock: number, signer: KeyringPair) => {
    return new Promise<DealOrderAdded>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processDealOrderAdded(api, result));
        addDealOrder(api, offerId, expirationBlock, signer, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
