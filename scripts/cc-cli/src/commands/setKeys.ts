import { Command, OptionValues } from "commander";
import { newApi } from "../api";
<<<<<<< HEAD
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";
=======
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
>>>>>>> 6dc67c37 (CSUB-589: changed sensitive input to interactive or env vars (#1151))

export function makeSetKeysCommand() {
  const cmd = new Command("set-keys");
  cmd.description("Set session keys for a Controller account");
  cmd.option("-k, --keys [keys]", "Specify keys to set");
  cmd.option("-r, --rotate", "Rotate and set new keys");

  cmd.action(setKeysAction);
  return cmd;
}

async function setKeysAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Build account
  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controller = initKeyringPair(controllerSeed);

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
<<<<<<< HEAD
  const result = await signSendAndWatch(tx, api, stash);
=======
  const hash = await tx.signAndSend(controller);
>>>>>>> 6dc67c37 (CSUB-589: changed sensitive input to interactive or env vars (#1151))

  console.log(result.info);

  process.exit(0);
}
