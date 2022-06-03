import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { DealOrderId } from '../model';
import { TxCallback } from '../types';
import { handleTransaction, handleTransactionFailed } from './common';
import { KeyringPair } from '@polkadot/keyring/types';

export type LoanExempted = { dealOrderId: DealOrderId };

export const exemptLoan = async (
    api: ApiPromise,
    dealOrderId: DealOrderId,
    lender: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .exempt(api.createType('PalletCreditcoinDealOrderId', dealOrderId))
        .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const processLoanExempted = ({ events }: SubmittableResult): LoanExempted => {
    const sourceEvent = events.find(({ event: { method } }) => method === 'LoanExempted');
    if (!sourceEvent) throw new Error('LoanExempted event not found');
    return { dealOrderId: sourceEvent.event.data.toJSON() as DealOrderId };
};

export const exemptLoanAsync = (api: ApiPromise, dealOrderId: DealOrderId, lender: KeyringPair) => {
    return new Promise<LoanExempted>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processLoanExempted(result));
        exemptLoan(api, dealOrderId, lender, onSuccess, onFail).catch((reason) => reject(reason));
    });
};
