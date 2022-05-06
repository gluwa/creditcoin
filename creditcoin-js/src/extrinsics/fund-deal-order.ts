import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { DealOrderFunded, DealOrderId, TransferId, TransferProcessed } from '../model';
import { createDealOrder, createTransfer } from '../transforms';
import { TxCallback } from '../types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { KeyringPair } from '@polkadot/keyring/types';

export const fundDealOrder = async (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    transferId: TransferId,
    lender: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const ccDealOrderId = api.createType('PalletCreditcoinDealOrderId', dealOrderId);
    const unsubscribe: () => void = await api.tx.creditcoin
        .fundDealOrder(ccDealOrderId, transferId)
        .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const processDealOrderFunded = (
    api: ApiPromise,
    result: SubmittableResult,
): [DealOrderFunded, TransferProcessed] => {
    const deal = processEvents(
        api,
        result,
        'DealOrderFunded',
        'PalletCreditcoinDealOrder',
        createDealOrder,
    ) as DealOrderFunded;
    const transfer = processEvents(
        api,
        result,
        'TransferProcessed',
        'PalletCreditcoinTransfer',
        createTransfer,
    ) as TransferProcessed;
    return [deal, transfer];
};

export const fundDealOrderAsync = (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    transferId: TransferId,
    lender: KeyringPair,
) => {
    return new Promise<[DealOrderFunded, TransferProcessed]>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processDealOrderFunded(api, result));
        fundDealOrder(api, dealOrderId, transferId, lender, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
