import { initKeyringPair } from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundFromSudo, randomTestAccount } from "./helpers";
import { arg } from "../../globalSetup";
import execa from "execa";
import {
  Blockchain,
  CreditcoinApi,
  KeyringPair,
  Wallet,
  creditcoinApi,
  providers,
} from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { testData } from "creditcoin-js/lib/testUtils";
import { describeIf } from "../../utils/tests";

describeIf(
  process.env.INTEGRATION_TEST && arg("CREDITCOIN_EXECUTE_SETUP_AUTHORITY"),
  "collect-coins",
  () => {
    let ccApi: CreditcoinApi;
    let collector: KeyringPair;
    let caller: any;

    const { keyring, blockchain } = testData(
      arg("CREDITCOIN_ETHEREUM_CHAIN") as Blockchain,
      arg("CREDITCOIN_CREATE_WALLET"),
    );

    beforeAll(async () => {
      await cryptoWaitReady();

      caller = randomTestAccount(false);

      ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
      collector = arg("CREDITCOIN_CREATE_SIGNER")(keyring, "lender");

      const { api } = ccApi;

      /* eslint-disable @typescript-eslint/naming-convention */
      const contract = api.createType(
        "PalletCreditcoinOcwTasksCollectCoinsDeployedContract",
        {
          address: (global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS,
          chain: blockchain,
        },
      );

      await api.tx.sudo
        .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
        .signAndSend(collector, { nonce: -1 });
    });

    afterAll(async () => {
      await ccApi.api.disconnect();
    });

    test("e2e", async () => {
      const { api } = ccApi;

      const provider = new providers.JsonRpcProvider(
        arg("CREDITCOIN_ETHEREUM_NODE_URL"),
      );
      const deployerWallet = new Wallet(
        arg("CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY"),
        provider,
      );

      const fundTx = await fundFromSudo(
        caller.address,
        parseAmountInternal("1000000"),
        arg("CREDITCOIN_API_URL"),
      );
      await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

      const registerResult = execa.commandSync(
        `npx creditcoin-cli register-address -u ${
          arg("CREDITCOIN_API_URL") as string
        }`,
        {
          env: {
            BLOCKCHAIN: "Ethereum",
            PRIVATE_KEY: arg("CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY"),
            CC_SECRET: caller.secret,
          },
        },
      );

      const stdout = registerResult.stdout.split("\n");
      expect(registerResult.failed).toBe(false);
      expect(registerResult.exitCode).toBe(0);
      expect(registerResult.stderr).toBe("");
      expect(
        stdout[stdout.length - 1].includes("Address Registered Successfully"),
      ).toBe(true);

      const collectResult = execa.commandSync(
        `npx creditcoin-cli collect-coins -u ${
          arg("CREDITCOIN_API_URL") as string
        }`,
        {
          env: {
            EXTERNAL_ADDR: deployerWallet.address,
            BURN_TX_HASH: arg("CREDITCOIN_CTC_BURN_TX_HASH"),
            CC_SECRET: caller.secret,
          },
        },
      );

      const collectOutput = collectResult.stdout.split("\n");
      expect(collectResult.failed).toBe(false);
      expect(collectResult.exitCode).toBe(0);
      expect(collectResult.stderr).toBe("");
      expect(collectOutput[collectOutput.length - 1]).toBe("Success");
    }, 900_000);
  },
);
