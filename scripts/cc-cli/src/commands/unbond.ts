import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { toMicrounits } from "../utils/balance";
import { getStatus, requireStatus } from "../utils/status";

export function makeUnbondCommand() {
  const cmd = new Command("unbond");
  cmd.description("Schedule a portion of the stash to be unlocked");
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.action(unbondAction);
  return cmd;
}

async function unbondAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAmount(options);

  // Build account
  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controllerKeyring = initKeyringPair(controllerSeed);
  const controllerAddress = controllerKeyring.address;

  const controllerStatus = await getStatus(controllerAddress, api);
  const stashStatus = await getStatus(controllerStatus.stash, api);
  requireStatus(stashStatus, "bonded");

  // Unbond transaction
  const tx = api.tx.staking.unbond(toMicrounits(options.amount).toString());

  const hash = await tx.signAndSend(controllerKeyring);

  console.log("Unbond transaction hash: " + hash.toHex());
  process.exit(0);
}

function checkAmount(options: OptionValues) {
  if (!options.amount) {
    console.log("Must specify amount to send");
    process.exit(1);
  }
}
