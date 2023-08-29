import { newApi } from "../../api";
import { initKeyringPair } from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundAddressesFromSudo, randomTestAccount } from "./helpers";
import execa from "execa";
import { JsonRpcProvider } from "@ethersproject/providers";
import {
    AccountId,
    CreditcoinApi,
    Keyring,
    KeyringPair,
    TxCallback,
    TxFailureCallback,
    Wallet,
    creditcoinApi,
} from "creditcoin-js";
import { GluwaCreditVestingToken } from "./ethereum/ctc/typechain";
import { ContractFactory } from "ethers";
import CtcArtifact from "./ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json";
import { SubmittableResult } from "@polkadot/api"

describe("Request Collect Coins", () => {
    let provider: JsonRpcProvider;
    let deployer: Wallet;
    let token: GluwaCreditVestingToken;
    let ccApi: CreditcoinApi;
    let keyring: Keyring;
    let sudoSigner: KeyringPair;

    beforeAll(async () => {
        expect(process.env.ETH_NODE_URL).not.toBeUndefined();
        expect(process.env.CTC_API_URL).not.toBeUndefined();

        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);

        provider = new JsonRpcProvider(process.env.ETH_NODE_URL);
        deployer = new Wallet(
            "0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe",
            provider,
        );

        const factory = new ContractFactory(
            CtcArtifact.abi,
            CtcArtifact.bytecode,
            deployer,
        );
        const deployerAddress = await deployer.getAddress();

        token = (await factory.deploy(
            deployerAddress,
            deployerAddress,
        )) as GluwaCreditVestingToken;

        const keyring = new Keyring({ type: 'sr25519' });
        sudoSigner = keyring.addFromUri("//Alice");

        const { api } = ccApi;

        await setupAuthority(api, sudoSigner)

    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    test("e2e", async () => {

        console.log(token.address)
        console.log(deployer.address)
        const res = await token.burn(1);
        console.log(res.hash)
    });
});

import { ApiPromise } from '@polkadot/api';
import { Option, Null } from '@polkadot/types';
import { handleTransaction } from "creditcoin-js/lib/extrinsics/common";

const AUTHORITY_PUBKEY = '0x0238bcdc4d9ab1ef09a2f18ea49e512aafabaab02d21a8c6ff7d2ecee1f2a34d';
export const AUTHORITY_SURI = 'version energy retire rely olympic figure shop stumble fence trust spider civil';
const AUTHORITY_ACCOUNTID = '5C7conswAmt3HJrSyhcehWo7qqwy4f2thW2P2VLz1x4yMW6e';

export const addAuthority = async (
    api: ApiPromise,
    authorityAccount: AccountId,
    sudoSigner: KeyringPair,
    onSuccess: TxCallback,
    onFail: TxFailureCallback,
) => {
    const unsubscribe: () => void = await api.tx.sudo
        .sudo(api.tx.creditcoin.addAuthority(authorityAccount))
        .signAndSend(sudoSigner, { nonce: -1 }, (result) =>
            handleTransaction(api, unsubscribe, result, onSuccess, onFail),
        );
};

export const addAuthorityAsync = async (api: ApiPromise, authorityAccount: AccountId, sudoSigner: KeyringPair) => {
    return new Promise<void>((resolve, reject) => {
        const onSuccess = (result: SubmittableResult) => resolve(); // eslint-disable-line @typescript-eslint/no-unused-vars
        addAuthority(api, authorityAccount, sudoSigner, onSuccess, reject).catch(reject);
    });
};

export const setupAuthority = async (api: ApiPromise, sudoSigner: KeyringPair) => {
    const u8aToHex = (bytes: Uint8Array): string => {
        return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '0x');
    };
    const rpcUri = u8aToHex(api.createType('String', 'http://localhost:8545').toU8a());
    await api.rpc.offchain.localStorageSet('PERSISTENT', 'ethereum-rpc-uri', rpcUri);
    const hasAuthKey = await api.rpc.author.hasKey(AUTHORITY_PUBKEY, 'gots');
    if (hasAuthKey.isFalse) {
        console.log('no auth key!');
        await api.rpc.author.insertKey('gots', AUTHORITY_SURI, AUTHORITY_PUBKEY);
    }
    const auth = await api.query.taskScheduler.authorities<Option<Null>>(AUTHORITY_ACCOUNTID);
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