import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { chill } from "../utils/validate";
import { getValidatorStatus, requireStatus } from "../utils/validatorStatus";
import { setInteractivity } from "../utils/interactive";

export function makeChillCommand() {
  const cmd = new Command("chill");
  cmd.description(
    "Signal intention to stop validating from a Controller account"
  );
  cmd.action(chillAction);
  return cmd;
}

async function chillAction(options: OptionValues) {
  const interactive = setInteractivity(options);
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt(interactive);
  const controllerKeyring = initKeyringPair(controllerSeed);
  const controllerAddress = controllerKeyring.address;

  const controllerStatus = await getValidatorStatus(controllerAddress, api);

  if (!controllerStatus.stash) {
    console.error(`Cannot chill, ${controllerAddress} is not staked`);
    process.exit(1);
  }
  const stashStatus = await getValidatorStatus(controllerStatus.stash, api);

  requireStatus(stashStatus, "validating");

  console.log("Creating chill transaction...");

  const result = await chill(controllerSeed, api);

  console.log(result.info);
  process.exit(0);
}
