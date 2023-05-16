import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions } from "../utils/account";
import { bond, parseRewardDestination } from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";

export function makeBondCommand() {
  const cmd = new Command("bond");
  cmd.description("Bond CTC from a Stash account");
  cmd.option("-a, --amount [amount]", "Amount to bond");
  cmd.option("-s, --seed [mnemonic]", "Specify mnemonic phrase to bond from");
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to bond from"
  );
  // cmd.option('-p, --password [password]', 'Specify password to use for new account')
  cmd.option("-c, --controller [controller]", "Specify controller address");
  cmd.option(
    "-r, --reward-destination [reward-destination]",
    "Specify reward destination account to use for new account"
  );
  cmd.action(bondAction);
  return cmd;
}

async function bondAction(options: OptionValues) {
  // If no controller error and exit
  if (!options.controller) {
    console.log("Must specify controller address");
    process.exit(1);
  }

  // If no amount error and exit
  if (!options.amount || parseInt(options.amount, 10) < 1) {
    console.log("Must specify amount to bond");
    process.exit(1);
  }

  const api = await newApi(options.url);

  const stashSeed = getSeedFromOptions(options);

  const rewardDestination = options.rewardDestination
    ? parseRewardDestination(options.rewardDestination)
    : "Staked";

  console.log("Creating bond transaction...");
  console.log("Controller address:", options.controller);
  console.log("Reward destination:", rewardDestination);
  console.log("Amount:", parseInt(options.amount, 10));

  await promptContinue();

  const bondTxHash = await bond(
    stashSeed,
    options.controller,
    parseInt(options.amount, 10),
    rewardDestination,
    api
  );

  console.log("Bond transaction sent with hash:", bondTxHash);
  process.exit(0);
}
