import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { bond, parseRewardDestination } from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";
import { Balance, getBalance, toMicrounits } from "../utils/balance";

export function makeBondCommand() {
  const cmd = new Command("bond");
  cmd.description("Bond CTC from a Stash account");
  cmd.option("-a, --amount [amount]", "Amount to bond");
  cmd.option("-s, --seed [seed phrase]", "Specify seed phrase to bond from");
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with seed phrase to bond from"
  );
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
  if (!options.amount || !parseInt(options.amount, 10)) {
    console.log("Must specify amount to bond");
    process.exit(1);
  }

  const { api } = await newApi(options.url);

  const stashSeed = getSeedFromOptions(options);

  // Check balance
  const stashKeyring = initKeyringPair(stashSeed);
  const stashAddress = stashKeyring.address;
  const balance = await getBalance(stashAddress, api);
  const amount = parseInt(options.amount, 10);
  checkBalanceAgainstBondAmount(balance, amount);

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
    amount,
    rewardDestination,
    api
  );

  console.log("Bond transaction sent with hash:", bondTxHash);
  process.exit(0);
}

function checkBalanceAgainstBondAmount(balance: Balance, amount: number) {
  if (balance.free.lt(toMicrounits(amount))) {
    throw new Error(
      `Insufficient funds to bond ${toMicrounits(amount).toString()}`
    );
  }
}
