import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";

export function makeSetKeysCommand() {
  const cmd = new Command("set-keys");
  cmd.description("Set session keys for a Controller account");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to set keys from"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to set keys from"
  );
  cmd.option("-k, --keys [keys]", "Specify keys to set");
  cmd.option("-r, --rotate", "Rotate and set new keys");

  cmd.action(setKeysAction);
  return cmd;
}

async function setKeysAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Build account
  const seed = getSeedFromOptions(options);
  const stash = initKeyringPair(seed);

  let keys;
  if (!options.keys && !options.rotate) {
    console.log(
      "Must specify keys to set or generate new ones using the --rotate flag"
    );
    process.exit(1);
  } else if (options.rotate) {
    keys = (await api.rpc.author.rotateKeys()).toString();
  } else {
    keys = options.keys;
  }

  const tx = api.tx.session.setKeys(keys, []);
  const result = await signSendAndWatch(tx, api, stash);

  console.log(result.info);

  process.exit(0);
}
