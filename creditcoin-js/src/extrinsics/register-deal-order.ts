import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { u8aConcat } from '@polkadot/util';
import { AddressId, DealOrderAdded, LoanTerms } from '../model';
import { createCreditcoinLoanTerms } from '../transforms';
import { TxCallback } from '../types';
import { handleTransaction, handleTransactionFailed } from './common';
import { KeyringPair } from '@polkadot/keyring/types';
import { AskOrderAdded, processAskOrderAdded } from './add-ask-order';
import { BidOrderAdded, processBidOrderAdded } from './add-bid-order';
import { OfferAdded, processOfferAdded } from './add-offer';
import { processDealOrderAdded } from './add-deal-order';
import { Guid } from 'js-guid';

export type DealOrderRegistered = {
    askOrder: AskOrderAdded;
    bidOrder: BidOrderAdded;
    offer: OfferAdded;
    dealOrder: DealOrderAdded;
};

export const signLoanParams = (
    api: ApiPromise,
    signer: KeyringPair,
    expBlock: number,
    askGuid: Guid,
    bidGuid: Guid,
    loanTerms: LoanTerms,
) => {
    const ccLoanTerms = createCreditcoinLoanTerms(api, loanTerms);
    const bytesParams = u8aConcat(
        api.createType('u32', expBlock).toU8a(),
        api.createType('String', askGuid.toString()).toU8a(),
        api.createType('String', bidGuid.toString()).toU8a(),
        ccLoanTerms.toU8a(),
    );

    return signer.sign(bytesParams);
};

export const registerDealOrder = async (
    api: ApiPromise,
    lenderAddressId: string,
    borrowerAddressId: string,
    loanTerms: LoanTerms,
    expBlock: number,
    askGuid: Guid,
    bidGuid: Guid,
    borrowerKey: Uint8Array,
    signedParams: Uint8Array,
    lender: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxCallback,
) => {
    const ccLoanTerms = createCreditcoinLoanTerms(api, loanTerms);
    const unsubscribe: () => void = await api.tx.creditcoin
        .registerDealOrder(
            lenderAddressId,
            borrowerAddressId,
            ccLoanTerms,
            expBlock,
            askGuid.toString(),
            bidGuid.toString(),
            { Sr25519: borrowerKey }, // eslint-disable-line  @typescript-eslint/naming-convention
            { Sr25519: signedParams }, // eslint-disable-line  @typescript-eslint/naming-convention
        )
        .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processRegisterDealOrder = (api: ApiPromise, result: SubmittableResult): DealOrderRegistered => {
    const askOrder = processAskOrderAdded(api, result);
    const bidOrder = processBidOrderAdded(api, result);
    const offer = processOfferAdded(api, result);
    const dealOrder = processDealOrderAdded(api, result);
    return { askOrder, bidOrder, offer, dealOrder };
};

export const registerDealOrderAsync = async (
    api: ApiPromise,
    lenderAddressId: AddressId,
    borrowerAddressId: AddressId,
    loanTerms: LoanTerms,
    expBlock: number,
    askGuid: Guid,
    bidGuid: Guid,
    borrowerKey: Uint8Array,
    signedParams: Uint8Array,
    lender: KeyringPair,
) =>
    new Promise<DealOrderRegistered>((resolve, reject) => {
        const onFail = (result: SubmittableResult) => reject(handleTransactionFailed(api, result));
        const onSuccess = (result: SubmittableResult) => resolve(processRegisterDealOrder(api, result));

        registerDealOrder(
            api,
            lenderAddressId,
            borrowerAddressId,
            loanTerms,
            expBlock,
            askGuid,
            bidGuid,
            borrowerKey,
            signedParams,
            lender,
            onSuccess,
            onFail,
        ).catch((reason) => reject(reason));
    });
