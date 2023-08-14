import { cryptoWaitReady } from "@polkadot/util-crypto";
import { Command, OptionValues } from "commander";
import { initCallerKeyring } from "../utils/account";

export function makeShowAddressCommand() {
  const cmd = new Command("show-address");
  cmd.description("Show account address");
  cmd.option("--ecdsa", "Show address using ECDSA PK");
  cmd.action(showAddressAction);
  return cmd;
}

async function showAddressAction(options: OptionValues) {
  await cryptoWaitReady();
  const caller = await initCallerKeyring(options);
  const address = caller.address;
  console.log("Account address:", address);
  process.exit(0);
}
