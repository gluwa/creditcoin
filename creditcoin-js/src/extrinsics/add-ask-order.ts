import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { AddressId, AskOrder, AskOrderId, LoanTerms, EventReturnJoinType } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, handleTransactionFailed, processEvents } from './common';
import { TxCallback } from '../types';
import { createAskOrder, createCreditcoinLoanTerms } from '../transforms';
import { Guid } from 'js-guid';
import { blake2AsHex } from '@polkadot/util-crypto';

export type AskOrderAdded = EventReturnJoinType<AskOrderId, AskOrder>;

export const createAskOrderId = (expirationBlock: number, guid: Guid): AskOrderId =>
    [expirationBlock, blake2AsHex(guid.toString())] as AskOrderId;

export const addAskOrder = async (
    api: ApiPromise,
    lenderAddressId: AddressId,
    loanTerms: LoanTerms,
    expirationBlock: number,
    guid: Guid,
    signer: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .addAskOrder(lenderAddressId, createCreditcoinLoanTerms(api, loanTerms), expirationBlock, guid.toString())
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const processAskOrderAdded = (api: ApiPromise, result: SubmittableResult): AskOrderAdded => {
    return processEvents(api, result, 'AskOrderAdded', 'PalletCreditcoinAskOrder', createAskOrder) as AskOrderAdded;
};

export const addAskOrderAsync = async (
    api: ApiPromise,
    lenderAddressId: AddressId,
    loanTerms: LoanTerms,
    expirationBlock: number,
    guid: Guid,
    signer: KeyringPair,
) => {
    return new Promise<AskOrderAdded>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processAskOrderAdded(api, result));
        addAskOrder(api, lenderAddressId, loanTerms, expirationBlock, guid, signer, onSuccess, onFail).catch((reason) =>
            reject(reason),
        );
    });
};
