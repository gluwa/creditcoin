import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { toMicrounits } from "../utils/balance";

export function makeUnbondCommand() {
  const cmd = new Command("unbond");
  cmd.description("Schedule a portion of the stash to be unlocked");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use for sending CTC"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to use for sending CTC"
  );
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.action(unbondAction);
  return cmd;
}

async function unbondAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAmount(options);

  // Build account
  const seed = getSeedFromOptions(options);
  const stash = initKeyringPair(seed);

  // Unbond transaction
  const tx = api.tx.staking.unbond(toMicrounits(options.amount).toString());

  const hash = await tx.signAndSend(stash);

  console.log("Unbond transaction hash: " + hash.toHex());
  process.exit(0);
}

function checkAmount(options: OptionValues) {
  if (!options.amount) {
    console.log("Must specify amount to send");
    process.exit(0);
  }
}
