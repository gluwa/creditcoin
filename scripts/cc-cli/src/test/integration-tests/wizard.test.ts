import { newApi } from "../../api";
import { parseAmountInternal } from "../../utils/parsing";
import { getValidatorStatus } from "../../utils/validatorStatus";
import { fundAccounts } from "./helpers";
import execa from "execa";

describe("integration test: validator wizard setup", () => {
  test("new validator should appear as waiting after using the wizard setup", async () => {
    // Fund stash and controller
    const { stash, controller } = await fundAccounts(
      parseAmountInternal("10000")
    );
    // Run wizard setup with 1k ctc ang to pair with node Bob
    execa.commandSync(
      `creditcoin-cli wizard --amount 1000 --url ws://localhost:9945`,
      {
        env: {
          CC_STASH_SEED: stash.seed,
          CC_CONTROLLER_SEED: controller.seed,
        },
      }
    );

    const { api } = await newApi("ws://localhost:9944");
    const validatorStatus = await getValidatorStatus(stash.address, api);

    expect(validatorStatus.waiting).toBe(true);
  }, 100000);
});
