import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { toMicrounits } from "../utils/balance";
import { signSendAndWatch } from "../utils/tx";

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
  const controller = initKeyringPair(controllerSeed);

  // Unbond transaction
  const tx = api.tx.staking.unbond(toMicrounits(options.amount).toString());

  const result = await signSendAndWatch(tx, api, stash);
  const hash = await tx.signAndSend(controller);

  console.log(result.info);
  process.exit(0);
}

function checkAmount(options: OptionValues) {
  if (!options.amount) {
    console.log("Must specify amount to send");
    process.exit(1);
  }
}
