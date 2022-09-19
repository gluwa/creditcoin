import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { BN } from '@polkadot/util';
import { Blockchain, DealOrderId, Transfer, TransferId, TransferKind, TransferProcessed } from '../model';
import { u8aConcat, u8aToU8a } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { createCreditcoinTransferKind, createTransfer } from '../transforms';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction, listenForVerificationOutcome, processEvents } from './common';
import { TxCallback, TxFailureCallback, VerificationError } from '..';

export type TransferEventKind = 'TransferRegistered' | 'TransferVerified' | 'TransferProcessed';
export type TransferEvent = {
    kind: TransferEventKind;
    transferId: TransferId;
    transfer?: Transfer;
    waitForVerification: (timeout?: number) => Promise<Transfer>;
};

export const createFundingTransferId = (blockchain: Blockchain, txHash: string) => {
    const blockchainBytes = Buffer.from(blockchain.toString().toLowerCase());
    const key = u8aConcat(blockchainBytes, u8aToU8a(txHash));
    return blake2AsHex(key);
};

export const registerFundingTransfer = async (
    api: ApiPromise,
    transferKind: TransferKind,
    dealOrderId: DealOrderId,
    txHash: string,
    lender: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const ccTransferKind = createCreditcoinTransferKind(api, transferKind);
    const ccDealOrderId = api.createType('PalletCreditcoinDealOrderId', dealOrderId);
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerFundingTransfer(ccTransferKind, ccDealOrderId, txHash)
        .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const registerRepaymentTransfer = async (
    api: ApiPromise,
    transferKind: TransferKind,
    repaymentAmount: BN,
    dealOrderId: DealOrderId,
    txHash: string,
    borrower: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerRepaymentTransfer(
            createCreditcoinTransferKind(api, transferKind),
            api.createType('U256', repaymentAmount),
            api.createType('PalletCreditcoinDealOrderId', dealOrderId),
            txHash,
        )
        .signAndSend(borrower, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const verifiedTransfer = async (api: ApiPromise, transferId: TransferId, timeout = 20_000) => {
    return listenForVerificationOutcome(api, {
        successEvent: api.events.creditcoin.TransferVerified,
        failEvent: api.events.creditcoin.TransferFailedVerification,
        processSuccessEvent: async ([id]) => {
            if (id.toString() === transferId) {
                const result = await api.query.creditcoin.transfers(transferId);
                return createTransfer(result.unwrap());
            }
        },
        processFailEvent: async ([id, cause]) => {
            if (id.toString() === transferId) {
                return new VerificationError(`RegisterTransfer ${transferId} failed: ${cause}`, cause);
            }
        },
    });
};

const processTransferEvent = (api: ApiPromise, result: SubmittableResult, kind: TransferEventKind): TransferEvent => {
    const { itemId, item } = processEvents(
        api,
        result,
        kind,
        'PalletCreditcoinTransfer',
        createTransfer,
    ) as TransferProcessed;

    const transferEventData = { kind, transferId: itemId, transfer: item };
    const waitForVerification = (timeout = 180_000) => verifiedTransfer(api, transferEventData.transferId, timeout);
    return { ...transferEventData, waitForVerification };
};

export const registerFundingTransferAsync = async (
    api: ApiPromise,
    transferKind: TransferKind,
    dealOrderId: DealOrderId,
    txHash: string,
    signer: KeyringPair,
) => {
    return new Promise<TransferEvent>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) =>
            resolve(processTransferEvent(api, result, 'TransferRegistered'));
        registerFundingTransfer(api, transferKind, dealOrderId, txHash, signer, onSuccess, reject).catch((reason) =>
            reject(reason),
        );
    });
};

export const registerRepaymentTransferAsync = async (
    api: ApiPromise,
    transferKind: TransferKind,
    repaymentAmount: BN,
    dealOrderId: DealOrderId,
    txHash: string,
    signer: KeyringPair,
) => {
    return new Promise<TransferEvent>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) =>
            resolve(processTransferEvent(api, result, 'TransferRegistered'));
        registerRepaymentTransfer(
            api,
            transferKind,
            repaymentAmount,
            dealOrderId,
            txHash,
            signer,
            onSuccess,
            reject,
        ).catch((reason) => reject(reason));
    });
};
