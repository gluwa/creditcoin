// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { GenericEventData } from '@polkadot/types/';
import { PalletCreditcoinBidOrderId, PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { handleTransaction, TxOnFail, TxOnSuccess } from '../utils';

export const addBidOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    bidGuid: string,
    signer: KeyringPair,
    onSuccess: TxOnSuccess,
    onFail: TxOnFail,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .addBidOrder(externalAddress, loanTerms, expirationBlock, bidGuid)
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processBidOrderAdded = (api: ApiPromise, result: SubmittableResult): PalletCreditcoinBidOrderId | undefined => {
    const { events } = result;
    const bidOrderAdded = events.find(
        ({ event }) => event.section === 'creditcoin' && event.method === 'BidOrderAdded',
    );

    const getData = (data: GenericEventData) => {
        const bidOrderId = data[0] as PalletCreditcoinBidOrderId;
        return bidOrderId;
    };

    return bidOrderAdded && getData(bidOrderAdded.event.data);
};

export const addBidOrderAsync = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    bidGuid: string,
    signer: KeyringPair,
) => {
    return new Promise<PalletCreditcoinBidOrderId | undefined>((resolve, reject) => {
        const onFail = () => resolve(undefined);
        const onSuccess = (result: SubmittableResult) => resolve(processBidOrderAdded(api, result));

        addBidOrder(api, externalAddress, loanTerms, expirationBlock, bidGuid, signer, onSuccess, onFail).catch(
            (reason) => reject(reason),
        );
    });
};
