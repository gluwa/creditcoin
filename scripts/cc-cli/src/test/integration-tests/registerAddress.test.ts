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
} from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { AUTHORITY_SURI } from "creditcoin-js/lib/examples/setup-authority";
import { testData, tryRegisterAddress } from "creditcoin-js/lib/testUtils";

describe("register-address", () => {
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


    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    test("e2e", async () => {
        const { api } = ccApi;

        const fundTx = await fundFromSudo(
            caller.address,
            parseAmountInternal("1000000"),
            arg("CREDITCOIN_API_URL"),
        );
        await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

        const result = execa.commandSync(
            `npx creditcoin-cli register-address -u ${arg("CREDITCOIN_API_URL")}`,
            {
                env: {
                    BLOCKCHAIN: "Ethereum",
                    PRIVATE_KEY: ethWallet.privateKey,
                    CC_SECRET: caller.secret,
                },
            },
        );

        const stdout = result.stdout.split("\n");

        expect(result.failed).toBe(false);
        expect(result.exitCode).toBe(0);
        expect(result.stderr).toBe("");
        expect(stdout[stdout.length - 1]).toBe("Address Registered Successfully!");
    }, 50_000);
});
