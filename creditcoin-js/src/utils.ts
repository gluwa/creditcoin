import { Wallet } from 'ethers';
import { joinSignature } from '@ethersproject/bytes';
import { sha256AsU8a, blake2AsU8a } from '@polkadot/util-crypto';
import { AccountId } from './model';
import { ApiPromise } from '@polkadot/api';
import { BN } from '@polkadot/util';
import { SiLookupTypeId } from '@polkadot/types/interfaces/scaleInfo';

export const signAccountId = (api: ApiPromise, signer: Wallet, accountId: AccountId) => {
    const accountIdBytes = api.createType('AccountId', accountId).toU8a();
    const accountIdHash = blake2AsU8a(sha256AsU8a(accountIdBytes));
    return joinSignature(signer._signingKey().signDigest(accountIdHash)); // eslint-disable-line no-underscore-dangle
};

export const personalSignAccountId = async (api: ApiPromise, signer: Wallet, accountId: Uint8Array) => {
    return joinSignature(await signer.signMessage(blake2AsU8a(accountId)));
};

type OldWeight = BN;
type NewWeight = { refTime: BN; proofSize: BN };
type Weight = OldWeight | NewWeight;

export const createOverrideWeight = (api: ApiPromise): Weight => {
    const sudoCallTypeId = api.runtimeMetadata.registry.metadata.pallets
        .find((v) => v.name.toString() === 'Sudo')
        ?.calls.unwrap().type as SiLookupTypeId;
    const sudoCallTypeDef = api.runtimeMetadata.registry.lookup.getTypeDef(sudoCallTypeId);
    if (Array.isArray(sudoCallTypeDef.sub)) {
        const sudoUncheckedWeightType = sudoCallTypeDef.sub.find((def) => def.name === 'sudo_unchecked_weight');
        if (sudoUncheckedWeightType && Array.isArray(sudoUncheckedWeightType.sub)) {
            // the weight is the second argument to sudo_unchecked_weight
            const weightType = sudoUncheckedWeightType.sub[1];
            if (weightType.type === 'u64') {
                // old weight (simple u64)
                return new BN(1);
            } else {
                // new weight
                return {
                    refTime: new BN(1),
                    proofSize: new BN(0),
                };
            }
        }
    }

    throw new Error("Couldn't find expected Weight type from sudoUncheckedWeight metadata");
};

export const utils = (api: ApiPromise) => {
    return {
        signAccountId: (signer: Wallet, accountId: AccountId) => signAccountId(api, signer, accountId),
        personalSignAccountId: (signer: Wallet, accountId: Uint8Array) => personalSignAccountId(api, signer, accountId),
        createOverrideWeight: () => createOverrideWeight(api),
    };
};


// block time in seconds
export const BLOCK_TIME = 15;

// We currently guarantee that a given collect coins will be dealt with in 60 blocks
// After that the task expires and you'd have to re-submit the transaction.
export const OCW_BLOCK_DEADLINE = 60;

