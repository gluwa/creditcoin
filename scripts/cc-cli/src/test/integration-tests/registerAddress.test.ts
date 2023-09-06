import { initKeyringPair } from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundFromSudo, randomTestAccount } from "./helpers";
import { arg } from "../../globalSetup";
import execa from "execa";
import { CreditcoinApi, Wallet, creditcoinApi } from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { describeIf } from "../../utils/tests";

describeIf(
    process.env.INTEGRATION_TEST && arg("CREDITCOIN_EXECUTE_SETUP_AUTHORITY"),
    "register-address",
    () => {
        let ccApi: CreditcoinApi;
        let ethWallet: Wallet;
        let caller: any;

        beforeAll(async () => {
            expect(process.env.INTEGRATION_TEST).toBeTruthy();

            await cryptoWaitReady();

            ethWallet = Wallet.createRandom();
            caller = randomTestAccount(false);

            ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
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
                `npx creditcoin-cli register-address -u ${arg("CREDITCOIN_API_URL") as string
                }`,
                {
                    env: {
                        // eslint-disable-next-line @typescript-eslint/naming-convention
                        BLOCKCHAIN: "Ethereum",
                        // eslint-disable-next-line @typescript-eslint/naming-convention
                        PRIVATE_KEY: ethWallet.privateKey,
                        CC_SECRET: caller.secret,
                    },
                },
            );

            const stdout = result.stdout.split("\n");

            expect(result.failed).toBe(false);
            expect(result.exitCode).toBe(0);
            expect(result.stderr).toBe("");
            expect(
                stdout[stdout.length - 1].includes("Address Registered Successfully"),
            ).toBe(true);
        }, 50_000);
    },
);
