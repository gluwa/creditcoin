import { newApi } from "../../api";
import { initKeyringPair } from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import {
    fundAddressesFromSudo,
    fundFromSudo,
    randomTestAccount,
} from "./helpers";
import { arg } from "../../globalSetup";
import execa from "execa";
import {
    ApiPromise,
    Blockchain,
    CreditcoinApi,
    KeyringPair,
    Wallet,
    creditcoinApi,
    providers,
} from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { AUTHORITY_SURI } from "creditcoin-js/lib/examples/setup-authority";
import { testData, tryRegisterAddress } from "creditcoin-js/lib/testUtils";

describe("collect-coins", () => {
    let ccApi: CreditcoinApi;
    let authority: KeyringPair;
    let collector: KeyringPair;
    let ethWallet: Wallet;
    let caller: any;

    const { keyring, blockchain } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        expect(process.env.INTEGRATION_TEST).toBeTruthy();

        await cryptoWaitReady();

        ethWallet = Wallet.createRandom();
        caller = randomTestAccount(false);

        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            authority = keyring.createFromUri(AUTHORITY_SURI);
        }

        collector = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, "lender");

        const { api } = ccApi;

        /* eslint-disable @typescript-eslint/naming-convention */
        const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
            address: (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
            chain: blockchain,
        });

        await api.tx.sudo
            .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
            .signAndSend(collector, { nonce: -1 });
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    test("e2e", async () => {
        const { api } = ccApi;

        const provider = new providers.JsonRpcProvider(arg('CREDITCOIN_ETHEREUM_NODE_URL'));
        const deployerWallet = new Wallet(arg('CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY'), provider);

        const fundTx = await fundFromSudo(
            caller.address,
            parseAmountInternal("1000000"),
            arg("CREDITCOIN_API_URL"),
        );
        await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

        const registerResult = execa.commandSync(
            `npx creditcoin-cli register-address -u ${arg("CREDITCOIN_API_URL")}`,
            {
                env: {
                    BLOCKCHAIN: "Ethereum",
                    PRIVATE_KEY: arg('CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY'),
                    CC_SECRET: caller.secret,
                },
            },
        );

        const stdout = registerResult.stdout.split("\n");
        expect(registerResult.failed).toBe(false);
        expect(registerResult.exitCode).toBe(0);
        expect(registerResult.stderr).toBe("");
        expect(stdout[stdout.length - 1]).toBe("Address Registered Successfully!");

        const collectResult = execa.commandSync(
            `npx creditcoin-cli collect-coins -u ${arg('CREDITCOIN_API_URL')}`,
            {
                env: {
                    EXTERNAL_ADDR: deployerWallet.address,
                    BURN_TX_HASH: arg('CREDITCOIN_CTC_BURN_TX_HASH'),
                    CC_SECRET: caller.secret,
                }
            }
        )

        const collectOutput = collectResult.stdout.split("\n");
        expect(collectResult.failed).toBe(false);
        expect(collectResult.exitCode).toBe(0);
        expect(collectResult.stderr).toBe("");
        expect(collectOutput[collectOutput.length - 1]).toBe("Success");
    }, 900_000);
});
