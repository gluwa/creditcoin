import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { AccountId } from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { handleTransaction } from './common';
import { TxCallback, TxFailureCallback } from '../types';

export const removeAuthority = async (
    api: ApiPromise,
    authorityAccount: AccountId,
    sudoSigner: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.sudo
        .sudo(api.tx.creditcoin.removeAuthority(authorityAccount))
        .signAndSend(sudoSigner, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const removeAuthorityAsync = async (api: ApiPromise, authorityAccount: AccountId, sudoSigner: KeyringPair) => {
    return new Promise<void>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(); // eslint-disable-line @typescript-eslint/no-unused-vars
        removeAuthority(api, authorityAccount, sudoSigner, onSuccess, reject).catch(reject);
    });
};
