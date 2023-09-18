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
} from "creditcoin-js";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { testData } from "creditcoin-js/lib/testUtils";
import { utils } from "ethers";

describe("register-address", () => {
  let ccApi: CreditcoinApi;
  let ethWallet: Wallet;
  let caller: any;
  let sudo: KeyringPair;

  beforeAll(async () => {
    await cryptoWaitReady();

    ethWallet = Wallet.createRandom();
    caller = randomTestAccount(false);

    ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);

    const { keyring } = testData(
      arg("CREDITCOIN_ETHEREUM_CHAIN") as Blockchain,
      arg("CREDITCOIN_CREATE_WALLET"),
    );

    sudo = arg("CREDITCOIN_CREATE_SIGNER")(keyring, "sudo");
  });

  afterAll(async () => {
    await ccApi.api.disconnect();
  });

  it.each([
    ['using ethereum private key', false],
    ['Using an ethereum mnemonic', true]
  ])("should be able to register address", async (text, useMnemonic) => {
    let ethPrivateKey: string;

    if (useMnemonic) {
      ethPrivateKey = utils.entropyToMnemonic(utils.randomBytes(32))
    } else {
      ethPrivateKey = ethWallet.privateKey;
    }

    const { api } = ccApi;

    const fundTx = await fundFromSudo(
      caller.address,
      parseAmountInternal("1000000"),
      arg("CREDITCOIN_API_URL"),
    );
    await signSendAndWatch(fundTx, api, sudo);

    const url = arg("CREDITCOIN_API_URL") as string;
    const result = execa.commandSync(
      `node dist/index.js register-address --url ${url} --blockchain Ethereum ${useMnemonic ? "--eth-mnemonic" : ""}`,
      {
        env: {
          // eslint-disable-next-line @typescript-eslint/naming-convention
          ETH_PRIVATE_KEY: ethPrivateKey,
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
});
