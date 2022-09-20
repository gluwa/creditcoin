import { Wallet } from 'ethers';
import { joinSignature } from '@ethersproject/bytes';
import { sha256AsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { AccountId } from './model';
import { ApiPromise } from '@polkadot/api';

export const signAccountId = (api: ApiPromise, signer: Wallet, accountId: AccountId) => {
    const accountIdBytes = api.createType('AccountId', accountId).toU8a();
    const accountIdHash = blake2AsU8a(sha256AsU8a(accountIdBytes));
    return joinSignature(signer._signingKey().signDigest(accountIdHash)); // eslint-disable-line no-underscore-dangle
};

export const utils = (api: ApiPromise) => {
    return {
        signAccountId: (signer: Wallet, accountId: AccountId) => signAccountId(api, signer, accountId),
    };
};
