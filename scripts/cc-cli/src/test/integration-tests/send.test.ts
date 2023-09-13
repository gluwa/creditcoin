import { newApi } from "../../api";
import { initKeyringPair } from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundAddressesFromSudo, randomTestAccount } from "./helpers";
import execa from "execa";

describe("Send command", () => {
  it.each([
    ["using a seed phrase", false],
    ["using an ecdsa pk", true],
  ])(
    "should be able to send CTC when %s",
    async (text, ecdsa) => {
      const { api } = await newApi();

      const caller = randomTestAccount(ecdsa);

      const fundTx = await fundAddressesFromSudo(
        [caller.address],
        parseAmountInternal("10000"),
      );
      await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

      const result = execa.commandSync(
        `node dist/index.js send --to 5HDRB6edmWwwh6aCDKrRSbisV8iFHdP7jDy18U2mt9w2wEkq --amount 10 ${
          ecdsa ? "--ecdsa" : ""
        }`,
        {
          env: {
            CC_SECRET: caller.secret,
          },
        },
      );

      expect(result.stdout).toContain("Transaction included");
    },
    60000,
  );
});
