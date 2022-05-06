import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { DealOrderId, TransferId } from '../model';
import { createDealOrder, createTransfer } from '../transforms';
import { TxCallback } from '../types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { KeyringPair } from '@polkadot/keyring/types';
import { DealOrderClosed, TransferProcessed } from '../model';

export const closeDealOrder = async (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    transferId: TransferId,
    borrower: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const ccDealOrderId = api.createType('PalletCreditcoinDealOrderId', dealOrderId);
    const unsubscribe: () => void = await api.tx.creditcoin
        .closeDealOrder(ccDealOrderId, transferId)
        .signAndSend(borrower, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const processDealOrderClosed = (
    api: ApiPromise,
    result: SubmittableResult,
): [DealOrderClosed, TransferProcessed] => {
    const closedDeal = processEvents(
        api,
        result,
        'DealOrderClosed',
        'PalletCreditcoinDealOrder',
        createDealOrder,
    ) as DealOrderClosed;
    const processedTransfer = processEvents(
        api,
        result,
        'TransferProcessed',
        'PalletCreditcoinTransfer',
        createTransfer,
    ) as TransferProcessed;
    return [closedDeal, processedTransfer];
};

export const closeDealOrderAsync = (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    transferId: TransferId,
    lender: KeyringPair,
) => {
    return new Promise<[DealOrderClosed, TransferProcessed]>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processDealOrderClosed(api, result));
        closeDealOrder(api, dealOrderId, transferId, lender, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
