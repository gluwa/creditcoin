import { Wallet } from 'ethers';
import { personalSignAccountId } from '../utils';
import { createAddressId, personalSignSignature } from '../extrinsics/register-address-v2';
import { Keyring, creditcoinApi } from '../index';

export async function registerAddressV2Example(
) {
    // Create a keyring with Alice
    const creditcoinAddress = new Keyring({
        type: 'sr25519',
    }).addFromUri('//Alice');

    const {
        api,
        extrinsics: { registerAddressV2 },
    } = await creditcoinApi('ws://127.0.0.1:9944');

    const ethSigner = Wallet.createRandom();
    const blockchain = "Ethereum";

    const accountId = creditcoinAddress.addressRaw;
    const externalAddress = ethSigner.address;

    const signature = await personalSignAccountId(api, ethSigner, accountId);
    const proof = personalSignSignature(signature);

    const lenderRegAddr = await registerAddressV2(externalAddress, blockchain, proof, creditcoinAddress);
    const addressId = createAddressId(blockchain, ethSigner.address);

    await api.disconnect()

    return { lenderRegAddr, addressId }
}
