import { cryptoWaitReady } from "@polkadot/util-crypto";
import { Command, OptionValues } from "commander";
import { getSeedFromEnvOrPrompt, initKeyringPair } from "../utils/account";

export function makeShowAddressCommand() {
  const cmd = new Command("show-address");
  cmd.description("Show account address");
  cmd.action(showAddressAction);
  return cmd;
}

async function showAddressAction(options: OptionValues) {
  await cryptoWaitReady();
  const seed = await getSeedFromEnvOrPrompt(process.env.CC_SEED, "Specify seed phrase");
  const pair = initKeyringPair(seed);
  const address = pair.address;

  console.log("Account address:", address);

  process.exit(0);
}
