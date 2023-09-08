import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundFromSudo, randomTestAccount } from "./helpers";
import {
  arg,
  deployCtcToken,
  retry,
  setArg,
  setArgIfUndef,
  setAuthorities,
} from "../../globalSetup";
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
import { testData, tryRegisterAddress } from "creditcoin-js/lib/testUtils";
import { describeIf } from "../../utils/tests";
import { getBalance } from "../../utils/balance";

describeIf(arg("CREDITCOIN_EXECUTE_SETUP_AUTHORITY"), "collect-coins", () => {
  let ccApi: CreditcoinApi;
  let sudo: KeyringPair;
  let caller: any;

  const { keyring, blockchain } = testData(
    arg("CREDITCOIN_ETHEREUM_CHAIN") as Blockchain,
    arg("CREDITCOIN_CREATE_WALLET"),
  );

  beforeAll(async () => {
    await deployCtcToken((global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS);
    setArg(
      "CREDITCOIN_CTC_CONTRACT_ADDRESS",
      process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS,
    );
    setArgIfUndef(
      "CREDITCOIN_CTC_BURN_TX_HASH",
      process.env.CREDITCOIN_CTC_BURN_TX_HASH,
    );

    await retry(5, async (resolve: any, reject: any) => {
      await setAuthorities().then(resolve).catch(reject);
    }).catch(() => {
      console.log("Could not setup testing authorities");
      process.exit(1);
    });

    await cryptoWaitReady();

    caller = randomTestAccount(false);

    ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
    sudo = arg("CREDITCOIN_CREATE_SIGNER")(keyring, "sudo");

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
      .signAndSend(sudo, { nonce: -1 });
  }, 100_000);

  afterAll(async () => {
    await ccApi.api.disconnect();
  });

  test("e2e", async () => {
    const {
      api,
      utils: { signAccountId },
    } = ccApi;

    const provider = new providers.JsonRpcProvider(
      arg("CREDITCOIN_ETHEREUM_NODE_URL"),
    );
    const deployerWallet = new Wallet(
      arg("CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY"),
      provider,
    );

    const fundTx = await fundFromSudo(
      caller.address,
      parseAmountInternal("10"),
      arg("CREDITCOIN_API_URL"),
    );
    await signSendAndWatch(fundTx, api, sudo);

    await tryRegisterAddress(
      ccApi,
      deployerWallet.address,
      blockchain,
      signAccountId(deployerWallet, caller.address),
      caller.keyring,
      (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
    );

    // Read balance after register address call but prior to collect coins
    const starting = await getBalance(caller.address, api);

    const collectResult = execa.commandSync(
      `npx creditcoin-cli collect-coins -u ${arg("CREDITCOIN_API_URL") as string
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
    expect(collectOutput[collectOutput.length - 1]).toBe("Success!");

    // read the balance after collect coins
    const ending = await getBalance(caller.address, api);
    expect(ending.total.sub(starting.total).toNumber()).toBe(1);
    // expect(ending.transferable.sub(starting.transferable).toNumber()).toBe(1);

    console.log(ending.transferable.toString())
    console.log(starting.transferable.toString())
  }, 900_000);
});
