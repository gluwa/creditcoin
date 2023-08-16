import { newApi } from "../../api";
import {
  initECDSAKeyringPairFromPK,
  initKeyringPair,
} from "../../utils/account";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import { fundAddressesFromSudo } from "./helpers";
import { mnemonicGenerate } from "@polkadot/util-crypto";
import execa from "execa";

describe("Send command", () => {
  it.each([
    ["using a seed phrase", false, mnemonicGenerate(12)],
    [
      "using an ecdsa pk",
      true,
      "0x657f8717980b9265d6a2114763468705588a7a8046a672b94ce8d427ee16a382",
    ],
  ])(
    "should be able to send CTC when %s",
    async (text, ecdsa, secret) => {
      const { api } = await newApi();
      const keyring = ecdsa
        ? initECDSAKeyringPairFromPK(secret)
        : initKeyringPair(secret);
      const address = keyring.address;

      const fundTx = await fundAddressesFromSudo(
        [address],
        parseAmountInternal("10000")
      );
      await signSendAndWatch(fundTx, api, initKeyringPair("//Alice"));

      const result = execa.commandSync(
        `creditcoin-cli send --to 5HDRB6edmWwwh6aCDKrRSbisV8iFHdP7jDy18U2mt9w2wEkq --amount 10 ${
          ecdsa ? "--ecdsa" : ""
        }`,
        {
          env: {
            CC_SECRET: secret,
          },
        }
      );

      expect(result.stdout).toContain("Transaction included");
    },
    60000
  );
});
