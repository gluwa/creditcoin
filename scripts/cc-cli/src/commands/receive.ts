import { Command, OptionValues } from "commander";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";

export function makeReceiveCommand() {
  const cmd = new Command("receive");
  cmd.description("Show account address");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use of account"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase of account"
  );
  cmd.action(receiveAction);
  return cmd;
}

function receiveAction(options: OptionValues) {
  const seed = getSeedFromOptions(options);
  const pair = initKeyringPair(seed);
  const address = pair.address;

  console.log("Account address:", address);

  process.exit(0);
}
