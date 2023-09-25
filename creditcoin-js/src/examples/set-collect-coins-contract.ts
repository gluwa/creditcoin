import { KeyringPair } from '@polkadot/keyring/types';
import { Blockchain } from 'src/model';
import { CreditcoinApi } from 'src/types';

export async function setCollectCoinsContractExample(
    ccApi: CreditcoinApi,
    contractAddress: string,
    blockchain: Blockchain,
    sudoSigner: KeyringPair,
) {
    const { api } = ccApi;

    const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
        address: contractAddress,
        chain: blockchain,
    });

    await api.tx.sudo.sudo(api.tx.creditcoin.setCollectCoinsContract(contract)).signAndSend(sudoSigner, { nonce: -1 });
}
