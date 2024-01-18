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

    // This line converts from a creditcoin-js type to one that the blockchain can interact with
    // The `set_collect_coins_contract` extrinsics expects a DeployedContract which is how we end up with PalletCreditcoinOcwTasksCollectCoinsDeployedContract
    // DeployedContract is defined in pallets/creditcoin/ocw/tasks/collect_coins.rs.
    // These types and their names can be found in creditcoin-js/lib/interfaces/registry.d.ts
    const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
        address: contractAddress,
        chain: blockchain,
    });

    await api.tx.sudo.sudo(api.tx.creditcoin.setCollectCoinsContract(contract)).signAndSend(sudoSigner, { nonce: -1 });
}
