import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { addAuthorityAsync } from '../extrinsics/add-authority';
import { Option, Null, u64 } from '@polkadot/types';

const AUTHORITY_PUBKEY = '0x0238bcdc4d9ab1ef09a2f18ea49e512aafabaab02d21a8c6ff7d2ecee1f2a34d';
export const AUTHORITY_SURI = 'version energy retire rely olympic figure shop stumble fence trust spider civil';
const AUTHORITY_ACCOUNTID = '5C7conswAmt3HJrSyhcehWo7qqwy4f2thW2P2VLz1x4yMW6e';

export const setupAuthority = async (api: ApiPromise, sudoSigner: KeyringPair) => {
    const u8aToHex = (bytes: Uint8Array): string => {
        return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
    };
    const rpcUri = u8aToHex(api.createType('String', 'http://localhost:8545').toU8a());
    // platform + chain id + '-rpc-uri' suffix, to match `Blockchain::rpc_key` in the rust code
    await api.rpc.offchain.localStorageSet('PERSISTENT', 'evm-31337-rpc-uri', rpcUri);
    if ((await api.query.difficulty.targetBlockTime<u64>()).toNumber() > 4000) {
        console.log('setting target block time to 4000');
        await api.tx.sudo.sudo(api.tx.difficulty.setTargetBlockTime(4000)).signAndSend(sudoSigner, { nonce: -1 });
    }
    const hasAuthKey = await api.rpc.author.hasKey(AUTHORITY_PUBKEY, 'gots');
    if (hasAuthKey.isFalse) {
        console.log('no auth key!');
        await api.rpc.author.insertKey('gots', AUTHORITY_SURI, AUTHORITY_PUBKEY);
    }
    const auth = await api.query.creditcoin.authorities<Option<Null>>(AUTHORITY_ACCOUNTID);
    if (auth.isNone) {
        console.log('adding authority');
        await addAuthorityAsync(api, AUTHORITY_ACCOUNTID, sudoSigner);
    }
    await api.tx.sudo
        .sudo(api.tx.balances.setBalance(AUTHORITY_ACCOUNTID, '10000000000000000000', '0'))
        .signAndSend(sudoSigner, { nonce: -1 });
    await api.tx.sudo
        .sudo(api.tx.balances.setBalance(sudoSigner.address, '10000000000000000000', '0'))
        .signAndSend(sudoSigner, { nonce: -1 });
};
