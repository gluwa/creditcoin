// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { GenericEventData } from '@polkadot/types/';
import { PalletCreditcoinAskOrderId, PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';

import { handleTransaction, TxOnFail, TxOnSuccess } from '../utils';

export const addAskOrder = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    askGuid: string,
    signer: KeyringPair,
    onSuccess: TxOnSuccess,
    onFail: TxOnFail,
) => {
    const unsubscribe: () => void = await api.tx.creditcoin
        .addAskOrder(externalAddress, loanTerms, expirationBlock, askGuid)
        .signAndSend(signer, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

const processAskOrderAdded = (api: ApiPromise, result: SubmittableResult): PalletCreditcoinAskOrderId | undefined => {
    const { events } = result;
    const askOrderAdded = events.find(
        ({ event }) => event.section === 'creditcoin' && event.method === 'AskOrderAdded',
    );

    const getData = (data: GenericEventData) => {
        const askOrderId = data[0] as PalletCreditcoinAskOrderId;
        return askOrderId;
    };

    return askOrderAdded && getData(askOrderAdded.event.data);
};

export const addAskOrderAsync = async (
    api: ApiPromise,
    externalAddress: string,
    loanTerms: PalletCreditcoinLoanTerms,
    expirationBlock: number,
    askGuid: string,
    signer: KeyringPair,
) => {
    return new Promise<PalletCreditcoinAskOrderId | undefined>((resolve, reject) => {
        const onFail = () => resolve(undefined);
        const onSuccess = (result: SubmittableResult) => resolve(processAskOrderAdded(api, result));

        addAskOrder(api, externalAddress, loanTerms, expirationBlock, askGuid, signer, onSuccess, onFail).catch(
            (reason) => reject(reason),
        );
    });
};
