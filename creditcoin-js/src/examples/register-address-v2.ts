import { CreditcoinApi } from '../types';
import { Wallet } from 'ethers';
import { Blockchain } from '../model';
import { personalSignAccountId } from '../utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { personalSignSignature } from '../extrinsics/register-address-v2';

export async function registerAddressV2Example(
    ccApi: CreditcoinApi,
    ethSigner: Wallet,
    creditcoinAddress: KeyringPair,
    blockchain: Blockchain,
) {
    const {
        api,
        extrinsics: { registerAddressV2 },
    } = ccApi;

    const accountId = creditcoinAddress.addressRaw;
    const externalAddress = ethSigner.address;

    const signature = await personalSignAccountId(api, ethSigner, accountId);
    const proof = personalSignSignature(signature);

    return registerAddressV2(externalAddress, blockchain, proof, creditcoinAddress);
}
