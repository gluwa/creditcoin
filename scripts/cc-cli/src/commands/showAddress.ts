import { cryptoWaitReady } from "@polkadot/util-crypto";
import { Command, OptionValues } from "commander";
import {
  getCallerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { setInteractivity } from "../utils/interactive";

export function makeShowAddressCommand() {
  const cmd = new Command("show-address");
  cmd.description("Show account address");
  cmd.action(showAddressAction);
  return cmd;
}

async function showAddressAction(options: OptionValues) {
  const interactive = setInteractivity(options);
  await cryptoWaitReady();
  const callerSeed = await getCallerSeedFromEnvOrPrompt(interactive);
  const pair = initKeyringPair(callerSeed);
  const address = pair.address;

  console.log("Account address:", address);

  process.exit(0);
}
