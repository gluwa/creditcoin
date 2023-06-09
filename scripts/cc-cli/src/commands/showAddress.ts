import { Command, OptionValues } from "commander";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { cryptoWaitReady } from "@polkadot/util-crypto";

export function makeShowAddressCommand() {
  const cmd = new Command("show-address");
  cmd.description("Show account address");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use of account"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase of account"
  );
  cmd.action(showAddressAction);
  return cmd;
}

async function showAddressAction(options: OptionValues) {
  await cryptoWaitReady();
  const seed = getSeedFromOptions(options);
  const pair = initKeyringPair(seed);
  const address = pair.address;

  console.log("Account address:", address);

  process.exit(0);
}
