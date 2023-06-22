import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { chill } from "../utils/validate";
import { getStatus, requireStatus } from "../utils/status";

export function makeChillCommand() {
  const cmd = new Command("chill");
  cmd.description(
    "Signal intention to stop validating from a Controller account"
  );
  cmd.action(chillAction);
  return cmd;
}

async function chillAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controllerKeyring = initKeyringPair(controllerSeed);
  const controllerAddress = controllerKeyring.address;

  const controllerStatus = await getStatus(controllerAddress, api);

  if (!controllerStatus.stash) {
    console.error(`Cannot chill, ${controllerAddress} is not staked`);
    process.exit(1);
  }
  const stashStatus = await getStatus(controllerStatus.stash, api);

  requireStatus(stashStatus, "validating");

  console.log("Creating chill transaction...");

  const chillTxHash = await chill(controllerSeed, api);

  console.log("Chill transaction sent with hash:", chillTxHash.toHex());
  process.exit(0);
}
