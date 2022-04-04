// Copyright 2022 Gluwa, Inc. & contributors
// SPDX-License-Identifier: The Unlicense

import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { PalletCreditcoinLoanTerms } from '@polkadot/types/lookup';
import { u8aConcat } from '@polkadot/util';

import { handleTransaction, TxOnFail, TxOnSuccess } from '../utils';

export const signLoanParams = (
  api: ApiPromise,
  signer: KeyringPair,
  expBlock: number,
  askGuid: string,
  bidGuid: string,
  loanTerms: PalletCreditcoinLoanTerms
) => {
  const bytesParams = u8aConcat(
    api.createType('u32', expBlock).toU8a(),
    api.createType('String', askGuid).toU8a(),
    api.createType('String', bidGuid).toU8a(),
    loanTerms.toU8a()
  );

  return signer.sign(bytesParams);
};

export const registerDealOrder = async (
  api: ApiPromise,
  lenderAddressId: string,
  borrowerAddressId: string,
  loanTerms: PalletCreditcoinLoanTerms,
  expBlock: number,
  askGuid: string,
  bidGuid: string,
  borrowerKey: Uint8Array,
  signedParams: Uint8Array,
  lender: KeyringPair,
  onSuccess: TxOnSuccess,
  onFail: TxOnFail
) => {
  const unsubscribe: () => void = await api.tx.creditcoin
    .registerDealOrder(
      lenderAddressId,
      borrowerAddressId,
      loanTerms,
      expBlock,
      askGuid,
      bidGuid,
      { Sr25519: borrowerKey },
      { Sr25519: signedParams }
    )
    .signAndSend(lender, { nonce: -1 }, (result) => handleTransaction(api, unsubscribe, result, onSuccess, onFail));
};

export const registerDealOrderAsync = async (
  api: ApiPromise,
  lenderAddressId: string,
  borrowerAddressId: string,
  loanTerms: PalletCreditcoinLoanTerms,
  expBlock: number,
  askGuid: string,
  bidGuid: string,
  borrowerKey: Uint8Array,
  signedParams: Uint8Array,
  lender: KeyringPair
) =>
  new Promise<boolean>((resolve, reject) => {
    const onFail = () => resolve(false);
    /* eslint-disable @typescript-eslint/no-unused-vars */
    const onSuccess = (result: SubmittableResult) => resolve(true);

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
      onFail
    ).catch((reason) => reject(reason));
  });
