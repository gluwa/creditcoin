import execa from "execa";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { randomTestAccount } from "./helpers";

describe("Show address command", () => {
  it.each([
    ["using a seed phrase", false],
    ["using an ecdsa pk", true],
  ])(
    "should return the correct address when %s",
    async (text, ecdsa) => {
      await cryptoWaitReady();

      const caller = randomTestAccount(ecdsa);

      const result = execa.commandSync(
        `node dist/index.js show-address ${ecdsa ? "--ecdsa" : ""}`,
        {
          env: {
            CC_SECRET: caller.secret,
          },
        },
      );

      expect(result.stdout.split("Account address: ")[1]).toEqual(
        caller.address.toString(),
      );
    },
    60000,
  );
});
