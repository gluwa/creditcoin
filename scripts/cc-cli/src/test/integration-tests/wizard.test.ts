import { newApi } from "../../api";
import { parseAmountInternal } from "../../utils/parsing";
import { getValidatorStatus } from "../../utils/validatorStatus";
import { ALICE_NODE_URL, BOB_NODE_URL, fundAddressesFromSudo, randomTestAccount } from "./helpers";
import execa from "execa";
import { signSendAndWatch } from "../../utils/tx";
import { initKeyringPair } from "../../utils/account";

describe("integration test: validator wizard setup", () => {
  it.each([
    ["using an ecdsa pk", true],
    ["using a seed phrase", false],
  ])("new validator should appear as waiting after running %s", async (text, ecdsa) => {
    // Fund stash and controller
    const stash = randomTestAccount(ecdsa);
    const controller = randomTestAccount(ecdsa);

    const fundTx = await fundAddressesFromSudo([stash.address, controller.address], parseAmountInternal("10000"));
    const {api} = await newApi(ALICE_NODE_URL);
    await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

    // Run wizard setup with 1k ctc ang to pair with node Bob
    execa.commandSync(
      `creditcoin-cli wizard --amount 1000 --url ${BOB_NODE_URL} ${ecdsa ? "--ecdsa" : ""}`,
      {
        env: {
          CC_STASH_SECRET: stash.secret,
          CC_CONTROLLER_SECRET: controller.secret,
        },
      },
    );

    const validatorStatus = await getValidatorStatus(stash.address, api);

    expect(validatorStatus.waiting).toBe(true);
  }, 120000);
});
