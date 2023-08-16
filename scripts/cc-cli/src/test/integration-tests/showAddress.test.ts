import execa from "execa";
import { fundAddressesFromSudo } from "./helpers";
import { parseAmountInternal } from "../../utils/parsing";
import { signSendAndWatch } from "../../utils/tx";
import {
  initECDSAKeyringPairFromPK,
  initKeyringPair,
} from "../../utils/account";
import { newApi } from "../../api";
import { mnemonicGenerate } from "@polkadot/util-crypto";

describe("Show address command", () => {
  it.each([
    ["using a seed phrase", false, mnemonicGenerate(12)],
    [
      "using an ecdsa pk",
      true,
      "0x657f8717980b9265d6a2114763468705588a7a8046a672b94ce8d427ee16a382",
    ],
  ])(
    "should return the correct address when %s",
    async (text, ecdsa, secret) => {
      const { api } = await newApi();

      const keyring = ecdsa
        ? initECDSAKeyringPairFromPK(secret)
        : initKeyringPair(secret);
      const address = keyring.address;

      const result = execa.commandSync(
        `creditcoin-cli show-address ${ecdsa ? "--ecdsa" : ""}`,
        {
          env: {
            CC_SECRET: secret,
          },
        }
      );

      expect(result.stdout.split("Account address: ")[1]).toEqual(
        address.toString()
      );
    },
    60000
  );
});
