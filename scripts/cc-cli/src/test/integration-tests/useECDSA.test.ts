import execa from "execa";
import { fundAddressesFromSudo } from "./helpers";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { initKeyringPair } from "../../utils/account";
import { newApi } from "../../api";

describe("Using an ECDSA PK", () => {
  it("show-address should return a valid Creditcoin address", () => {
    const pk =
      "0x657f8717980b9265d6a2114763468705588a7a8046a672b94ce8d427ee16a382";

    // Address checked using Subkey
    const expectedAddress = "5HDRB6edmWwwh6aCDKrRSbisV8iFHdP7jDy18U2mt9w2wEkq";

    const result = execa.commandSync(`creditcoin-cli show-address --ecdsa`, {
      env: {
        CC_PK: pk,
      },
    });

    expect(result.stdout.split("Account address: ")[1]).toContain(
      expectedAddress
    );
  });

  it("should be able to send CTC", async () => {
    const { api } = await newApi();

    const pk =
      "0x657f8717980b9265d6a2114763468705588a7a8046a672b94ce8d427ee16a382";

    const fundTx = await fundAddressesFromSudo(
      ["5HDRB6edmWwwh6aCDKrRSbisV8iFHdP7jDy18U2mt9w2wEkq"],
      parseAmountInternal("10000")
    );

    await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

    const result = execa.commandSync(
      `creditcoin-cli send --to 5HDRB6edmWwwh6aCDKrRSbisV8iFHdP7jDy18U2mt9w2wEkq --amount 10 --ecdsa`,
      {
        env: {
          CC_PK: pk,
        },
      }
    );

    expect(result.stdout).toContain("Transaction included");
  }, 60000);
});
